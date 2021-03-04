from common import load_minecraft_json, camel_case, generate_enum, generate_enum_property, output


# build item ID => item kind index
item_kinds_by_id = {}
for item in load_minecraft_json("items.json"):
    item_kinds_by_id[item['id']] = camel_case(item['name'])

# Build material name => dig multipliers index
material_dig_multipliers = {}
for name, material in load_minecraft_json("materials.json").items():
    dig_multipliers = {}
    for item_id, multiplier in material.items():
        dig_multipliers[item_kinds_by_id[int(item_id)]] = float(multiplier)
    material_dig_multipliers[name] = dig_multipliers

# Build material dig multipliers constants
material_constants = ""
material_constant_refs = {}
for name, dig_multipliers in material_dig_multipliers.items():
    dm = ""
    for item, multiplier in dig_multipliers.items():
        dm += f"(libcraft_items::Item::{item}, {multiplier}_f32),"
    constant = f"DIG_MULTIPLIERS_{name}"
    material_constants += f"#[allow(dead_code, non_upper_case_globals)] const {constant}: &[(libcraft_items::Item, f32)] = &[{dm}];"
    material_constant_refs[name] = constant

blocks = []
ids = {}
names = {}
display_names = {}
hardnesses = {}
diggables = {}
harvest_tools = {}
transparents = {}
light_emissions = {}
light_filters = {}
dig_multipliers = {}
solids = {}

for block in load_minecraft_json("blocks.json"):
    variant = camel_case(block['name'])
    blocks.append(variant)
    ids[variant] = block['id']
    names[variant] = block['name']
    display_names[variant] = block['displayName']
    hardnesses[variant] = block['hardness']
    if hardnesses[variant] is None:
        hardnesses[variant] = 0
    diggables[variant] = block['diggable']
    transparents[variant] = block['transparent']
    light_emissions[variant] = block['emitLight']
    light_filters[variant] = block['filterLight']

    solids[variant] = block['boundingBox'] == 'block'

    # Dig multipliers
    material = block.get('material')
    if material_constant_refs.get(material) is not None:
        constant = material_constant_refs[material]
        dig_multipliers[variant] = f"{constant}"
    else:
        dig_multipliers[variant] = "&[]"

    # Harvest tools
    ht = ""
    for tool_id in block.get('harvestTools', {}):
        kind = item_kinds_by_id[int(tool_id)]
        ht += f"libcraft_items::Item::{kind},"

    if len(ht) == 0:
        harvest_tools[variant] = 'None'
    else:
        harvest_tools[variant] = f"""
        const TOOLS: &[libcraft_items::Item] = &[{ht}];
        Some(TOOLS)
        """

output_data = "#[derive(num_derive::FromPrimitive, num_derive::ToPrimitive, serde::Serialize, serde::Deserialize)]" + \
    generate_enum("BlockKind", blocks)
output_data += generate_enum_property("BlockKind", "id", "u32", ids, True)
output_data += generate_enum_property("BlockKind", "name", "&str", names, True, "&'static str")
output_data += generate_enum_property("BlockKind", "display_name", "&str", display_names, True, "&'static str")
output_data += generate_enum_property("BlockKind", "hardness", "f32", hardnesses)
output_data += generate_enum_property("BlockKind", "diggable", "bool", diggables)
output_data += generate_enum_property("BlockKind", "transparent", "bool", transparents)
output_data += generate_enum_property("BlockKind", "light_emission", "u8", light_emissions)
output_data += generate_enum_property("BlockKind", "light_filter", "u8", light_filters)
output_data += generate_enum_property("BlockKind", "solid", "bool", solids)
output_data += material_constants
output_data += generate_enum_property("BlockKind", "dig_multipliers",
                                      "&'static [(libcraft_items::Item, f32)]", dig_multipliers)
output_data += generate_enum_property("BlockKind", "harvest_tools",
                                      "Option<&'static [libcraft_items::Item]>", harvest_tools)

output("blocks/src/block.rs", output_data)
