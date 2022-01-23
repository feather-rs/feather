from common import load_minecraft_json, camel_case, generate_enum, generate_enum_property, output

effects = []
ids = {}
names = {}
display_names = {}
is_good = {}

for effect in load_minecraft_json("effects.json", "1.16.1"):
    variant = effect['name']
    effects.append(variant)
    ids[variant] = effect['id']
    names[variant] = effect['name']
    display_names[variant] = effect['displayName']
    is_good[variant] = True if effect['type'] == "good" else False

enumName = "Effect"

output_data = generate_enum(enumName, effects, ["serde::Deserialize", "serde::Serialize"])
output_data += generate_enum_property(enumName, "id", "u8", ids, True)
output_data += generate_enum_property(enumName, "name", "&str", names, True, "&'static str")
output_data += generate_enum_property(enumName, "display_name", "&str", display_names, True, "&'static str")
output_data += generate_enum_property(enumName, "is_good", "bool", is_good)

output("effects/src/effect.rs", output_data)
