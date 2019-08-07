#!/usr/bin/env bash

GENERATOR="cargo run --release --bin feather_generator -- "

${GENERATOR} item-mappings -i data/items/1.13.2.json -o ../items/data/1.13.2.dat
${GENERATOR} item-rust -i data/items/1.13.2.json -o ../items/src/item.rs