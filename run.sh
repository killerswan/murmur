LEVEL=3

   rustc --opt-level $LEVEL -L ./djb --lib ./djb/djb.rs \
&& rustc --opt-level $LEVEL -L ../rust-tools --lib ../rust-tools/str2.rs \
&& rustc --opt-level $LEVEL -L ../rust-tools --lib ../rust-tools/vec2.rs \
&& rustc --opt-level $LEVEL -L . --lib murmur.rs \
&& rustc --opt-level $LEVEL -L . -L ./djb -L ../rust-tools bench.rs -o bench \
&& ./bench

#&& rustc --opt-level $LEVEL -L . test.rs -o test \
#&& ./test \


