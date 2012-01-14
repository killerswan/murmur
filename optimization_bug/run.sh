echo "With bench.rs and bench2.rs at --opt-level 0:"
rustc --opt-level 0 -L . main.rs -o main \
&& ./main

echo ""
echo "With bench.rs and bench2.rs at --opt-level 1:"
rustc --opt-level 0 -L . main.rs -o main \
&& ./main
echo ""

