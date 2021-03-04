generators=$(find python/ -type f -name "*.py")

echo "Running python generators"
for generator in ${generators[@]}; do
    echo "Running $generator"
    python3 $generator
done

echo "Running rust generators"
cargo run --package libcraft-generators --bin libcraft-generators

cargo fmt