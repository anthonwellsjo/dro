#! /bin/bash

cd Formula
version=$(./get-version.sh)
brew create --rust "https://github.com/anthonwellsjo/dro/releases/tag/${version}"
