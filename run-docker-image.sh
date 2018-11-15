#!/bin/bash

image=$1

docker run -v $(pwd):/tectonic $image
