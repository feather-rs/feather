# Common code shared by most code generators.

import subprocess
import json
from re import split


# Runs rustfmt on a file.
def rustfmt(file_path):
    subprocess.run(["rustfmt", file_path])


PRISMARINEJS_BASE_PATH = "../../minecraft-data/data/pc"
FEATHER_BASE_PATH = "feather"


# Loads in a JSON file from the minecraft-data file with the given name.
def load_minecraft_json(name, version="1.16.1"):
    file = open(f'{PRISMARINEJS_BASE_PATH}/{version}/{name}')
    return json.load(file)


# Loads in a JSON file from the feather file with the given name.
def load_feather_json(name):
    file = open(f"{FEATHER_BASE_PATH}/{name}")
    return json.load(file)


# Writes Rust source to an output file, then runs rustfmt.
def output(path, content):
    f = open(path, "w")
    f.write("// This file is @generated. Please do not edit.\n")
    f.write(content)
    f.close()

    rustfmt(path)


# Generate two functions for an enum, one which maps
# the enum value to some property value and one which
# does the reverse (returning an Option).
def generate_enum_property(
        enum,  # Identifier of the enum (e.g. "Biome")
        property_name, # Name of the property
        type_,  # The property type (e.g. u32, &str
        mapping,  # Dictionary mapping from enum variant name => property value expression
        reverse=False, # Whether to generate the reverse mapping (property value => Self)
        return_type=None,
        # Property type that should be returned. This is used when the type has a lifetime, such as &'static str
        needs_bindings=False, # Whether to bind enum fields using Enum::Variant { .. }
):
    if return_type is None:
        return_type = type_

    self_to_prop = ""
    prop_to_self = ""

    # Add quotes to strings
    if type_ == "&str":
        for key, property_value in mapping.items():
            mapping[key] = f'"{property_value}"'

    # If floats are needed, convert integers to floats
    if type_ == "f32" or type_ == "f64":
        for key, property_value in mapping.items():
            mapping[key] = f'{property_value} as {type_}'

    # Bools are lowercase in Rust
    if type_ == "bool":
        for key, property_value in mapping.items():
            mapping[key] = str(property_value).lower()

    for variant, property_value in mapping.items():
        fields = ""
        if needs_bindings:
            fields = "{ .. }"
        self_to_prop += f"{enum}::{variant} {fields} => {{ {property_value} }},"
        prop_to_self += f"{property_value} => Some({enum}::{variant}),"

    result = f"""
        #[allow(warnings)]
        #[allow(clippy::all)]
        impl {enum} {{
            /// Returns the `{property_name}` property of this `{enum}`.
            pub fn {property_name}(&self) -> {return_type} {{
                match self {{
                    {self_to_prop}
                }}
            }}
    """

    if reverse:
        result += f"""
            /// Gets a `{enum}` by its `{property_name}`.
            pub fn from_{property_name}({property_name}: {type_}) -> Option<Self> {{
                match {property_name} {{
                    {prop_to_self}
                    _ => None,
                }}
            }}
    """

    # closing brace
    result += "}"

    return result


# Generates an enum definition with the provided variants.
def generate_enum(name, variants):
    body = ""
    for variant in variants:
        body += f"{variant},"

    return f"""
    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub enum {name} {{
        {body}
    }}
    """


# Converts a string to UpperCamelCase.
def camel_case(string):
    return ''.join(a.capitalize() for a in split('([^a-zA-Z0-9])', string)
       if a.isalnum())
