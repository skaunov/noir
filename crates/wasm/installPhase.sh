#!/usr/bin/env bash

mkdir -p $out

cp ./README.md ./pkg/
cp ./package.json ./pkg/
cp -r ./pkg/* $out/

echo "## Tracking" >> $out/README.md
echo "Built from [noir-lang/noir@$GIT_COMMIT](https://github.com/noir-lang/noir/tree/$GIT_COMMIT)" >> $out/README.md
