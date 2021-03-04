//! Constants representing various standard inventory slot indices
//! for the `Player` window
//! Deprecated; mainly exists for interop with world saves.

pub const SLOT_CRAFTING_OUTPUT: usize = 0;
pub const SLOT_CRAFTING_INPUT_X0_Y0: usize = 1;
pub const SLOT_CRAFTING_INPUT_X1_Y0: usize = 2;
pub const SLOT_CRAFTING_INPUT_X0_Y1: usize = 3;
pub const SLOT_CRAFTING_INPUT_X1_Y1: usize = 4;

pub const SLOT_ARMOR_MIN: usize = 5;
pub const SLOT_ARMOR_MAX: usize = 8;

pub const SLOT_ARMOR_HEAD: usize = 5;
pub const SLOT_ARMOR_CHEST: usize = 6;
pub const SLOT_ARMOR_LEGS: usize = 7;
pub const SLOT_ARMOR_FEET: usize = 8;

pub const SLOT_OFFHAND: usize = 45;

pub const SLOT_INVENTORY_OFFSET: usize = 9;
pub const SLOT_HOTBAR_OFFSET: usize = 36;

pub const HOTBAR_SIZE: usize = 9;
pub const INVENTORY_SIZE: usize = 27;

pub const SLOT_ENTITY_EQUIPMENT_MAIN_HAND: usize = 0;
pub const SLOT_ENTITY_EQUIPMENT_OFF_HAND: usize = 1;
pub const SLOT_ENTITY_EQUIPMENT_BOOTS: usize = 2;
pub const SLOT_ENTITY_EQUIPMENT_LEGGINGS: usize = 3;
pub const SLOT_ENTITY_EQUIPMENT_CHESTPLATE: usize = 4;
pub const SLOT_ENTITY_EQUIPMENT_HELMET: usize = 5;
