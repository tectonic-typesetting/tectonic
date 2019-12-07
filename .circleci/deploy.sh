#! /bin/bash
# Copyright 2018-2019 the Tectonic Project
# Licensed under the MIT License

# Work-in-progress as I figure out how CircleCI wires things up!

echo "work dir: $CIRCLE_WORKING_DIRECTORY"

ls -l $CIRCLE_WORKING_DIRECTORY

echo =========================

ls -l $CIRCLE_WORKING_DIRECTORY/*

echo =========================

ls -l $CIRCLE_WORKING_DIRECTORY/*/*
