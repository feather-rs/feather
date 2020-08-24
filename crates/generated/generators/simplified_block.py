import common
import re

blocks = common.load_minecraft_json("blocks.json")
simplified_block = common.load_feather_json("simplified_block.json")

regexes = {}
for name, regex in simplified_block['regexes'].items():
    regexes[name] = re.compile(regex)

variants = []
mapping = {}
for name in regexes:
    variants.append(common.camel_case(name))

for block in blocks:
    name = block['name']
    block_variant = common.camel_case(name)

    # Detect which SimplifiedBlockKind matches this block.
    found = False
    for simplified, regex in regexes.items():
        if regex.match(name) is not None:
            mapping[block_variant] = "SimplifiedBlockKind::" + common.camel_case(simplified)
            found = True
            break

    if not found:
        # Default to block variant
        variants.append(block_variant)
        mapping[block_variant] = "SimplifiedBlockKind::" + block_variant

output = "use crate::BlockKind;" + common.generate_enum("SimplifiedBlockKind", variants)
output += common.generate_enum_property("BlockKind", "simplified_kind", "SimplifiedBlockKind", mapping)
common.output("src/simplified_block.rs", output)
