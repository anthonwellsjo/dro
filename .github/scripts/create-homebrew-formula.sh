#! /bin/bash

version="$(sh get-version.sh | cut -d "=" -f 2)"
cd .github/scripts
cd ../../Formula
brew create --rust "https://github.com/anthonwellsjo/dro/releases/tag/${version}"
