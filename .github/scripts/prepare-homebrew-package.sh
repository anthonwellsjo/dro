#!/bin/bash

if [ $# -eq 0 ]; then
  echo "Error: No version argument given."
  exit 1
fi

filename="dro-$1-x86_64-apple-darwin.tar.gz"

cd target/release
tar -czf $filename dro
echo "sha256=$(shasum -a 256 $filename)" | awk '{ print $1 }' >> $GITHUB_OUTPUT
mv $filename ../..

