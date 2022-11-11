#! /bin/bash

cd Formula
version=$(sh ./get-version.sh)
brew create --rust "https://github.com/anthonwellsjo/dro/releases/tag/${version}"
