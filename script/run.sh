#!/bin/sh

dirname=$(dirname $(readlink -f "$0"))
parent=$(dirname $dirname)

for file in $(ls $parent -I README.md -I script); do
  path="$parent/$file"
  read -p "delete $path? " -r REPLY && test "$REPLY" = y && rm -rf $path || exit 1
done

$dirname/crates.ts | cut -f 2 -d ' ' | xargs -I {} cargo watt build --crate {} -o {}