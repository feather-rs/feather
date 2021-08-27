from os import listdir
prefix = "minecraft/data/minecraft/tags/"
block_tags = listdir(prefix + "blocks")
entity_types = listdir(prefix + "entity_types")
fluid_tags = listdir(prefix + "fluids")
item_tags = listdir(prefix + "items")
tag_list_list = (block_tags, entity_types, fluid_tags, item_tags)
enum_names = ("VanillaBlockTags", "VanillaEntityTypes", "VanillaFluidTags", "VanillaItemTags")
f = open("src/vanilla_tags.rs", "w")
new_line = "\n"
quotes = "\""
f.write("use std::str::FromStr;\n")
for (tags, enum_name) in zip(tag_list_list, enum_names):
    f.write("#[derive(Copy, Clone, Debug, PartialEq, Eq)]\n")
    f.write(f"pub enum {enum_name} {{{new_line}")
    for s in tags:
        camelcase = ''.join(map(str.title, s[:-5].split('_')))
        f.write(f"    {camelcase},{new_line}")
    f.write(f"}}{new_line}{new_line}")
for (tags, enum_name) in zip(tag_list_list, enum_names):
    f.write(f"impl {enum_name} {{{new_line}")
    f.write(f"    pub fn name(&self) -> &'static str {{{new_line}")
    f.write(f"        match self {{{new_line}")
    for s in tags:
        snakecase = s[:-5]
        camelcase = ''.join(map(str.title, snakecase.split('_')))
        f.write(f"            {enum_name}::{camelcase} => {quotes}{snakecase}{quotes},{new_line}")
    f.write("        }\n    }\n")
    f.write("    pub fn namespaced_name(&self) -> crate::NamespacedId {\n")
    f.write("        crate::NamespacedId::from_str(self.name()).unwrap()\n")
    f.write("    }\n}\n")
    f.write(f"impl From<{enum_name}> for &'static str {{{new_line}")
    f.write(f"    fn from(tag: {enum_name}) -> Self {{{new_line}")
    f.write("        tag.name()\n")
    f.write("    }\n}\n")
    f.write(f"impl From<{enum_name}> for crate::NamespacedId {{{new_line}")
    f.write(f"    fn from(tag: {enum_name}) -> Self {{{new_line}")
    f.write("        tag.namespaced_name()\n")
    f.write("    }\n}\n")



