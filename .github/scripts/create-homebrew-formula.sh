#! /bin/bash

cd .github/scripts
version="$(sh get-version.sh | cut -d "=" -f 2)"
echo "this is the version ${version}";
cd ../../Formula
echo "Creating formula with this url: https://github.com/anthonwellsjo/dro/releases/tag/${version}"
brew create --rust "https://github.com/anthonwellsjo/dro/releases/tag/${version}" --set-version ${version} --set-license "MIT"
