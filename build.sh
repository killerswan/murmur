   rustc -L . --lib str2.rs \
&& rustc -L . --lib vec2.rs \
&& rustc -L . --lib murmur.rs \
&& rustc -L . test.rs -o test
