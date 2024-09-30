#!/bin/bash

find $ANDROID_NDK_HOME -name 'libunwind.a' | \
  sed 's@libunwind.a$@libgcc.a@' | \
  while read x; do
    echo "INPUT(-lunwind)" > $x
  done
