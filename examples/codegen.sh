#!bash

BASEDIR=$(dirname "$0")
cd $BASEDIR
cargo run --bin codegen > api_trait.rs