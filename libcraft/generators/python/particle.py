# This file cannot be generated anymore, since the current particle.rs in libcraft/crates/particles has a 
# is an enum that has the particle data built into it.

# I made an attempt on incorporating this into the generator, but ultimately gave up since it's not future-proof
# at all

from common import load_minecraft_json, output, generate_enum, generate_enum_property, camel_case
    
def main ():
    particles = []
    ids = {}
    names = {}

    types = load_minecraft_json("protocol.json", "1.16")["types"]["particleData"][1]['fields']
    print(types)

    for particle in load_minecraft_json("particles.json", "1.16"):
        variant = camel_case(particle['name'])
        id = str(particle['id'])
        if id in types.keys():
            data = types[id]
            print(data[1])
            particles.append(generate_particle_data(variant, data[1]))
        else: 
            particles.append(variant)
        ids[variant] = id
        names[variant] = particle['name']

    output_data = generate_enum("Particle", particles)
    output_data += generate_enum_property("Particle", "id", "u32", ids, True)
    output_data += generate_enum_property("Particle", "name", "&str", names, True, "&'static str")
    output("core/src/particle.rs", output_data)

def generate_particle_data (name: str, data: dict):

    if (len(data) == 1):
        feather_type = "f32"
        if data[0]['name'] == 'blockState':
            feather_type = "BlockId"
        return name + f"({feather_type})"
    else:
        enum_item = f"{name}{{"
        for i in range(0, len(data)):
            enum_item += f"{data[i]['name']}:{data[i]['type']}"
            if i < len(data):
                enum_item += ","

        return enum_item + "}"

if __name__ == "__main__": 
  main()