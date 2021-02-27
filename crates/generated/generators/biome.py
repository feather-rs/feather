# Generation of the Biome enum. Uses minecraft-data/biomes.json.
import common

data = common.load_minecraft_json("biomes.json")

variants = []
ids = {}
names = {}
display_names = {}
rainfalls = {}
temperatures = {}

for biome in data:
    variant = common.camel_case(biome['name'])
    variants.append(variant)
    ids[variant] = biome['id']
    names[variant] = biome['name']
    display_names[variant] = biome['displayName']
    rainfalls[variant] = biome['rainfall']
    temperatures[variant] = biome['temperature']


output = common.generate_enum("Biome", variants)
output += common.generate_enum_property("Biome", "id", "u32", ids, True)
output += common.generate_enum_property("Biome", "name", "&str", names, True, "&'static str")
output += common.generate_enum_property("Biome", "display_name", "&str", display_names, True, "&'static str")
output += common.generate_enum_property("Biome", "rainfall", "f32", rainfalls)
output += common.generate_enum_property("Biome", "temperature", "f32", temperatures)

common.output("src/biome.rs", output)

