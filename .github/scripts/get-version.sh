#!/bin/bash

# Extract the version value from a cargo.toml file

# Check if a cargo.toml file was specified as an argument
if [ $# -eq 0 ]; then
  echo "Error: No cargo.toml file specified"
  exit 1
fi

# Extract the version value using grep and sed
version=$(grep '^version =' $1 | sed -E 's/^version *= *"([^"]+)"/\1/')

# Print the extracted version value
echo "v$version"
