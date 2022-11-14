#! /bin/bash

cd .github/scripts
version=$(sh get-version.sh | awk '{ print $1 } )
cd ../../Formula
brew create --rust "https://github.com/anthonwellsjo/dro/releases/tag/${version}"
