#! /bin/sh
# Copyright 2021 the Tetonic Project
# Licensed under the MIT License.

# A stand-in for biber for our testing framework.

if [ "$1" = success ] ; then
  echo "fake biber says success and makes a file"
  echo 456 >biberout.qqq
  exit 0
elif [ "$1" = failure ] ; then
  echo "fake biber says failure"
  exit 1
elif [ "$1" = signal ] ; then
  echo "fake biber kills self"
  kill -QUIT $$
fi

echo "unexpected mode"
exit 1
