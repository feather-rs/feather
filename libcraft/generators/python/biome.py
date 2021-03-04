"""Generation of the Biome enum. Uses minecraft-data/biomes.json."""
from common import load_minecraft_json, camel_case, generate_enum, generate_enum_property, output

variants = []
ids = {}
names = {}
display_names = {}
rainfalls = {}
temperatures = {}

for biome in load_minecraft_json("biomes.json"):
    variant = camel_case(biome['name'])
    variants.append(variant)
    ids[variant] = biome['id']
    names[variant] = biome['name']
    display_names[variant] = biome['displayName']
    rainfalls[variant] = biome['rainfall']
    temperatures[variant] = biome['temperature']


output_data = generate_enum("Biome", variants)
output_data += generate_enum_property("Biome", "id", "u32", ids, True)
output_data += generate_enum_property("Biome", "name", "&str", names, True, "&'static str")
output_data += generate_enum_property("Biome", "display_name", "&str", display_names, True, "&'static str")
output_data += generate_enum_property("Biome", "rainfall", "f32", rainfalls)
output_data += generate_enum_property("Biome", "temperature", "f32", temperatures)

output("core/src/biome.rs", output_data)

