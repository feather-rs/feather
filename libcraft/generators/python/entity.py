from common import load_minecraft_json, camel_case, generate_enum, generate_enum_property, output

entities = []
ids = {}
internal_ids = {}
names = {}
display_names = {}
bboxes = {}

for entity in load_minecraft_json("entities.json","1.16.2"):
    variant = camel_case(entity['name'])
    entities.append(variant)
    ids[variant] = entity['id']
    internal_ids[variant] = entity['internalId']
    names[variant] = entity['name']
    display_names[variant] = entity['displayName']

    width = entity['width']
    height = entity['height']
    bboxes[variant] = f"vek::Aabb {{ min: vek::Vec3::zero(), max: vek::Vec3::new({width} as f64, {height} as f64, {width} as f64), }}"

output_data = generate_enum("EntityKind", entities)
output_data += generate_enum_property("EntityKind", "id", "u32", ids, True)
output_data += generate_enum_property("EntityKind", "internal_id", "u32", internal_ids, True)
output_data += generate_enum_property("EntityKind", "name", "&str", names, True, "&'static str")
output_data += generate_enum_property("EntityKind", "display_name", "&str", display_names, True, "&'static str")
output_data += generate_enum_property("EntityKind", "bounding_box", "vek::Aabb<f64>", bboxes)

output("core/src/entity.rs", output_data)
