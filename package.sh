#!/usr/bin/env bash

INCLUDE=( "src/*" "tests/*" "examples/*" "Cargo.toml" "Cargo.lock" "README.md")

function usage(){
  echo -e "$1\n\nUsage: ./package <crate-name>"
}

function exists(){
  EXISTS=0
  if [[ -d $1 ]]; then EXISTS=1; fi
  if [[ -f $1 ]]; then EXISTS=1; fi
  echo $EXISTS
}

if [[ -z $1 ]]; then usage "Expected crate name"; exit -1; fi
if [[ $(exists $1) == 0 ]]; then usage "Crate not found: $1"; exit -1; fi

# Get the crate file paths
FILES=()
for file in ${INCLUDE[@]}; do
  NAME="$1/$file"
  FILES+="$NAME "
done

ZIP="$1.zip"
7z a $ZIP ${FILES[@]}
mv $ZIP ~/Downloads
