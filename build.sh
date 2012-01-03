rustc --lib vec2.rs \
&& rustc --lib murmur.rs -L . \
&& rustc test.rs -o test -L .
