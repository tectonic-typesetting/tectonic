@ECHO OFF
REM Copyright 2021 the Tectonic Project
REM Licensed under the MIT License

IF "%1"=="success" (
  ECHO fake biber says success and makes a file
  ECHO 456 >biberout.qqq
  EXIT 0
) ELSE IF "%1"=="failure" (
  ECHO fake biber says failure
  EXIT 1
) ELSE (
  ECHO unexpected mode
  EXIT 1
)
