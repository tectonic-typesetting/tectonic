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

# Exit immediately if a command exits with a non-zero status.
set -e

echo ""

# We use `travis_retry` [1] to deal with transient network errors for commands
# that download things (e.g. apt-get, wget).
#
# Notes:
# * `cargo` does retry. [2]
# * `rustup` does not retry [3], but it may be coming soon. [4]
#
# [1] https://docs.travis-ci.com/user/common-build-problems/#timeouts-installing-dependencies
# [2] https://github.com/rust-lang/cargo/pull/2396
# [3] https://github.com/rust-lang/rustup.rs/issues/1667
# [4] https://github.com/rust-lang/rustup.rs/pull/1722

# We use `travis_fold` commands to hide chunks of the Travis-CI log. Follow the
# usage documentation below and put `travis_fold_start`/`travis_fold_end` pairs
# _inside_ `if` blocks to reduce the log noise. (For example, after `set -x`,
# `if` statements print out as `+ false`, which is not very useful.)

# Usage: travis_fold_start <fold-name> <title> [<verbose>]
#
#   Start a fold with a name and title and (optionally) enable Bash verbose
#   logging.
#
#   <fold-name>  string to use with travis_fold_end to close the fold
#   <title>      string that appears alone when the fold is closed
#   <verbose>    (optional) if non-empty, enables Bash verbose echoing
#
function travis_fold_start() {
    echo "travis_fold:start:$1"
    echo -e "\033[33;1m$2\033[0m"
    if [ -n "$3" ]; then
        set -x
    fi
}

# Usage: travis_fold_end <fold-name>
#
#   End a fold started with the given name and disable Bash verbose logging in
#   case it was enabled by `travis_fold_start`.
#
#   <fold-name>  string that should have been used with travis_fold_start to
#                open the fold
#
function travis_fold_end() {
    set +x
    echo ""
    echo "travis_fold:end:$1"
}

# Helpful context.

travis_fold_start env "Environment variables"
echo "TRAVIS_ALLOW_FAILURE: $TRAVIS_ALLOW_FAILURE"
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
    # This is a push to master that's not tagged, so we'll want to initiate
    # our continuous-deployment (CD) workflow. Note that this variable can be
    # true with $is_main_build being false.
    is_continuous_deployment_trigger=true
else
    is_continuous_deployment_trigger=false
fi
echo "is_continuous_deployment_trigger: $is_continuous_deployment_trigger"

publish_artifacts=false
version_text=none
upload_artifact=false

if [[ "$TRAVIS_EVENT_TYPE" == push && "$TRAVIS_TAG" == continuous ]] ; then
    # The second half of our CD workflow is triggered by the creation of a new
    # tag called "continuous". Creating that tag triggers another CI run,
    # which is the one that does most of the actual work.
    is_continuous_deployment_build=true
    publish_artifacts=true
    version_text=latest
    upload_artifacts=(
        dist/add-github-release-artifact.sh
        -d "Continuous deployment of commit $TRAVIS_COMMIT"
        -p  # prerelease
        continuous
    )
else
    is_continuous_deployment_build=false
fi
echo "is_continuous_deployment_build: $is_continuous_deployment_build"

if [[ "$TRAVIS_EVENT_TYPE" == push && "$TRAVIS_TAG" =~ ^v[0-9]+\. ]] ; then
    # This is a push of tag that matches the above regex: a new release. As
    # above, this variable can be true with $is_main_build being false.
    is_release_build=true
    publish_artifacts=true
    version_text="$(echo "$TRAVIS_TAG" |sed -e 's/^v//')"
    upload_artifacts=(
        dist/add-github-release-artifact.sh
        -d "Release tag $TRAVIS_TAG"
        $TRAVIS_TAG
    )
else
    is_release_build=false
fi
echo "is_release_build: $is_release_build"
echo "publish_artifacts: $publish_artifacts (v. $version_text)"
travis_fold_end env

# The main build of the CD trigger deletes the previous GitHub Release and tag
# named "continuous", then recreates the tag to point to the current commit.
# This makes it so that the same workflows can run for both the CD case and
# when full release tags are pushed. Another nice aspect of this approach is
# that it does the right thing even although multiple instances of this script
# are running without any way to communicate with each other.

if $is_continuous_deployment_trigger ; then
    if $is_main_build ; then
        travis_fold_start launch_cd "Trigger continuous deployment workflow" verbose
        dist/delete-github-release-and-tag.sh continuous
        dist/create-github-tag.sh continuous $TRAVIS_COMMIT
        travis_fold_end launch_cd
    fi

    echo -e "\033[34;1mThis is a continuous deployment trigger build. Exiting.\033[0m"
    sleep 5  # work around travis-ci/travis-ci#6018
    exit 0
fi

# Install dependencies

if [[ "$TRAVIS_OS_NAME" == osx ]]; then
    travis_fold_start install_deps "Install dependencies" verbose
    export OPENSSL_INCLUDE_DIR=$(brew --prefix openssl)/include
    export OPENSSL_LIB_DIR=$(brew --prefix openssl)/lib
    export DEP_OPENSSL_INCLUDE=$(brew --prefix openssl)/include
    export PKG_CONFIG_PATH=/usr/local/opt/icu4c/lib/pkgconfig
    travis_fold_end install_deps
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

        travis_fold_start install_deps "Install dependencies" verbose
        sudo systemctl stop apt-daily.timer
        sudo systemctl stop apt-daily.service
        sudo systemctl kill --kill-who=all apt-daily.service

        while ! (systemctl list-units --all apt-daily.service | fgrep -q dead) ; do
            sleep 1
        done

        travis_retry sudo add-apt-repository -y ppa:k-peter/tectonic-ci
        travis_retry sudo apt-get update
        travis_retry sudo apt-get install -y libharfbuzz-dev
        travis_fold_end install_deps
    fi
