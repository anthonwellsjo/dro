#! /bin/bash

version="$(sh get-version.sh | cut -d "=" -f 2)"
cd .github/scripts
cd ../../Formula
echo "Creating formula with this url: https://github.com/anthonwellsjo/dro/releases/tag/${version}"
brew create --rust "https://github.com/anthonwellsjo/dro/releases/tag/${version} --set-version ${version} --set-license "MIT"" 
