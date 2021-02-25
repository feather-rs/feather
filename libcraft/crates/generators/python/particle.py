from common import load_minecraft_json, output, generate_enum, generate_enum_property, camel_case

particles = []
ids = {}
names = {}

for particle in load_minecraft_json("particles.json", "1.16"):
    variant = camel_case(particle['name'])
    particles.append(variant)
    ids[variant] = particle['id']
    names[variant] = particle['name']

output_data = generate_enum("Particle", particles)
output_data += generate_enum_property("Particle", "id", "u32", ids, True)
output_data += generate_enum_property("Particle", "name", "&str", names, True, "&'static str")

output("crates/core/src/particle.rs", output_data)

