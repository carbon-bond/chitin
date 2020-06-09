#!bash

BASEDIR=$(dirname "$0")
cd $BASEDIR
cargo run --bin codegen -- $1