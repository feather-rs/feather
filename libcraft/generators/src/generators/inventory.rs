use convert_case::{Case, Casing};
use indexmap::IndexMap;
use proc_macro2::TokenStream;
use serde::de::{Error, Unexpected};
use serde::Deserializer;

use crate::utils::*;

pub fn generate() {
    let inventories: Inventories = load_libcraft_json("inventory.json").unwrap();

    let mut out = generate_enum!(
        Area,
        inventories
            .areas
            .iter()
            .map(|area| area.0.to_case(Case::UpperCamel))
            .collect::<Vec<_>>()
    );

    let mut window_offsets = IndexMap::new();
    for (name, window) in &inventories.windows {
        let mut offset = 0;
        let mut offsets = IndexMap::new();
        for (inv_area, slots) in &window.slots {
            offsets.insert(inv_area, offset..(offset + slots));
            offset += slots;
        }
        window_offsets.insert(name, offsets);
    }

    let mut window_declaration = Vec::new();
    let mut index_to_slot = TokenStream::new();
    let mut slot_to_index = TokenStream::new();
    for (name, window) in &inventories.windows {
        let window_name = format_ident!("{}", name.0.to_case(Case::UpperCamel));
        let window_inventories = window
            .inventories
            .iter()
            .map(|inv| format_ident!("{}", inv.0))
            .collect::<Vec<_>>();

        window_declaration.push(quote! {
            #window_name {
                #(#window_inventories: crate::Inventory),*
            }
        });

        let window_binding = quote! {
            Window::#window_name {
                #(#window_inventories),*
            }
        };
        let inv = window_offsets
            .get(name)
            .unwrap()
            .keys()
            .map(|inv_area| format_ident!("{}", inv_area.0 .0))
            .collect::<Vec<_>>();
        let area = window_offsets
            .get(name)
            .unwrap()
            .keys()
            .map(|inv_area| format_ident!("{}", inv_area.1 .0.to_case(Case::UpperCamel)))
            .collect::<Vec<_>>();
        let offset_start = window_offsets
            .get(name)
            .unwrap()
            .values()
            .map(|offset| offset.start)
            .collect::<Vec<_>>();
        let offset_end = window_offsets
            .get(name)
            .unwrap()
            .values()
            .map(|offset| offset.end - 1)
            .collect::<Vec<_>>();
        index_to_slot.extend(quote! {
            #((#window_binding, #offset_start..=#offset_end) => {
                Some((#inv, Area::#area, index - #offset_start))
            },)*
        });
        slot_to_index.extend(quote! {
            #((#window_binding, Area::#area) if #inv.ptr_eq(inventory) => {
                Some(slot + #offset_start)
            })*,
        });
    }
    let mut inventory_declaration = Vec::new();
    let mut area_slice = TokenStream::new();
    let mut areas = TokenStream::new();
    let mut new_backing = TokenStream::new();
    let mut new_inventory = TokenStream::new();
    for (name, inventory_areas) in &inventories.inventories {
        let inventory_name = format_ident!("{}", name.0);
        let inventory_name_camel_case = format_ident!("{}", name.0.to_case(Case::UpperCamel));
        let area_name = inventory_areas
            .keys()
            .map(|area| format_ident!("{}", area.0))
            .collect::<Vec<_>>();
        let area_name_camel_case = inventory_areas
            .keys()
            .map(|area| format_ident!("{}", area.0.to_case(Case::UpperCamel)))
            .collect::<Vec<_>>();
        let area_size = inventory_areas.values().collect::<Vec<_>>();
        inventory_declaration.push(quote! {
            #inventory_name_camel_case {
                #(#area_name: [T; #area_size]),*
            }
        });
        let inventory_binding = quote! {
            InventoryBacking::#inventory_name_camel_case {
                #(#area_name),*
            }
        };
        area_slice.extend(quote! {
            #((#inventory_binding, Area::#area_name_camel_case) => Some(#area_name),)*
        });
        areas.extend(quote! {
            InventoryBacking::#inventory_name_camel_case { .. } => &[ #(Area::#area_name_camel_case),* ],
        });
        new_backing.extend(quote! {
            pub fn #inventory_name() -> Self where T: Default {
                InventoryBacking::#inventory_name_camel_case {
                    #(#area_name: Default::default()),*
                }
            }
        });
        new_inventory.extend(quote! {
            pub fn #inventory_name() -> Self {
                Self {
                    inner: std::rc::Rc::new(crate::Inner {
                        backing: InventoryBacking::#inventory_name(),
                        slot_mutated_callback: std::cell::RefCell::new(None),
                    }),
                }
            }
        });
    }
    out.extend(quote! {
        #[derive(Debug, Clone)]
        pub enum Window {
            #(#window_declaration),*
        }
        impl Window {
            pub fn index_to_slot(&self, index: usize) -> Option<(&crate::Inventory, Area, usize)> {
                match (self, index) {
                    #index_to_slot
                    _ => None
                }
            }
            pub fn slot_to_index(&self, inventory: &crate::Inventory, area: Area, slot: usize) -> Option<usize> {
                match (self, area) {
                    #slot_to_index
                    _ => None
                }
            }
        }
        #[derive(Debug, Clone)]
        pub enum InventoryBacking<T> {
            #(#inventory_declaration),*
        }
        impl<T> InventoryBacking<T> {
            #new_backing
            pub fn area_slice(&self, area: Area) -> Option<&[T]> {
                match (self, area) {
                    #area_slice
                    _ => None
                }
            }
            pub fn areas(&self) -> &'static [Area] {
                match self {
                    #areas
                }
            }
        }
        impl crate::Inventory {
            #new_inventory
        }
    });

    output(
        "libcraft/inventory/src/inventory.rs",
        out.to_string().as_str(),
    );
}

#[derive(Deserialize)]
struct Inventories {
    areas: Vec<AreaName>,
    inventories: IndexMap<InventoryName, IndexMap<AreaName, usize>>,
    windows: IndexMap<WindowName, WindowInfo>,
}

#[derive(Deserialize)]
struct WindowInfo {
    inventories: Vec<InventoryName>,
    slots: IndexMap<InventoryArea, usize>,
}

#[derive(Deserialize, Eq, PartialEq, Hash)]
struct AreaName(String);
#[derive(Deserialize, Eq, PartialEq, Hash)]
struct InventoryName(String);
#[derive(Deserialize, Eq, PartialEq, Hash)]
struct WindowName(String);

#[derive(Eq, PartialEq, Hash)]
struct InventoryArea(InventoryName, AreaName);

impl<'de> Deserialize<'de> for InventoryArea {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let (inventory, area) = s.split_once(':').ok_or_else(|| {
            D::Error::invalid_value(
                Unexpected::Str(&s),
                &"string in format 'inventory_name:area_name'",
            )
        })?;
        Ok(InventoryArea(
            InventoryName(inventory.to_owned()),
            AreaName(area.to_owned()),
        ))
    }
}
