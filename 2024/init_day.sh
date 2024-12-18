#!/bin/bash

cp -r base day$1
sed -E "s/base/day$1/g" day$1/Cargo.toml > day$1/Cargo.toml
