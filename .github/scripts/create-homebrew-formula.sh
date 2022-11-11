#! /bin/bash

cd Formula
version=$(/bin/bash ./get-version.sh)
brew create --rust "https://github.com/anthonwellsjo/dro/releases/tag/${version}"
