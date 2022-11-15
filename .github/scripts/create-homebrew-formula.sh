#! /bin/bash

ver="$(sh get-version.sh)"
cd .github/scripts
version=${ver.CURRENT_VERSION}
cd ../../Formula
brew create --rust "https://github.com/anthonwellsjo/dro/releases/tag/${version}"
