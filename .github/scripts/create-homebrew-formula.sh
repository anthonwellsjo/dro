#! /bin/bash

cd Formula
version=$(/bin/bash/ sh ./get-version.sh)
brew create --rust "https://github.com/anthonwellsjo/dro/releases/tag/${version}"
