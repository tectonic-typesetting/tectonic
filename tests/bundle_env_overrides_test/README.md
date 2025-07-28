# Bundle URL overrides Test

This is a simple test program used by the integration tests in the `tectonic_bundles` crate to test the `get_fallback_bundle_url` function with different compile-time environment variable overrides.

## Purpose

The `get_fallback_bundle_url` function reads compile-time environment variables
- `$TECTONIC_BUNDLE_LOCKED`, and
- `$TECTONIC_BUNDLE_PREFIX`

via Rust's `option_env!` macro. Since these values are baked in at compile time, the only way to properly test different override scenarios is to compile separate instances of a test program with different environment variables set.

## Usage

This test program is invoked by the integration tests with different environment variable overrides:

```bash
# Test default behavior with format version 32
cargo run -- 32

# Test with locked bundle
TECTONIC_BUNDLE_LOCKED="https://example.com/bundle.tar" cargo run -- 32

# Test with custom prefix
TECTONIC_BUNDLE_PREFIX="https://custom.mirror.com" cargo run -- 31
```

## Arguments

- First argument: Format version number (required)

## Output

The program outputs the URL returned by `get_fallback_bundle_url()` to stdout.

## Environment Variables Tested

- `TECTONIC_BUNDLE_LOCKED`: When set to a non-empty value, this URL is returned regardless of format version
- `TECTONIC_BUNDLE_PREFIX`: hard codes a custom prefix for bundle URLs

## Integration with Tests

This test program is used by [`../bundle_env_overrides.rs`](../bundle_env_overrides.rs) to verify bundle override behavior:
1. Locked bundle behavior (same URL for all format versions)
2. Custom prefix behavior
3. Format version transitions (v < 32 vs v >= 32)
4. Environment variable precedence (locked takes priority over prefix)
