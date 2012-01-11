LEVEL=0

   rustc --opt-level $LEVEL -L . --lib str2.rs \
&& rustc --opt-level $LEVEL -L . --lib vec2.rs \
&& rustc --opt-level $LEVEL -L . --lib djb.rs \
&& rustc --opt-level $LEVEL -L . bench.rs -o bench \
&& ./bench

