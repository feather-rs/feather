#!/usr/bin/env bash

GENERATOR="cargo run --release --bin feather-generator -- "

${GENERATOR} item-mappings -i data/items/1.13.2.json -o ../items/data/1.13.2.dat
${GENERATOR} item-rust -i data/items/1.13.2.json -o ../items/src/item.rs

${GENERATOR} items-to-blocks --items data/items/1.13.2.json --blocks data/blocks/1.13.2.json --output ../item_block/src/mappings.rs