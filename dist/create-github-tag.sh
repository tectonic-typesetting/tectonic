#! /bin/bash
# Copyright 2019 the Tectonic Project
# Licensed under the MIT License.

# Create a Git tag on GitHub. This script supports the Tectonic continuous
# deployment workflow.
#
# Arguments:
#
# $1 -- the tag name
# $2 -- the Git commit identifier to associate with the tag
#
# Environment variables:
#
# $GITHUB_TOKEN -- the API token for talking to GitHub
# $TRAVIS_REPO_SLUG -- the "repo slug" (e.g. "tectonic-typesetting/tectonic") we're building

set -o pipefail
set -e

tag_name="$1"
commit_id="$2"

if [ -z "$tag_name" ] ; then
    echo >&2 "error: no tag name specified"
    exit 1
fi

if [ -z "$commit_id" ] ; then
    echo >&2 "error: no commit ID specified"
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

api_base_url="https://api.github.com/repos/$TRAVIS_REPO_SLUG"

echo "info: planning to create lightweight tag $TRAVIS_REPO_SLUG:$tag_name"

tag_json='{
  "ref": "refs/tags/'"$tag_name"'",
  "sha": "'"$commit_id"'"
}'

"${curl[@]}" --data "$tag_json" "$api_base_url/git/refs"