fi

# Check that the code is properly rustfmt'd and clippy'd.

if $is_main_build ; then
    travis_fold_start cargo_fmt "cargo fmt" verbose
    travis_retry rustup component add rustfmt
    cargo fmt --all -- --check
    travis_fold_end cargo_fmt
    travis_fold_start cargo_clippy "cargo clippy" verbose
    travis_retry rustup component add clippy
    cargo clippy --all --all-targets --all-features -- --deny warnings
    travis_fold_end cargo_clippy
fi

# OK, the biggie: does it compile and pass the test suite?

export RUST_BACKTRACE=1

if $is_docker_build ; then
    travis_fold_start docker_build "docker build" verbose
    docker build --build-arg=uid=$(id -u) -t ttci-$IMAGE dist/docker/$IMAGE/
    travis_fold_end docker_build
    travis_fold_start docker_test "docker test" verbose
    docker run -v $(pwd):/alpine/home/rust/src ttci-$IMAGE
    travis_fold_end docker_test
else
    travis_fold_start cargo_build_no_default_features "cargo build --no-default-features" verbose
    cargo build --no-default-features --verbose
    travis_fold_end cargo_build_no_default_features
    travis_fold_start cargo_build "cargo build" verbose
    cargo build --verbose
    travis_fold_end cargo_build
    travis_fold_start cargo_test "cargo test" verbose
    cargo test
    travis_fold_end cargo_test
fi

# OK! If we got this far, we think we made a functional set of (debug-mode)
# Tectonic artifacts for this build matrix element.

# The main build is equipped to test code coverage.

if $is_main_build ; then
    travis_fold_start cargo_kcov "cargo kcov" verbose
    travis_retry sudo apt-get install -y kcov
    cargo install --force cargo-kcov
    cargo test --no-run
    env RUNNING_COVERAGE=1 cargo kcov --no-clean-rebuild
    bash <(curl -s https://codecov.io/bash)
    travis_fold_end cargo_kcov
fi

# The docs/ mdbook: only built in the main build.

if $is_main_build ; then
    if $publish_artifacts ; then
        verb="build and publish"

        if $is_release_build ; then
            book_desc="docs mdbook @ v$version_text"
        else  # continuous deployment
            book_desc="docs mdbook @ $TRAVIS_COMMIT"
        fi
    else
        verb="build"
    fi

    travis_fold_start docs_mdbook "mdbook $verb docs" verbose
    dist/build-mdbook.sh docs

    if $publish_artifacts; then
        dist/force-push-tree.sh \
            docs/book \
            https://github.com/tectonic-typesetting/book.git \
            "$version_text" \
            "$book_desc"
    fi

    travis_fold_end docs_mdbook
fi

# Everything until now has been exercising and validating the build. The rest
# of this script builds release artifacts, which is only worth doing if we
# have a place to put them. Currently, this happens for continuous deployment
# and release tag builds, but not pull requests.

if ! $publish_artifacts; then
    echo -e "\033[34;1mThis build does not include release artifacts. Stopping here.\033[0m"
    sleep 5  # work around travis-ci/travis-ci#6018
    exit 0
fi

# Careful! For the code coverage, we use "-C link-dead-code" in the main
# build, which we don't want for release artifacts. But if we ever add other
# stuff to $RUSTFLAGS, this command will lose it.

if $is_main_build ; then
    unset RUSTFLAGS
fi

# XXXX temporary -- check out filenames for other artifacts we can make

ls -l target/*

# The Docker build makes a statically-linked executable that we can
# publish.

if $is_docker_build; then
    travis_fold_start dockerpub "Publish Docker-built binary artifact" verbose
    tarname="tectonic-$version_text-$IMAGE.tar.gz"
    (cd target/$IMAGE/release && tar czf ../../../"$tarname" tectonic) || exit $?
    "${upload_artifacts[@]}" "$tarname"
    travis_fold_end dockerpub
fi

# AppImage artifact: currently only Linux/x86_64, i.e. main build

if $is_main_build; then
    travis_fold_start appimage "Build AppImage" verbose

    if $is_continuous_deployment_build ; then
        # In CD, we include extra information for incremental updates.
        repo_info=$(echo "$TRAVIS_REPO_SLUG" |sed -e 's,/,|,g')
        export TECTONIC_APPIMAGE_TAG=continuous
        export UPDATE_INFORMATION="gh-releases-zsync|$repo_info|continuous|tectonic-*.AppImage.zsync"
    fi

    dist/appimage/build.sh
    "${upload_artifacts[@]}" dist/appimage/tectonic-*.AppImage*
    travis_fold_end appimage
fi

# Everything below here is only for tagged releases.

if ! $is_release_build; then
    echo -e "\033[34;1mThis build is not a tagged release. Stopping here.\033[0m"
    sleep 5  # work around travis-ci/travis-ci#6018
    exit 0
fi

# Yet more work to do. Trigger Arch Linux to build a new package.

if $is_main_build; then
    travis_fold_start arch_linux "Update Arch Linux package" verbose
    openssl aes-256-cbc -K $encrypted_bc40b17e21fa_key -iv $encrypted_bc40b17e21fa_iv \
            -in dist/deploy_key.enc -out /tmp/deploy_key -d
    chmod 600 /tmp/deploy_key
    bash dist/arch/deploy.sh
    travis_fold_end arch_linux
fi

# All done! Work around travis-ci/travis-ci#6018

sleep 5
exit 0
