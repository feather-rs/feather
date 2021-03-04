"""Common code shared by most code generators."""

from subprocess import run
from json import load
from re import split
from pathlib import Path

from typing import List

LIBCRAFT_ROOT = Path(__file__).parents[1] / ".."
PRISMARINEJS_BASE_PATH = Path(__file__).parents[1] / ".." / ".." / "minecraft-data" / "data" / "pc"
LIBCRAFT_DATA_BASE_PATH = Path(__file__).parents[1] / "libcraft-data"


def rustfmt(file_path):
    """ Runs rustfmt on a file"""
    run(["rustfmt", file_path])


def load_minecraft_json(name: str, version="1.16.1") -> dict:
    """
    Loads a JSON file from the minecraft-data sub repository.

    Parameters:
        name (str): Name of the file to load
        version (str): String matching the targe minecraft version, defaults to 1.16.1

    Returns:
        A dict containing JSON content
    """
    file = open(PRISMARINEJS_BASE_PATH / version / name)
    return load(file)


def load_feather_json(name: str) -> dict:
    """
    Loads a JSON file from the feather directory

    Parameters:
        name (str): Name of the file to load

    Returns:
        A dict containing JSON contents
    """
    file = open(LIBCRAFT_DATA_BASE_PATH / name)
    return load(file)


def output(path: str, content: str):
    """
    Writes the contents to a file in provided path, then runs rustfmt.

    Parameters:
        path: Path to destination file, relative to libcraft root
        content: Contents to be written in the file
    """

    path = LIBCRAFT_ROOT / path
    if not path.parent.exists():
        return print(f"Couldn't write to file.\nPath {path.parent} does not exist")
    f = open(path, "w")
    f.write("// This file is @generated. Please do not edit.\n")
    f.write(content)
    f.close()
    print(f"Generated {path.name}")

    rustfmt(path)


def generate_enum_property(
        enum: str,  # Identifier of the enum (e.g. "Biome")
        property_name: str,  # Name of the property
        type_: str,  # The property type (e.g. u32, &str
        mapping: dict,  # Dictionary mapping from enum variant name => property value expression
        # Whether to generate the reverse mapping (property value => Some(Self))
        reverse=False,
        return_type=None,
        # Property type that should be returned. This is used when the type has a lifetime, such as &'static str
        # Whether to bind enum fields using Enum::Variant { .. }
        needs_bindings=False,
) -> str:
    """
    Generates lookup functions for an enum.

    Generates two function for an enum, one which maps the enum value to some 
    property value and one which does the reverse (returning an Option)
    """
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


def generate_enum(name: str, variants: List[str], derives: List[str] = [], prelude: str = "") -> str:
    """Generates an enum definition with the provided variants and extra derives."""
    body = ','.join(variants) + ','
    extra_derives = "" if len(derives) == 0 else ',' + ','.join(derives)
    output = f"""
    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord{extra_derives})]"""
    if len(prelude) != 0:
        output += f"""
                    {prelude}"""
    output += f"""
    pub enum {name} {{
        {body}
    }}
    """
    return output


def camel_case(string: str) -> str:
    """Converts a string to UpperCamelCase."""
    return ''.join(a.capitalize() for a in split('([^a-zA-Z0-9])', string) if a.isalnum())
