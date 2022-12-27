#!/bin/bash

chmod +x ./.github/scripts/get-version.sh
filename = "dro-$(./.github/scripts/get-version.sh ../../Cargo.toml)-x86_64-apple-darwin.tar.gz"

cd target/release
tar -czf $filename dro
echo "sha256=$(shasum -a 256 $filename)" | awk '{ print $1 }' >> $GITHUB_OUTPUT
mv $filename ../..

