#! /usr/bin/env bash
# Copyright 2020 the Tectonic Project
# Licensed under the MIT license

# Auto-update the main Tectonic website repo to show the date of the latest
# release. This script is run from the CI/CD pipeline after the main package is
# updated. The environment variable $GITHUB_TOKEN should be set and the local
# Git user should be configured with proper GitHub authentication and the
# ability to create commits.

set -xeuo pipefail
cd "$(dirname $0)"

version="$(cranko show version tectonic)"
date="$(date +%Y-%m-%d)"

git clone https://github.com/tectonic-typesetting/tectonic-typesetting.github.io.git website
cd website

sed -e "s|^latest:.*|latest: \"$version\"|" \
  -e "s|^latest_date:.*|latest_date: \"$date\"|" \
  -i _config.yml

git add _config.yml
git commit -m "Auto-update for release of version $version"
git push origin master
