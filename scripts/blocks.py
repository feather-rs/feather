# Script for generating the block state file (core/world/block.rs).
# This script reads from the vanilla server's blocks.json data report
# and produces corresponding Rust code.

# Rustfmt should be run on the generated code,
# because proper formatting is not accounted
# for here.

import json
import sys
from collections import OrderedDict


# Strips the "minecraft:" prefix from
# a block identifier, leaving only the snake_case
# block name.
def strip_minecraft_identifier(s):
    return s[10:]


# Converts a snake_case string to
# UpperCamelCase.
def snake_to_upper_camel(s):
    components = s.split("_")
    return "".join(x.title() for x in components)


in_path = sys.argv[1]
out_path = sys.argv[2]

print("Generating block state code into %s using block state file %", in_path, out_path)

fin = open(in_path, "r+")
fout = open(out_path, "w+")

print("Reading data")

data = json.load(fin, object_pairs_hook=OrderedDict)

enum_code = ""

enum_code += "#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]\n" \
        "pub enum Block {\n"

block_state_id_code = ""

block_state_id_code += "pub fn block_state_id(&self) -> u16 {\n" \
                       "match self {"

from_block_state_id_code = ""

from_block_state_id_code += "pub fn from_block_state_id(id: u16) -> Self {\n" \
                            "match id {" \

for block_prop_name in data:
    print("Reading for block %s", block_prop_name)
    block_name = strip_minecraft_identifier(block_prop_name)
    print("Block name: %s", block_name)
    obj = data[block_prop_name]

    states = obj["states"]
    single_state = len(states) == 1

    if single_state:
        camel = snake_to_upper_camel(block_name)
        id = str(states[0]["id"])
        enum_code += camel + ",\n"
        block_state_id_code += "Block::" + camel + " => " + id + ",\n"
        from_block_state_id_code += id + " => Block::" + camel + ",\n"

    else:
        print("unimplemented")


enum_code += "}"

block_state_id_code += "}\n}"

from_block_state_id_code += "_ => panic!(\"Unknown block state ID {}\", id),"
from_block_state_id_code += "}\n}"

fout.write(enum_code)
fout.write("impl Block {\n")
fout.write(block_state_id_code)
fout.write("\n\n")
fout.write(from_block_state_id_code)
fout.write("}")
