#! /bin/bash

version=$(sh get-version.sh)
cd .github/scripts
version=${version.CURRENT_VERSION}
cd ../../Formula
brew create --rust "https://github.com/anthonwellsjo/dro/releases/tag/${version}"
