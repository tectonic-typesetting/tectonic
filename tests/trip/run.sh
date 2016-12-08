#! /bin/bash
# Copyright 2016 The Tectonic Project
# Licensed under the MIT License.

cd $(dirname $0)

# Throw away stderr since it has a bunch of diagnostic output from our I/O backend.
../../target/debug/tectonic-compat -fmt=trip -no-pdf -output-comment=trip trip >trip.fot 2>/dev/null
anybad=false

for f in trip.log trip.fot trip.xdv tripos.tex ; do
    if ! cmp $f reference-$f ; then
	echo >&2 "trip failed: file $f differs"
	anybad=true
    fi
done

$anybad && exit 1

rm -f 8terminal.tex trip.fot trip.log trip.xdv tripos.tex
