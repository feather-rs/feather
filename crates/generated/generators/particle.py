import common as common

data = common.load_minecraft_json("particles.json", "1.16")

particles = []
ids = {}
names = {}

for particle in data:
    variant = common.camel_case(particle['name'])
    particles.append(variant)
    ids[variant] = particle['id']
    names[variant] = particle['name']

output = common.generate_enum("Particle", particles)
output += common.generate_enum_property("Particle", "id", "u32", ids, True)
output += common.generate_enum_property("Particle", "name", "&str", names, True, "&'static str")

common.output("src/particle.rs", output)
