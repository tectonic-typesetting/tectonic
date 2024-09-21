# Testing Bundles
These are a work in progress, and may be broken.
All tests are run through `test.sh` as follows: `./test.sh <path-to-ttb> <test set>`.

Tests require the following:
 - a `ttb` bundle (local or remote)
 - a recent installation of Tectonic

## Test Sets
The following test sets are avaiable:
 - `files`, which tries to compile all files under `tests/files` and `tests/formats`
 - `classes`, which tries to compile a simple document using `tests/classes.list`

Note that most test files contain comments explaining the reason and expected outcome of the test.



## Test Output
All test output ends up under `tests/build`

**Output for `files`:**
 - `files/logs`: log files for all builds (passed or failed)
 - `files/*.{pdf,fmt,etc}`: output files for each build


**Output for `classes`**
 - `failed`: classes that failed to compile
 - `passed`: classes that complied without error
 - `logs`: log files for all compile jobs