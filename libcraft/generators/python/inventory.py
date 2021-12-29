from pathlib import Path
import common
import collections

data = common.load_feather_json("inventory.json")

# Areas
areas = []
for area in data['areas']:
    areas.append(common.camel_case(area))

# Windows
windows = []
names = {}
inventories = {}
area_offsets = collections.OrderedDict()

for name, window in data['windows'].items():
    variant = common.camel_case(name)
    windows.append(variant)

    names[variant] = name
    inventories[variant] = window['inventories']

    ao = collections.OrderedDict()
    slot_counter = 0
    for inventory_and_area, number_of_slots in window['slots'].items():
        parts = inventory_and_area.split(":")
        inventory = parts[0]
        area_in_inventory = parts[1]
        ao[(inventory, area_in_inventory)] = (slot_counter, number_of_slots)
        slot_counter += number_of_slots
    area_offsets[variant] = ao

output = common.generate_enum("Area", areas)

window = "#[derive(Debug, Clone)] pub enum Window {"
index_to_slot = "#[allow(unused_comparisons)] pub fn index_to_slot(&self, index: usize) -> Option<(&crate::Inventory, Area, usize)> { match self {"
slot_to_index = "pub fn slot_to_index(&self, inventory: &crate::Inventory, area: Area, slot: usize) -> Option<usize> { match self {"

for variant in windows:
    window += f"{variant} {{"
    for inventory in inventories[variant]:
        window += f"{inventory}: crate::Inventory,"
    window += "},"

    match_pattern = f"Window::{variant} {{"
    for inventory in inventories[variant]:
        match_pattern += f"{inventory},"
    match_pattern += "}"

    index_to_slot += f"{match_pattern} => {{"
    first = True
    for (inventory, area_in_inventory), (slot_offset, number_of_slots) in area_offsets[variant].items():
        if not first:
            index_to_slot += "else"
        first = False

        area_in_inventory = common.camel_case(area_in_inventory)
        max_slot = slot_offset + number_of_slots
        slot_offset_operation = ""
        if slot_offset != 0:
            slot_offset_operation += f" - {slot_offset}"
        index_to_slot += f"""
        if ({slot_offset}..{max_slot}).contains(&index) {{
            let area = Area::{area_in_inventory};
            let slot = index{slot_offset_operation};
            Some(({inventory}, area, slot))
        }}   
        """
    index_to_slot += "else { None } },"

    slot_to_index += f"{match_pattern} => {{"
    first = True
    for (inventory, area_in_inventory), (slot_offset, number_of_slots) in area_offsets[variant].items():
        if not first:
            slot_to_index += "else "
        first = False

        area_in_inventory = common.camel_case(area_in_inventory)
        if slot_offset == 0:
            slot_to_index += f"if area == Area::{area_in_inventory} && {inventory}.ptr_eq(inventory) {{ Some(slot) }}"
        else:
            slot_to_index += f"if area == Area::{area_in_inventory} && {inventory}.ptr_eq(inventory) {{ Some(slot + {slot_offset}) }}"

    slot_to_index += "else { None } },"


window += "}"
index_to_slot += "} }"
slot_to_index += "} }"

output += window
output += f"impl Window {{ {index_to_slot} {slot_to_index} }}"
output += common.generate_enum_property("Window", "name", "&str", names, False, "&'static str", True)

# Inventories
inventories = []
for name, areas in data['inventories'].items():
    variant = common.camel_case(name)
    inv = {
        'name': name,
        'variant': variant,
        'areas': areas,
    }
    inventories.append(inv)


output += "#[derive(Debug, Clone)] pub enum InventoryBacking<T> {"
for inventory in inventories:
    variant = inventory['variant']
    output += f"{variant} {{"
    for area_name, area_size in inventory['areas'].items():
        output += f"{area_name}: [T; {area_size}],"
    output += "},"
output += "}"

get_area_fn = "pub fn area_slice(&self, area: Area) -> Option<&[T]> { match self {"
get_areas_fn = "pub fn areas(&self) -> &'static [Area] { match self {"
constructor_fns = ""
inventory_constructor_fns = ""

for inventory in inventories:
    name = inventory['name']
    variant = inventory['variant']
    areas = inventory['areas']
    match_arm = f"InventoryBacking::{variant} {{"
    for area in areas:
        match_arm += f"{area},"
    match_arm += "}"

    get_area_fn += f"{match_arm} => match area {{"
    for area in areas:
        area_variant = common.camel_case(area)
        get_area_fn += f"Area::{area_variant} => Some({area}.as_ref()),"
    get_area_fn += "_ => None },"

    get_areas_fn += f"\nInventoryBacking::{variant} {{ .. }} => {{static AREAS: [Area; {len(areas)}] = ["
    for area in areas:
        get_areas_fn += f"Area::{common.camel_case(area)},"
    get_areas_fn += f"];\n &AREAS }},"

    constructor_fn = f"pub fn {name}() -> Self where T: Default {{ InventoryBacking::{variant} {{"
    for area in areas:
        constructor_fn += f"{area}: Default::default(),"

    constructor_fn += "} }\n"
    constructor_fns += constructor_fn

    inventory_constructor_fns += f"pub fn {name}() -> Self {{ Self {{ backing: std::sync::Arc::new(InventoryBacking::{name}()) }} }}"

get_area_fn += "} }"
get_areas_fn += "} }"
output += f"impl <T> InventoryBacking<T> {{ {get_area_fn} {get_areas_fn} {constructor_fns} }}"
output += f"impl crate::Inventory {{ {inventory_constructor_fns} }}"

common.output("inventory/src/inventory.rs", output)
