#!/usr/bin/env bash

echo "Generating block ID mappings and code"

GENERATOR="cargo run --release --bin feather_generator -- "

${GENERATOR} native-block-mappings -i block_data/1.13.2.json -o ../blocks/data/1.13.2.dat -v 1.13.2 -p 404
${GENERATOR} block-mappings -i block_data/1.14.4.json -n block_data/1.13.2.json -o ../blocks/data/1.14.4.dat -v 1.14.4 -p 498
${GENERATOR} block-rust -i block_data/1.13.2.json -o ../blocks/src/blocks.rs