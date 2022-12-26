#! /bin/bash

cd target/release
tar -czf dro-mac.tar.gz dro
echo "sha256=$(shasum -a 256 dro-mac.tar.gz)" | awk '{ print $1 }' >> $GITHUB_OUTPUT
mv dro-mac.tar.gz ../..

