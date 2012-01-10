   rustc -O -L . --lib str2.rs \
&& rustc -O -L . --lib vec2.rs \
&& rustc -O -L . --lib murmur.rs \
&& rustc -O -L . test.rs -o test \
&& rustc -O -L . bench.rs -o bench
