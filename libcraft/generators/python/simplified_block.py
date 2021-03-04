from common import load_minecraft_json, load_feather_json, camel_case, generate_enum, generate_enum_property, output
from re import compile

blocks = load_minecraft_json("blocks.json")
simplified_block = load_feather_json("simplified_block.json")

regexes = {}
for name, regex in simplified_block['regexes'].items():
    regexes[name] = compile(regex)

variants = []
mapping = {}
for name in regexes:
    variants.append(camel_case(name))

for block in blocks:
    name = block['name']
    block_variant = camel_case(name)

    # Detect which SimplifiedBlockKind matches this block.
    found = False
    for simplified, regex in regexes.items():
        if regex.match(name) is not None:
            mapping[block_variant] = "SimplifiedBlockKind::" + camel_case(simplified)
            found = True
            break

    if not found:
        # Default to block variant
        variants.append(block_variant)
        mapping[block_variant] = "SimplifiedBlockKind::" + block_variant

output_data = "use crate::BlockKind;" + generate_enum("SimplifiedBlockKind", variants)
output_data += generate_enum_property("BlockKind", "simplified_kind", "SimplifiedBlockKind", mapping)
output("blocks/src/simplified_block.rs", output_data)
