#! /bin/bash
# Copyright 2016 The Tectonic Project
# Licensed under the MIT License.
#
# Run the tests. We should use a nice test rig and so on, but for now let's
# just get things on the air.

set -e
cd $(dirname $0)

trip/run.sh
etrip/run.sh
echo "All tests passed."
