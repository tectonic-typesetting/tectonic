#! /bin/bash
# Copyright 2019 the Tectonic Project
# Licensed under the MIT License.

# Delete the named github release and its associated tag. This script is meant
# for our "continuous" continuous deployment release that hosts artifacts
# associated with the most recent build on `master`.
#
# Arguments:
#
# $1 -- the release tag name
#
# Environment variables:
#
# $GITHUB_TOKEN -- the API token for talking to GitHub
# $TRAVIS_REPO_SLUG -- the "repo slug" (e.g. "tectonic-typesetting/tectonic") we're building

set -o pipefail
set -e

release_name="$1"

if [ -z "$release_name" ] ; then
    echo >&2 "error: no release name specified"
    exit 1
fi

if [ -z "$GITHUB_TOKEN" ] ; then
    echo >&2 "error: no GITHUB_TOKEN environment variable"
    exit 1
fi

curl=(
    curl
    -s
    --header "Authorization: token ${GITHUB_TOKEN}"
)

if [ -z "$TRAVIS_REPO_SLUG" ] ; then
    echo >&2 "error: no TRAVIS_REPO_SLUG environment variable"
    exit 1
fi

# For now ...
if [ "$release_name" != continuous ] ; then
    echo >&2 "error: safety check: refusing to run with release name other than \"continuous\""
    exit 1
fi

api_base_url="https://api.github.com/repos/$TRAVIS_REPO_SLUG"

echo "info: planning to delete release $TRAVIS_REPO_SLUG:$release_name"

if ! type jql >/dev/null 2>&1 ; then
    echo "info: installing jql"
    cargo install -q --force jql
fi

release_id=$("${curl[@]}" -XGET "$api_base_url/releases/tags/$release_name" | jql '"id"')

if [ -z "$release_id" ] ; then
    echo "warning: couldn't get release ID; presuming no such release"
else
    echo "info: deleting release"
    "${curl[@]}" -XDELETE "$api_base_url/releases/$release_id"
fi

echo "info: deleting tag"
"${curl[@]}" -XDELETE "$api_base_url/git/refs/tags/$release_name"
