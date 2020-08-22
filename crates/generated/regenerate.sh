generators=(
  "biome"
  "block"
  "entity"
  "inventory"
  "item"
  "particle"
)

for generator in ${generators[@]}; do
  python3 generators/$generator.py
done

cargo fmt
