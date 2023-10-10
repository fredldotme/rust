#!/bin/bash

# This is supposed to be used as a Tide submodule

RUST_PWD=$(pwd)
LLVM_PWD=$RUST_PWD/../llvm
LLVM_BIN=$LLVM_PWD/build_osx

LLVM_BIN=$(echo "$LLVM_BIN" | sed "s/\//\\//g")
echo "LLVM binaries: $LLVM_BIN"

cp config.tide.toml config.toml

sed -i '' "s|@CC@|$LLVM_BIN/bin/clang|g" config.toml
sed -i '' "s|@CXX@|$LLVM_BIN/bin/clang++|g" config.toml
sed -i '' "s|@AR@|$LLVM_BIN/bin/llvm-ar|g" config.toml
sed -i '' "s|@RANLIB@|$LLVM_BIN/bin/llvm-ranlib|g" config.toml
sed -i '' "s|@LLVMCONFIG@|$LLVM_BIN/bin/llvm-config|g" config.toml

./x.py build --target aarch64-apple-ios