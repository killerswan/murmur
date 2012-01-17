#!/bin/bash

RUST_MIN_STACK=1000000
export RUST_MIN_STACK

LEVEL=3

rm ../rust-tools/*.so *.so

   rustc --opt-level $LEVEL -L . --lib ./djb.rs \
&& rustc --opt-level $LEVEL -L ../rust-tools --lib ../rust-tools/vec2.rs \
&& rustc --opt-level $LEVEL -L . -L ../rust-tools --lib murmur.rs \
&& rustc --opt-level $LEVEL -L . -L ../rust-tools bench.rs -o bench \
&& ./bench

#&& rustc --opt-level $LEVEL -L . test.rs -o test \
#&& ./test \


