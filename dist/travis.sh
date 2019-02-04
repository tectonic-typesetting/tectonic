# Copyright 2019 the Tectonic Project
# Licensed under the MIT License.
#
# This script handles most of the Travis processing. Travis lets you break
# your build into a bunch of steps in their YAML file, but that don't provide
# much value for us since we barely use their built-in features. So we just
# have a big script that does almost everything.
#
# For each CI'd commit, this script will be run numerous times -- once for
# each item in the build matrix defined in .travis.yml.
#
# This script might be run even *more* times when one commit corresponds to
# more than one "event": a pull request; a push to a branch; explicit trigger
# via the API; or timed trigger via cron.
#
# Finally, we might have multiple forks of the repo, each seeing the same
# commits but CI'ing them in various ways.
#
# So, it's complicated.

set -e

# Helpful context.

echo "TRAVIS_BRANCH: $TRAVIS_BRANCH"
echo "TRAVIS_BUILD_ID: $TRAVIS_BUILD_ID"
echo "TRAVIS_COMMIT: $TRAVIS_COMMIT"
echo "TRAVIS_EVENT_TYPE: $TRAVIS_EVENT_TYPE" # one of: push, pull_request, api, cron
echo "TRAVIS_OS_NAME: $TRAVIS_OS_NAME"
echo "TRAVIS_OSX_IMAGE: $TRAVIS_OSX_IMAGE"
echo "TRAVIS_REPO_SLUG: $TRAVIS_REPO_SLUG"
echo "TRAVIS_SECURE_ENV_VARS: $TRAVIS_SECURE_ENV_VARS"
echo "TRAVIS_TAG: $TRAVIS_TAG"

echo "IMAGE: $IMAGE"

if [[ "$TRAVIS_OS_NAME" == linux && "$TRAVIS_RUST_VERSION" == stable && "$IMAGE" = "" ]] ; then
    # This is the "main" build of the matrix for this commit. The commit might
    # still be on a non-master branch, or caused by a PR rather than a push,
    # etc.
    is_main_build=true
else
    is_main_build=false
fi
echo "is_main_build: $is_main_build"

if [[ "$TRAVIS_OS_NAME" == linux && "$IMAGE" != "" ]] ; then
    # This is a Linux build that should happen inside a custom Docker container.
    is_docker_build=true
else
    is_docker_build=false
fi
echo "is_docker_build: $is_docker_build"

if [[ "$TRAVIS_BRANCH" == master && "$TRAVIS_EVENT_TYPE" == push && "$TRAVIS_TAG" == "" ]] ; then
    # This is a push to master that's not tagged, so we'll want to run our
    # continuous-deployment logic. Note that this variable can be true with
    # $is_main_build being false.
    is_continuous_deployment_build=true
else
    is_continuous_deployment_build=false
fi
echo "is_continuous_deployment_build: $is_continuous_deployment_build"

if [[ "$TRAVIS_BRANCH" == master && "$TRAVIS_EVENT_TYPE" == push && "$TRAVIS_TAG" =~ ^v[0-9]+\. ]] ; then
    # This is a push to master associated with a tag that looks like the regex
    # above, so this seems to be a tagged release. As above, this variable can
    # be true with $is_main_build being false.
    is_release_build=true
else
    is_release_build=false
fi
echo "is_release_build: $is_release_build"

# Start being verbose. We use Travis "fold" commands to provide a bit more
# context, partially because in `set -x` mode the "if" statements just print
# out as `+ false`.

function travis_start_fold() {
    echo -e "\ntravis_fold:start:$1\033[33;1m$2\033[0m"
    set -x
}

function travis_end_fold() {
    set +x
    echo -e "\ntravis_fold:end:$1\r"
}

echo ""

# The special tag "continuous" is used to maintain a GitHub "release" that
# tracks `master`. If we've been triggered for that, that means that something
# *else* was triggered that caused the continuous deployment code to fire.
# So we should do nothing.

travis_start_fold continuous_abort "Abort if special continuous release tag?"
if [[ "$TRAVIS_TAG" == continuous ]] ; then
    exit 0
fi
travis_end_fold continuous_abort

# Pre-build setup.

travis_start_fold pre_build "Pre-build setup for OS = $TRAVIS_OS_NAME"
if [[ "$TRAVIS_OS_NAME" == osx ]]; then
    brew update
    brew install harfbuzz
    brew install --force openssl

    export OPENSSL_INCLUDE_DIR=$(brew --prefix openssl)/include
    export OPENSSL_LIB_DIR=$(brew --prefix openssl)/lib
    export DEP_OPENSSL_INCLUDE=$(brew --prefix openssl)/include
    export PKG_CONFIG_PATH=/usr/local/opt/icu4c/lib/pkgconfig
