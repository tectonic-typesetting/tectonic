#!/usr/bin/env bash
#
# Reproducible cold-cache benchmark for the concurrent bundle prefetch change
# (PR #1379).
#
# It runs the SAME tectonic binary twice over the SAME cache state and toggles
# only one knob, TECTONIC_PREFETCH_CONCURRENCY:
#
#     serial      TECTONIC_PREFETCH_CONCURRENCY=1    (one range request at a time;
#                                                      proxy for the old behavior)
#     concurrent  TECTONIC_PREFETCH_CONCURRENCY=16   (this change's default)
#
# Both arms start from an identical "warm bundle index + warm prefetch manifest,
# cold file blobs" state. That is exactly what a repeated cold build hits: you've
# compiled this document before (so the working set is known), but the cached
# file blobs are gone (fresh machine, cleared cache, ephemeral CI runner, etc.).
# Because only the concurrency knob differs between the two arms, the measured
# delta is attributable to parallel fetching alone, not to a hand-built scenario.
#
# Usage:
#     dev/bench-cold-prefetch.sh
#
# Environment:
#     TECTONIC_BIN              path to a prebuilt tectonic (skips cargo build)
#     TECTONIC_CARGO_FEATURES   extra cargo features for the build (optional)
#     BENCH_REPEATS             repeats per arm; reports the min (default: 1)
#
# Notes:
#   * This talks to the real default bundle over the network, so absolute timings
#     depend on your latency/bandwidth to the bundle host. The *ratio* is the
#     point and is far more stable than the absolute numbers.
#   * Run it on an otherwise-idle network for the cleanest comparison.

set -euo pipefail

REPO="$(git rev-parse --show-toplevel)"

TEC="${TECTONIC_BIN:-}"
if [[ -z "$TEC" ]]; then
  echo "building release tectonic (set TECTONIC_BIN to skip)..." >&2
  if [[ -n "${TECTONIC_CARGO_FEATURES:-}" ]]; then
    ( cd "$REPO" && cargo build --release --features "$TECTONIC_CARGO_FEATURES" ) >&2
  else
    ( cd "$REPO" && cargo build --release ) >&2
  fi
  TEC="$REPO/target/release/tectonic"
fi

if [[ ! -x "$TEC" ]]; then
  echo "error: tectonic binary not found/executable at: $TEC" >&2
  exit 1
fi

REPEATS="${BENCH_REPEATS:-1}"

BENCH_DIR="$(mktemp -d)"
CACHE="$BENCH_DIR/cache"
WORK="$BENCH_DIR/work"
mkdir -p "$CACHE" "$WORK"
trap 'rm -rf "$BENCH_DIR"' EXIT

cat > "$WORK/bench.tex" <<'TEX'
\documentclass{article}
\title{Cold-compile benchmark}
\author{tectonic}
\begin{document}
\maketitle
\section{Introduction}
A small document whose default-LaTeX build pulls a few hundred small support
files over the network on a cold cache.
\[ \Delta\mathcal{L} = \sum_{i=1}^{n} \log p(x_i) - \log q(x_i). \]
\end{document}
TEX

# Run one compile; echo the wall-clock seconds. $1 = label, $2 = concurrency.
run() {
  local label="$1" conc="$2"
  ( cd "$WORK" && rm -f bench.pdf
    /usr/bin/time -p env \
      XDG_CACHE_HOME="$CACHE" \
      TECTONIC_CACHE_DIR="$CACHE" \
      TECTONIC_PREFETCH_CONCURRENCY="$conc" \
      "$TEC" -X compile bench.tex ) >"$BENCH_DIR/$label.out" 2>"$BENCH_DIR/$label.log"
  if [[ ! -f "$WORK/bench.pdf" ]]; then
    echo "error: compile failed (see $BENCH_DIR/$label.log)" >&2
    exit 1
  fi
  grep '^real' "$BENCH_DIR/$label.log" | awk '{print $2}'
}

# Delete only the cached file blobs (data/<hash>/), keeping the sibling
# data/<hash>.index and data/<hash>.prefetch files intact. DATA_DIR is set after
# the warm-up, once the cache layout exists on disk.
DATA_DIR=""
cold_files() {
  find "$DATA_DIR" -mindepth 1 -maxdepth 1 -type d -exec rm -rf {} +
}

# min of stdin numbers
min() { awk 'NR==1||$1<m{m=$1} END{print m}'; }

echo "binary : $TEC"
"$TEC" --version | head -1
echo "repeats: $REPEATS"

echo
echo "== warm-up: build bundle index + prefetch manifest =="
run warmup 16 >/dev/null
MANIFEST="$(find "$CACHE" -name '*.prefetch' | head -1 || true)"
if [[ -z "$MANIFEST" ]]; then
  echo "error: no prefetch manifest was produced; is this the patched binary?" >&2
  exit 1
fi
DATA_DIR="$(dirname "$MANIFEST")"
echo "manifest: $(wc -l < "$MANIFEST" | tr -d ' ') files"

echo
echo "== serial arm (TECTONIC_PREFETCH_CONCURRENCY=1) =="
serial_times=""
for i in $(seq 1 "$REPEATS"); do
  cold_files
  t="$(run "serial$i" 1)"
  echo "  run $i: ${t}s"
  serial_times+="$t"$'\n'
done
S="$(printf '%s' "$serial_times" | min)"

echo
echo "== concurrent arm (TECTONIC_PREFETCH_CONCURRENCY=16) =="
conc_times=""
for i in $(seq 1 "$REPEATS"); do
  cold_files
  t="$(run "conc$i" 16)"
  echo "  run $i: ${t}s"
  conc_times+="$t"$'\n'
done
C="$(printf '%s' "$conc_times" | min)"

echo
echo "== summary (best of $REPEATS) =="
awk -v s="$S" -v c="$C" 'BEGIN{
  printf "serial     = %.2fs\n", s
  printf "concurrent = %.2fs\n", c
  printf "speedup    = %.1fx\n", s/c
}'
