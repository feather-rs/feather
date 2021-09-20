from os import listdir
import common
prefix = "../datapacks/minecraft/data/minecraft/tags/"
block_tags = listdir(prefix + "blocks")
entity_types = listdir(prefix + "entity_types")
fluid_tags = listdir(prefix + "fluids")
item_tags = listdir(prefix + "items")
tag_list_list = (block_tags, entity_types, fluid_tags, item_tags)
enum_names = ("VanillaBlockTags", "VanillaEntityTypes", "VanillaFluidTags", "VanillaItemTags")
new_line = "\n"
quotes = "\""
output = ""
for (tags, enum_name) in zip(tag_list_list, enum_names):
    output += "#[derive(Copy, Clone, Debug, PartialEq, Eq)]\n"
    output += f"pub enum {enum_name} {{{new_line}"
    for s in tags:
        camelcase = ''.join(map(str.title, s[:-5].split('_')))
        output += f"    {camelcase},{new_line}"
    output += f"}}{new_line}{new_line}"
for (tags, enum_name) in zip(tag_list_list, enum_names):
    output += f"impl {enum_name} {{{new_line}"
    output += f"    pub fn name(&self) -> &'static str {{{new_line}"
    output += f"        match self {{{new_line}"
    for s in tags:
        snakecase = s[:-5]
        camelcase = ''.join(map(str.title, snakecase.split('_')))
        output += f"            {enum_name}::{camelcase} => {quotes}{snakecase}{quotes},{new_line}"
    output += "        }\n    }\n}\n"
    output += f"impl From<{enum_name}> for &'static str {{{new_line}"
    output += f"    fn from(tag: {enum_name}) -> Self {{{new_line}"
    output += "        tag.name()\n"
    output += "    }\n}\n"
common.output("src/vanilla_tags.rs", output)