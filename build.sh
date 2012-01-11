LEVEL=2

   rustc --opt-level $LEVEL -L . --lib str2.rs   \
&& rustc --opt-level $LEVEL -L . --lib vec2.rs   \
&& rustc --opt-level $LEVEL -L . --lib murmur.rs \
&& rustc --opt-level $LEVEL -L . test.rs -o test \
&& rustc --opt-level $LEVEL -L . bench.rs -o bench
