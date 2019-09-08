#!/usr/bin/env bash

cargo run --release --bin feather-generator -- biomes -i data/registries.json -o ../core/src/biomes.rs