#! /bin/bash

cd .github/scripts
version=$(sh get-version.sh)
cd ../../Formula
brew create --rust "https://github.com/anthonwellsjo/dro/releases/tag/${version}"
