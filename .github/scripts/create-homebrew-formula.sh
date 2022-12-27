#!/bin/bash

cd .github/scripts
version="$(./get-version.sh ../../Cargo.toml)"
echo "this is the version ${version}";
cd ../../Formula
echo "Creating formula with this url: https://github.com/anthonwellsjo/dro/releases/tag/${version}"
brew create --rust "https://github.com/anthonwellsjo/dro/releases/tag/${version}" --set-version ${version} --set-license "MIT"