elif [[ "$TRAVIS_OS_NAME" == linux ]] ; then
    if $is_docker_build ; then
        : # Don't need the deps here; all the action is in the container.
    else
        # We actually use .travis.yml to install basic packages, but we need
        # to install a newer Harfbuzz. At the moment, Travis' Xenial images
        # have an auto-update daemon that can lock the apt/dpkg system under
        # us. See https://github.com/travis-ci/travis-cookbooks/issues/952 and
        # https://unix.stackexchange.com/questions/315502/how-to-disable-apt-daily-service-on-ubuntu-cloud-vm-image
        # . We adopt the workaround from the StackExchange post.

        sudo systemctl stop apt-daily.timer
        sudo systemctl stop apt-daily.service
        sudo systemctl kill --kill-who=all apt-daily.service

        while ! (systemctl list-units --all apt-daily.service | fgrep -q dead) ; do
            sleep 1
        done

        sudo add-apt-repository -y ppa:k-peter/tectonic-ci
        sudo apt-get update
        sudo apt-get install -y libharfbuzz-dev
    fi
fi

rustup component add rustfmt

travis_end_fold pre_build

# Check that the code is properly rustfmt'd.

travis_start_fold check_rustfmt "Maybe check rustfmt? ($is_main_build)"
if $is_main_build ; then
    cargo fmt --all -- --check
fi
travis_end_fold check_rustfmt

# OK, the biggie: does it compile and pass the test suite?

travis_start_fold build_and_test "Build and test"
if $is_docker_build ; then
    docker build -t ttci-$IMAGE dist/docker/$IMAGE/
    docker run -v $(pwd):/tectonic ttci-$IMAGE
else
    cargo build --verbose
    cargo test
fi
travis_end_fold build_and_test

# OK! If we got this far, we think we made a functional set of (debug-mode)
# Tectonic artifacts for this build matrix element.

# The main build is equipped to test code coverage.

travis_start_fold coverage "Maybe analyze code coverage? ($is_main_build)"
if $is_main_build ; then
    sudo apt-get install -y kcov
    cargo install --force cargo-kcov
    cargo test --no-run
    env RUNNING_COVERAGE=1 cargo kcov --no-clean-rebuild
    bash <(curl -s https://codecov.io/bash)
fi
travis_end_fold coverage

# If we're a "continuous deployment" build, we should push up artifacts for
# the "continuous" pseudo-release. Right now, all we do is make an AppImage
# for the main build. I believe that the upload script that we use deletes the
# "continuous" release every time it runs, so if we want different elements of
# the build matrix to contribute various artifacts, we're going to need to
# take a different tactic.

travis_start_fold continuous_deployment "Maybe continuous deployment activities? ($is_continuous_deployment_build)"
if $is_continuous_deployment_build; then
    if $is_main_build; then
        # Careful! For the code coverage, we use "-C link-dead-code", which we
        # don't want for release artifacts. (Which are built with `cargo build
        # --release` inside dist/appimage/build.sh.) But if we ever add other
        # stuff to $RUSTFLAGS, this command will lose it.
        unset RUSTFLAGS

        # Upload an AppImage into the "continuous" release
        wget https://github.com/probonopd/uploadtool/raw/master/upload.sh
        repo_info=$(echo "$TRAVIS_REPO_SLUG" |sed -e 's,/,|,g')
        TECTONIC_APPIMAGE_TAG=continuous \
        UPDATE_INFORMATION="gh-releases-zsync|$repo_info|continuous|tectonic-*.AppImage.zsync" \
            dist/appimage/build.sh
        bash ./upload.sh dist/appimage/tectonic-*.AppImage*
    fi

    # TODO: Do something with the Linux static build?
fi
travis_end_fold continuous_deployment

# If we're a release build, we should create and upload official release
# artifacts, etc.

travis_start_fold release "Maybe release activities? ($is_release_build)"
if $is_release_build; then
    if $is_main_build; then
        # Careful! See the warning above.
        unset RUSTFLAGS

        # Create an AppImage release artifact. (UNTESTED as of 0.1.11!)
        wget https://github.com/probonopd/uploadtool/raw/master/upload.sh
        dist/appimage/build.sh
        bash ./upload.sh dist/appimage/tectonic-*.AppImage*

        # Trigger Arch linux to build a new package.
        openssl aes-256-cbc -K $encrypted_bc40b17e21fa_key -iv $encrypted_bc40b17e21fa_iv \
                -in dist/deploy_key.enc -out /tmp/deploy_key -d
        chmod 600 /tmp/deploy_key
        bash dist/arch/deploy.sh
    fi

    # TODO: Do something with the Linux static build?
fi
travis_end_fold release
