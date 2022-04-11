// This file is @generated. Please do not edit.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Area {
    Storage,
    CraftingOutput,
    CraftingInput,
    Helmet,
    Chestplate,
    Leggings,
    Boots,
    Hotbar,
    Offhand,
    FurnaceIngredient,
    FurnaceFuel,
    FurnaceOutput,
    EnchantmentItem,
    EnchantmentLapis,
    BrewingBottle,
    BrewingIngredient,
    BrewingBlazePowder,
    VillagerInput,
    VillagerOutput,
    BeaconPayment,
    AnvilInput1,
    AnvilInput2,
    AnvilOutput,
    Saddle,
    HorseArmor,
    LlamaCarpet,
    CartographyMap,
    CartographyPaper,
    CartographyOutput,
    GrindstoneInput1,
    GrindstoneInput2,
    GrindstoneOutput,
    LecternBook,
    LoomBanner,
    LoomDye,
    LoomPattern,
    LoomOutput,
    StonecutterInput,
    StonecutterOutput,
}
impl Area {
    #[inline]
    pub fn values() -> &'static [Area] {
        use Area::*;
        &[
            Storage,
            CraftingOutput,
            CraftingInput,
            Helmet,
            Chestplate,
            Leggings,
            Boots,
            Hotbar,
            Offhand,
            FurnaceIngredient,
            FurnaceFuel,
            FurnaceOutput,
            EnchantmentItem,
            EnchantmentLapis,
            BrewingBottle,
            BrewingIngredient,
            BrewingBlazePowder,
            VillagerInput,
            VillagerOutput,
            BeaconPayment,
            AnvilInput1,
            AnvilInput2,
            AnvilOutput,
            Saddle,
            HorseArmor,
            LlamaCarpet,
            CartographyMap,
            CartographyPaper,
            CartographyOutput,
            GrindstoneInput1,
            GrindstoneInput2,
            GrindstoneOutput,
            LecternBook,
            LoomBanner,
            LoomDye,
            LoomPattern,
            LoomOutput,
            StonecutterInput,
            StonecutterOutput,
        ]
    }
}
#[derive(Debug, Clone)]
pub enum Window {
    Player {
        player: crate::Inventory,
    },
    Generic9X1 {
        block: crate::Inventory,
        player: crate::Inventory,
    },
    Generic9X2 {
        block: crate::Inventory,
        player: crate::Inventory,
    },
    Generic9X3 {
        block: crate::Inventory,
        player: crate::Inventory,
    },
    Generic9X4 {
        block: crate::Inventory,
        player: crate::Inventory,
    },
    Generic9X5 {
        block: crate::Inventory,
        player: crate::Inventory,
    },
    Generic9X6 {
        left_chest: crate::Inventory,
        right_chest: crate::Inventory,
        player: crate::Inventory,
    },
    Generic3X3 {
        block: crate::Inventory,
        player: crate::Inventory,
    },
    Crafting {
        crafting_table: crate::Inventory,
        player: crate::Inventory,
    },
    Furnace {
        furnace: crate::Inventory,
        player: crate::Inventory,
    },
    BlastFurnace {
        blast_furnace: crate::Inventory,
        player: crate::Inventory,
    },
    Smoker {
        smoker: crate::Inventory,
        player: crate::Inventory,
    },
    Enchantment {
        enchantment_table: crate::Inventory,
        player: crate::Inventory,
    },
    BrewingStand {
        brewing_stand: crate::Inventory,
        player: crate::Inventory,
    },
    Beacon {
        beacon: crate::Inventory,
        player: crate::Inventory,
    },
    Anvil {
        anvil: crate::Inventory,
        player: crate::Inventory,
    },
    Hopper {
        hopper: crate::Inventory,
        player: crate::Inventory,
    },
    ShulkerBox {
        shulker_box: crate::Inventory,
        player: crate::Inventory,
    },
    Cartography {
        cartography_table: crate::Inventory,
        player: crate::Inventory,
    },
    Grindstone {
        grindstone: crate::Inventory,
        player: crate::Inventory,
    },
    Lectern {
        lectern: crate::Inventory,
        player: crate::Inventory,
    },
    Loom {
        loom: crate::Inventory,
        player: crate::Inventory,
    },
    Stonecutter {
        stonecutter: crate::Inventory,
        player: crate::Inventory,
    },
}
impl Window {
    pub fn index_to_slot(&self, index: usize) -> Option<(&crate::Inventory, Area, usize)> {
        match (self, index) {
            (Window::Player { player }, 0usize..=0usize) => {
                Some((player, Area::CraftingOutput, index - 0usize))
            }
            (Window::Player { player }, 1usize..=4usize) => {
                Some((player, Area::CraftingInput, index - 1usize))
            }
            (Window::Player { player }, 5usize..=5usize) => {
                Some((player, Area::Helmet, index - 5usize))
            }
            (Window::Player { player }, 6usize..=6usize) => {
                Some((player, Area::Chestplate, index - 6usize))
            }
            (Window::Player { player }, 7usize..=7usize) => {
                Some((player, Area::Leggings, index - 7usize))
            }
            (Window::Player { player }, 8usize..=8usize) => {
                Some((player, Area::Boots, index - 8usize))
            }
            (Window::Player { player }, 9usize..=35usize) => {
                Some((player, Area::Storage, index - 9usize))
            }
            (Window::Player { player }, 36usize..=44usize) => {
                Some((player, Area::Hotbar, index - 36usize))
            }
            (Window::Player { player }, 45usize..=45usize) => {
                Some((player, Area::Offhand, index - 45usize))
            }
            (Window::Generic9X1 { block, player }, 0usize..=8usize) => {
                Some((block, Area::Storage, index - 0usize))
            }
            (Window::Generic9X1 { block, player }, 9usize..=35usize) => {
                Some((player, Area::Storage, index - 9usize))
            }
            (Window::Generic9X1 { block, player }, 36usize..=44usize) => {
                Some((player, Area::Hotbar, index - 36usize))
            }
            (Window::Generic9X2 { block, player }, 0usize..=17usize) => {
                Some((block, Area::Storage, index - 0usize))
            }
            (Window::Generic9X2 { block, player }, 18usize..=44usize) => {
                Some((player, Area::Storage, index - 18usize))
            }
            (Window::Generic9X2 { block, player }, 45usize..=53usize) => {
                Some((player, Area::Hotbar, index - 45usize))
            }
            (Window::Generic9X3 { block, player }, 0usize..=26usize) => {
                Some((block, Area::Storage, index - 0usize))
            }
            (Window::Generic9X3 { block, player }, 27usize..=53usize) => {
                Some((player, Area::Storage, index - 27usize))
            }
            (Window::Generic9X3 { block, player }, 54usize..=62usize) => {
                Some((player, Area::Hotbar, index - 54usize))
            }
            (Window::Generic9X4 { block, player }, 0usize..=35usize) => {
                Some((block, Area::Storage, index - 0usize))
            }
            (Window::Generic9X4 { block, player }, 36usize..=62usize) => {
                Some((player, Area::Storage, index - 36usize))
            }
            (Window::Generic9X4 { block, player }, 63usize..=71usize) => {
                Some((player, Area::Hotbar, index - 63usize))
            }
            (Window::Generic9X5 { block, player }, 0usize..=44usize) => {
                Some((block, Area::Storage, index - 0usize))
            }
            (Window::Generic9X5 { block, player }, 45usize..=71usize) => {
                Some((player, Area::Storage, index - 45usize))
            }
            (Window::Generic9X5 { block, player }, 72usize..=80usize) => {
                Some((player, Area::Hotbar, index - 72usize))
            }
            (
                Window::Generic9X6 {
                    left_chest,
                    right_chest,
                    player,
                },
                0usize..=26usize,
            ) => Some((left_chest, Area::Storage, index - 0usize)),
            (
                Window::Generic9X6 {
                    left_chest,
                    right_chest,
                    player,
                },
                27usize..=53usize,
            ) => Some((right_chest, Area::Storage, index - 27usize)),
            (
                Window::Generic9X6 {
                    left_chest,
                    right_chest,
                    player,
                },
                54usize..=80usize,
            ) => Some((player, Area::Storage, index - 54usize)),
            (
                Window::Generic9X6 {
                    left_chest,
                    right_chest,
                    player,
                },
                81usize..=89usize,
            ) => Some((player, Area::Hotbar, index - 81usize)),
            (Window::Generic3X3 { block, player }, 0usize..=8usize) => {
                Some((block, Area::Storage, index - 0usize))
            }
            (Window::Generic3X3 { block, player }, 9usize..=35usize) => {
                Some((player, Area::Storage, index - 9usize))
            }
            (Window::Generic3X3 { block, player }, 36usize..=44usize) => {
                Some((player, Area::Hotbar, index - 36usize))
            }
            (
                Window::Crafting {
                    crafting_table,
                    player,
                },
                0usize..=0usize,
            ) => Some((crafting_table, Area::CraftingOutput, index - 0usize)),
            (
                Window::Crafting {
                    crafting_table,
                    player,
                },
                1usize..=9usize,
            ) => Some((crafting_table, Area::CraftingInput, index - 1usize)),
            (
                Window::Crafting {
                    crafting_table,
                    player,
                },
                10usize..=36usize,
            ) => Some((player, Area::Storage, index - 10usize)),
            (
                Window::Crafting {
                    crafting_table,
                    player,
                },
                37usize..=45usize,
            ) => Some((player, Area::Hotbar, index - 37usize)),
            (Window::Furnace { furnace, player }, 0usize..=0usize) => {
                Some((furnace, Area::FurnaceIngredient, index - 0usize))
            }
            (Window::Furnace { furnace, player }, 1usize..=1usize) => {
                Some((furnace, Area::FurnaceFuel, index - 1usize))
            }
            (Window::Furnace { furnace, player }, 2usize..=2usize) => {
                Some((furnace, Area::FurnaceOutput, index - 2usize))
            }
            (Window::Furnace { furnace, player }, 3usize..=29usize) => {
                Some((player, Area::Storage, index - 3usize))
            }
            (Window::Furnace { furnace, player }, 30usize..=38usize) => {
                Some((player, Area::Hotbar, index - 30usize))
            }
            (
                Window::BlastFurnace {
                    blast_furnace,
                    player,
                },
                0usize..=0usize,
            ) => Some((blast_furnace, Area::FurnaceIngredient, index - 0usize)),
            (
                Window::BlastFurnace {
                    blast_furnace,
                    player,
                },
                1usize..=1usize,
            ) => Some((blast_furnace, Area::FurnaceFuel, index - 1usize)),
            (
                Window::BlastFurnace {
                    blast_furnace,
                    player,
                },
                2usize..=2usize,
            ) => Some((blast_furnace, Area::FurnaceOutput, index - 2usize)),
            (
                Window::BlastFurnace {
                    blast_furnace,
                    player,
                },
                3usize..=29usize,
            ) => Some((player, Area::Storage, index - 3usize)),
            (
                Window::BlastFurnace {
                    blast_furnace,
                    player,
                },
                30usize..=38usize,
            ) => Some((player, Area::Hotbar, index - 30usize)),
            (Window::Smoker { smoker, player }, 0usize..=0usize) => {
                Some((smoker, Area::FurnaceIngredient, index - 0usize))
            }
            (Window::Smoker { smoker, player }, 1usize..=1usize) => {
                Some((smoker, Area::FurnaceFuel, index - 1usize))
            }
            (Window::Smoker { smoker, player }, 2usize..=2usize) => {
                Some((smoker, Area::FurnaceOutput, index - 2usize))
            }
            (Window::Smoker { smoker, player }, 3usize..=29usize) => {
                Some((player, Area::Storage, index - 3usize))
            }
            (Window::Smoker { smoker, player }, 30usize..=38usize) => {
                Some((player, Area::Hotbar, index - 30usize))
            }
            (
                Window::Enchantment {
                    enchantment_table,
                    player,
                },
                0usize..=0usize,
            ) => Some((enchantment_table, Area::EnchantmentItem, index - 0usize)),
            (
                Window::Enchantment {
                    enchantment_table,
                    player,
                },
                1usize..=1usize,
            ) => Some((enchantment_table, Area::EnchantmentLapis, index - 1usize)),
            (
                Window::Enchantment {
                    enchantment_table,
                    player,
                },
                2usize..=28usize,
            ) => Some((player, Area::Storage, index - 2usize)),
            (
                Window::Enchantment {
                    enchantment_table,
                    player,
                },
                29usize..=37usize,
            ) => Some((player, Area::Hotbar, index - 29usize)),
            (
                Window::BrewingStand {
                    brewing_stand,
                    player,
                },
                0usize..=2usize,
            ) => Some((brewing_stand, Area::BrewingBottle, index - 0usize)),
            (
                Window::BrewingStand {
                    brewing_stand,
                    player,
                },
                3usize..=3usize,
            ) => Some((brewing_stand, Area::BrewingIngredient, index - 3usize)),
            (
                Window::BrewingStand {
                    brewing_stand,
                    player,
                },
                4usize..=4usize,
            ) => Some((brewing_stand, Area::BrewingBlazePowder, index - 4usize)),
            (
                Window::BrewingStand {
                    brewing_stand,
                    player,
                },
                5usize..=31usize,
            ) => Some((player, Area::Storage, index - 5usize)),
            (
                Window::BrewingStand {
                    brewing_stand,
                    player,
                },
                32usize..=40usize,
            ) => Some((player, Area::Hotbar, index - 32usize)),
            (Window::Beacon { beacon, player }, 0usize..=0usize) => {
                Some((beacon, Area::BeaconPayment, index - 0usize))
            }
            (Window::Beacon { beacon, player }, 1usize..=27usize) => {
                Some((player, Area::Storage, index - 1usize))
            }
            (Window::Beacon { beacon, player }, 28usize..=36usize) => {
                Some((player, Area::Hotbar, index - 28usize))
            }
            (Window::Anvil { anvil, player }, 0usize..=0usize) => {
                Some((anvil, Area::AnvilInput1, index - 0usize))
            }
            (Window::Anvil { anvil, player }, 1usize..=1usize) => {
                Some((anvil, Area::AnvilInput2, index - 1usize))
            }
            (Window::Anvil { anvil, player }, 2usize..=2usize) => {
                Some((anvil, Area::AnvilOutput, index - 2usize))
            }
            (Window::Anvil { anvil, player }, 3usize..=29usize) => {
                Some((player, Area::Storage, index - 3usize))
            }
            (Window::Anvil { anvil, player }, 30usize..=38usize) => {
                Some((player, Area::Hotbar, index - 30usize))
            }
            (Window::Hopper { hopper, player }, 0usize..=3usize) => {
                Some((hopper, Area::Storage, index - 0usize))
            }
            (Window::Hopper { hopper, player }, 4usize..=30usize) => {
                Some((player, Area::Storage, index - 4usize))
            }
            (Window::Hopper { hopper, player }, 31usize..=39usize) => {
                Some((player, Area::Hotbar, index - 31usize))
            }
            (
                Window::ShulkerBox {
                    shulker_box,
                    player,
                },
                0usize..=26usize,
            ) => Some((shulker_box, Area::Storage, index - 0usize)),
            (
                Window::ShulkerBox {
                    shulker_box,
                    player,
                },
                27usize..=53usize,
            ) => Some((player, Area::Storage, index - 27usize)),
            (
                Window::ShulkerBox {
                    shulker_box,
                    player,
                },
                54usize..=62usize,
            ) => Some((player, Area::Hotbar, index - 54usize)),
            (
                Window::Cartography {
                    cartography_table,
                    player,
                },
                0usize..=0usize,
            ) => Some((cartography_table, Area::CartographyMap, index - 0usize)),
            (
                Window::Cartography {
                    cartography_table,
                    player,
                },
                1usize..=1usize,
            ) => Some((cartography_table, Area::CartographyPaper, index - 1usize)),
            (
                Window::Cartography {
                    cartography_table,
                    player,
                },
                2usize..=2usize,
            ) => Some((cartography_table, Area::CartographyOutput, index - 2usize)),
            (
                Window::Cartography {
                    cartography_table,
                    player,
                },
                3usize..=29usize,
            ) => Some((player, Area::Storage, index - 3usize)),
            (
                Window::Cartography {
                    cartography_table,
                    player,
                },
                30usize..=38usize,
            ) => Some((player, Area::Hotbar, index - 30usize)),
            (Window::Grindstone { grindstone, player }, 0usize..=0usize) => {
                Some((grindstone, Area::GrindstoneInput1, index - 0usize))
            }
            (Window::Grindstone { grindstone, player }, 1usize..=1usize) => {
                Some((grindstone, Area::GrindstoneInput2, index - 1usize))
            }
            (Window::Grindstone { grindstone, player }, 2usize..=2usize) => {
                Some((grindstone, Area::GrindstoneOutput, index - 2usize))
            }
            (Window::Grindstone { grindstone, player }, 3usize..=29usize) => {
                Some((player, Area::Storage, index - 3usize))
            }
            (Window::Grindstone { grindstone, player }, 30usize..=38usize) => {
                Some((player, Area::Hotbar, index - 30usize))
            }
            (Window::Lectern { lectern, player }, 0usize..=0usize) => {
                Some((lectern, Area::LecternBook, index - 0usize))
            }
            (Window::Lectern { lectern, player }, 1usize..=27usize) => {
                Some((player, Area::Storage, index - 1usize))
            }
            (Window::Lectern { lectern, player }, 28usize..=36usize) => {
                Some((player, Area::Hotbar, index - 28usize))
            }
            (Window::Loom { loom, player }, 0usize..=0usize) => {
                Some((loom, Area::LoomBanner, index - 0usize))
            }
            (Window::Loom { loom, player }, 1usize..=1usize) => {
                Some((loom, Area::LoomDye, index - 1usize))
            }
            (Window::Loom { loom, player }, 2usize..=2usize) => {
                Some((loom, Area::LoomPattern, index - 2usize))
            }
            (Window::Loom { loom, player }, 3usize..=3usize) => {
                Some((loom, Area::LoomOutput, index - 3usize))
            }
            (Window::Loom { loom, player }, 4usize..=30usize) => {
                Some((player, Area::Storage, index - 4usize))
            }
            (Window::Loom { loom, player }, 31usize..=39usize) => {
                Some((player, Area::Hotbar, index - 31usize))
            }
            (
                Window::Stonecutter {
                    stonecutter,
                    player,
                },
                0usize..=0usize,
            ) => Some((stonecutter, Area::StonecutterInput, index - 0usize)),
            (
                Window::Stonecutter {
                    stonecutter,
                    player,
                },
                1usize..=1usize,
            ) => Some((stonecutter, Area::StonecutterOutput, index - 1usize)),
            (
                Window::Stonecutter {
                    stonecutter,
                    player,
                },
                2usize..=28usize,
            ) => Some((player, Area::Storage, index - 2usize)),
            (
                Window::Stonecutter {
                    stonecutter,
                    player,
                },
                29usize..=37usize,
            ) => Some((player, Area::Hotbar, index - 29usize)),
            _ => None,
        }
    }
    pub fn slot_to_index(
        &self,
        inventory: &crate::Inventory,
        area: Area,
        slot: usize,
    ) -> Option<usize> {
        match (self, area) {
            (Window::Player { player }, Area::CraftingOutput) if player.ptr_eq(inventory) => {
                Some(slot + 0usize)
            }
            (Window::Player { player }, Area::CraftingInput) if player.ptr_eq(inventory) => {
                Some(slot + 1usize)
            }
            (Window::Player { player }, Area::Helmet) if player.ptr_eq(inventory) => {
                Some(slot + 5usize)
            }
            (Window::Player { player }, Area::Chestplate) if player.ptr_eq(inventory) => {
                Some(slot + 6usize)
            }
            (Window::Player { player }, Area::Leggings) if player.ptr_eq(inventory) => {
                Some(slot + 7usize)
            }
            (Window::Player { player }, Area::Boots) if player.ptr_eq(inventory) => {
                Some(slot + 8usize)
            }
            (Window::Player { player }, Area::Storage) if player.ptr_eq(inventory) => {
                Some(slot + 9usize)
            }
            (Window::Player { player }, Area::Hotbar) if player.ptr_eq(inventory) => {
                Some(slot + 36usize)
            }
            (Window::Player { player }, Area::Offhand) if player.ptr_eq(inventory) => {
                Some(slot + 45usize)
            }
            (Window::Generic9X1 { block, player }, Area::Storage) if block.ptr_eq(inventory) => {
                Some(slot + 0usize)
            }
            (Window::Generic9X1 { block, player }, Area::Storage) if player.ptr_eq(inventory) => {
                Some(slot + 9usize)
            }
            (Window::Generic9X1 { block, player }, Area::Hotbar) if player.ptr_eq(inventory) => {
                Some(slot + 36usize)
            }
            (Window::Generic9X2 { block, player }, Area::Storage) if block.ptr_eq(inventory) => {
                Some(slot + 0usize)
            }
            (Window::Generic9X2 { block, player }, Area::Storage) if player.ptr_eq(inventory) => {
                Some(slot + 18usize)
            }
            (Window::Generic9X2 { block, player }, Area::Hotbar) if player.ptr_eq(inventory) => {
                Some(slot + 45usize)
            }
            (Window::Generic9X3 { block, player }, Area::Storage) if block.ptr_eq(inventory) => {
                Some(slot + 0usize)
            }
            (Window::Generic9X3 { block, player }, Area::Storage) if player.ptr_eq(inventory) => {
                Some(slot + 27usize)
            }
            (Window::Generic9X3 { block, player }, Area::Hotbar) if player.ptr_eq(inventory) => {
                Some(slot + 54usize)
            }
            (Window::Generic9X4 { block, player }, Area::Storage) if block.ptr_eq(inventory) => {
                Some(slot + 0usize)
            }
            (Window::Generic9X4 { block, player }, Area::Storage) if player.ptr_eq(inventory) => {
                Some(slot + 36usize)
            }
            (Window::Generic9X4 { block, player }, Area::Hotbar) if player.ptr_eq(inventory) => {
                Some(slot + 63usize)
            }
            (Window::Generic9X5 { block, player }, Area::Storage) if block.ptr_eq(inventory) => {
                Some(slot + 0usize)
            }
            (Window::Generic9X5 { block, player }, Area::Storage) if player.ptr_eq(inventory) => {
                Some(slot + 45usize)
            }
            (Window::Generic9X5 { block, player }, Area::Hotbar) if player.ptr_eq(inventory) => {
                Some(slot + 72usize)
            }
            (
                Window::Generic9X6 {
                    left_chest,
                    right_chest,
                    player,
                },
                Area::Storage,
            ) if left_chest.ptr_eq(inventory) => Some(slot + 0usize),
            (
                Window::Generic9X6 {
                    left_chest,
                    right_chest,
                    player,
                },
                Area::Storage,
            ) if right_chest.ptr_eq(inventory) => Some(slot + 27usize),
            (
                Window::Generic9X6 {
                    left_chest,
                    right_chest,
                    player,
                },
                Area::Storage,
            ) if player.ptr_eq(inventory) => Some(slot + 54usize),
            (
                Window::Generic9X6 {
                    left_chest,
                    right_chest,
                    player,
                },
                Area::Hotbar,
            ) if player.ptr_eq(inventory) => Some(slot + 81usize),
            (Window::Generic3X3 { block, player }, Area::Storage) if block.ptr_eq(inventory) => {
                Some(slot + 0usize)
            }
            (Window::Generic3X3 { block, player }, Area::Storage) if player.ptr_eq(inventory) => {
                Some(slot + 9usize)
            }
            (Window::Generic3X3 { block, player }, Area::Hotbar) if player.ptr_eq(inventory) => {
                Some(slot + 36usize)
            }
            (
                Window::Crafting {
                    crafting_table,
                    player,
                },
                Area::CraftingOutput,
            ) if crafting_table.ptr_eq(inventory) => Some(slot + 0usize),
            (
                Window::Crafting {
                    crafting_table,
                    player,
                },
                Area::CraftingInput,
            ) if crafting_table.ptr_eq(inventory) => Some(slot + 1usize),
            (
                Window::Crafting {
                    crafting_table,
                    player,
                },
                Area::Storage,
            ) if player.ptr_eq(inventory) => Some(slot + 10usize),
            (
                Window::Crafting {
                    crafting_table,
                    player,
                },
                Area::Hotbar,
            ) if player.ptr_eq(inventory) => Some(slot + 37usize),
            (Window::Furnace { furnace, player }, Area::FurnaceIngredient)
                if furnace.ptr_eq(inventory) =>
            {
                Some(slot + 0usize)
            }
            (Window::Furnace { furnace, player }, Area::FurnaceFuel)
                if furnace.ptr_eq(inventory) =>
            {
                Some(slot + 1usize)
            }
            (Window::Furnace { furnace, player }, Area::FurnaceOutput)
                if furnace.ptr_eq(inventory) =>
            {
                Some(slot + 2usize)
            }
            (Window::Furnace { furnace, player }, Area::Storage) if player.ptr_eq(inventory) => {
                Some(slot + 3usize)
            }
            (Window::Furnace { furnace, player }, Area::Hotbar) if player.ptr_eq(inventory) => {
                Some(slot + 30usize)
            }
            (
                Window::BlastFurnace {
                    blast_furnace,
                    player,
                },
                Area::FurnaceIngredient,
            ) if blast_furnace.ptr_eq(inventory) => Some(slot + 0usize),
            (
                Window::BlastFurnace {
                    blast_furnace,
                    player,
                },
                Area::FurnaceFuel,
            ) if blast_furnace.ptr_eq(inventory) => Some(slot + 1usize),
            (
                Window::BlastFurnace {
                    blast_furnace,
                    player,
                },
                Area::FurnaceOutput,
            ) if blast_furnace.ptr_eq(inventory) => Some(slot + 2usize),
            (
                Window::BlastFurnace {
                    blast_furnace,
                    player,
                },
                Area::Storage,
            ) if player.ptr_eq(inventory) => Some(slot + 3usize),
            (
                Window::BlastFurnace {
                    blast_furnace,
                    player,
                },
                Area::Hotbar,
            ) if player.ptr_eq(inventory) => Some(slot + 30usize),
            (Window::Smoker { smoker, player }, Area::FurnaceIngredient)
                if smoker.ptr_eq(inventory) =>
            {
                Some(slot + 0usize)
            }
            (Window::Smoker { smoker, player }, Area::FurnaceFuel) if smoker.ptr_eq(inventory) => {
                Some(slot + 1usize)
            }
            (Window::Smoker { smoker, player }, Area::FurnaceOutput)
                if smoker.ptr_eq(inventory) =>
            {
                Some(slot + 2usize)
            }
            (Window::Smoker { smoker, player }, Area::Storage) if player.ptr_eq(inventory) => {
                Some(slot + 3usize)
            }
            (Window::Smoker { smoker, player }, Area::Hotbar) if player.ptr_eq(inventory) => {
                Some(slot + 30usize)
            }
            (
                Window::Enchantment {
                    enchantment_table,
                    player,
                },
                Area::EnchantmentItem,
            ) if enchantment_table.ptr_eq(inventory) => Some(slot + 0usize),
            (
                Window::Enchantment {
                    enchantment_table,
                    player,
                },
                Area::EnchantmentLapis,
            ) if enchantment_table.ptr_eq(inventory) => Some(slot + 1usize),
            (
                Window::Enchantment {
                    enchantment_table,
                    player,
                },
                Area::Storage,
            ) if player.ptr_eq(inventory) => Some(slot + 2usize),
            (
                Window::Enchantment {
                    enchantment_table,
                    player,
                },
                Area::Hotbar,
            ) if player.ptr_eq(inventory) => Some(slot + 29usize),
            (
                Window::BrewingStand {
                    brewing_stand,
                    player,
                },
                Area::BrewingBottle,
            ) if brewing_stand.ptr_eq(inventory) => Some(slot + 0usize),
            (
                Window::BrewingStand {
                    brewing_stand,
                    player,
                },
                Area::BrewingIngredient,
            ) if brewing_stand.ptr_eq(inventory) => Some(slot + 3usize),
            (
                Window::BrewingStand {
                    brewing_stand,
                    player,
                },
                Area::BrewingBlazePowder,
            ) if brewing_stand.ptr_eq(inventory) => Some(slot + 4usize),
            (
                Window::BrewingStand {
                    brewing_stand,
                    player,
                },
                Area::Storage,
            ) if player.ptr_eq(inventory) => Some(slot + 5usize),
            (
                Window::BrewingStand {
                    brewing_stand,
                    player,
                },
                Area::Hotbar,
            ) if player.ptr_eq(inventory) => Some(slot + 32usize),
            (Window::Beacon { beacon, player }, Area::BeaconPayment)
                if beacon.ptr_eq(inventory) =>
            {
                Some(slot + 0usize)
            }
            (Window::Beacon { beacon, player }, Area::Storage) if player.ptr_eq(inventory) => {
                Some(slot + 1usize)
            }
            (Window::Beacon { beacon, player }, Area::Hotbar) if player.ptr_eq(inventory) => {
                Some(slot + 28usize)
            }
            (Window::Anvil { anvil, player }, Area::AnvilInput1) if anvil.ptr_eq(inventory) => {
                Some(slot + 0usize)
            }
            (Window::Anvil { anvil, player }, Area::AnvilInput2) if anvil.ptr_eq(inventory) => {
                Some(slot + 1usize)
            }
            (Window::Anvil { anvil, player }, Area::AnvilOutput) if anvil.ptr_eq(inventory) => {
                Some(slot + 2usize)
            }
            (Window::Anvil { anvil, player }, Area::Storage) if player.ptr_eq(inventory) => {
                Some(slot + 3usize)
            }
            (Window::Anvil { anvil, player }, Area::Hotbar) if player.ptr_eq(inventory) => {
                Some(slot + 30usize)
            }
            (Window::Hopper { hopper, player }, Area::Storage) if hopper.ptr_eq(inventory) => {
                Some(slot + 0usize)
            }
            (Window::Hopper { hopper, player }, Area::Storage) if player.ptr_eq(inventory) => {
                Some(slot + 4usize)
            }
            (Window::Hopper { hopper, player }, Area::Hotbar) if player.ptr_eq(inventory) => {
                Some(slot + 31usize)
            }
            (
                Window::ShulkerBox {
                    shulker_box,
                    player,
                },
                Area::Storage,
            ) if shulker_box.ptr_eq(inventory) => Some(slot + 0usize),
            (
                Window::ShulkerBox {
                    shulker_box,
                    player,
                },
                Area::Storage,
            ) if player.ptr_eq(inventory) => Some(slot + 27usize),
            (
                Window::ShulkerBox {
                    shulker_box,
                    player,
                },
                Area::Hotbar,
            ) if player.ptr_eq(inventory) => Some(slot + 54usize),
            (
                Window::Cartography {
                    cartography_table,
                    player,
                },
                Area::CartographyMap,
            ) if cartography_table.ptr_eq(inventory) => Some(slot + 0usize),
            (
                Window::Cartography {
                    cartography_table,
                    player,
                },
                Area::CartographyPaper,
            ) if cartography_table.ptr_eq(inventory) => Some(slot + 1usize),
            (
                Window::Cartography {
                    cartography_table,
                    player,
                },
                Area::CartographyOutput,
            ) if cartography_table.ptr_eq(inventory) => Some(slot + 2usize),
            (
                Window::Cartography {
                    cartography_table,
                    player,
                },
                Area::Storage,
            ) if player.ptr_eq(inventory) => Some(slot + 3usize),
            (
                Window::Cartography {
                    cartography_table,
                    player,
                },
                Area::Hotbar,
            ) if player.ptr_eq(inventory) => Some(slot + 30usize),
            (Window::Grindstone { grindstone, player }, Area::GrindstoneInput1)
                if grindstone.ptr_eq(inventory) =>
            {
                Some(slot + 0usize)
            }
            (Window::Grindstone { grindstone, player }, Area::GrindstoneInput2)
                if grindstone.ptr_eq(inventory) =>
            {
                Some(slot + 1usize)
            }
            (Window::Grindstone { grindstone, player }, Area::GrindstoneOutput)
                if grindstone.ptr_eq(inventory) =>
            {
                Some(slot + 2usize)
            }
            (Window::Grindstone { grindstone, player }, Area::Storage)
                if player.ptr_eq(inventory) =>
            {
                Some(slot + 3usize)
            }
            (Window::Grindstone { grindstone, player }, Area::Hotbar)
                if player.ptr_eq(inventory) =>
            {
                Some(slot + 30usize)
            }
            (Window::Lectern { lectern, player }, Area::LecternBook)
                if lectern.ptr_eq(inventory) =>
            {
                Some(slot + 0usize)
            }
            (Window::Lectern { lectern, player }, Area::Storage) if player.ptr_eq(inventory) => {
                Some(slot + 1usize)
            }
            (Window::Lectern { lectern, player }, Area::Hotbar) if player.ptr_eq(inventory) => {
                Some(slot + 28usize)
            }
            (Window::Loom { loom, player }, Area::LoomBanner) if loom.ptr_eq(inventory) => {
                Some(slot + 0usize)
            }
            (Window::Loom { loom, player }, Area::LoomDye) if loom.ptr_eq(inventory) => {
                Some(slot + 1usize)
            }
            (Window::Loom { loom, player }, Area::LoomPattern) if loom.ptr_eq(inventory) => {
                Some(slot + 2usize)
            }
            (Window::Loom { loom, player }, Area::LoomOutput) if loom.ptr_eq(inventory) => {
                Some(slot + 3usize)
            }
            (Window::Loom { loom, player }, Area::Storage) if player.ptr_eq(inventory) => {
                Some(slot + 4usize)
            }
            (Window::Loom { loom, player }, Area::Hotbar) if player.ptr_eq(inventory) => {
                Some(slot + 31usize)
            }
            (
                Window::Stonecutter {
                    stonecutter,
                    player,
                },
                Area::StonecutterInput,
            ) if stonecutter.ptr_eq(inventory) => Some(slot + 0usize),
            (
                Window::Stonecutter {
                    stonecutter,
                    player,
                },
                Area::StonecutterOutput,
            ) if stonecutter.ptr_eq(inventory) => Some(slot + 1usize),
            (
                Window::Stonecutter {
                    stonecutter,
                    player,
                },
                Area::Storage,
            ) if player.ptr_eq(inventory) => Some(slot + 2usize),
            (
                Window::Stonecutter {
                    stonecutter,
                    player,
                },
                Area::Hotbar,
            ) if player.ptr_eq(inventory) => Some(slot + 29usize),
            _ => None,
        }
    }
}
#[derive(Debug, Clone)]
pub enum InventoryBacking<T> {
    Player {
        crafting_input: [T; 4usize],
        crafting_output: [T; 1usize],
        helmet: [T; 1usize],
        chestplate: [T; 1usize],
        leggings: [T; 1usize],
        boots: [T; 1usize],
        storage: [T; 27usize],
        hotbar: [T; 9usize],
        offhand: [T; 1usize],
    },
    Chest {
        storage: [T; 27usize],
    },
    CraftingTable {
        crafting_input: [T; 9usize],
        crafting_output: [T; 1usize],
    },
    Furnace {
        furnace_ingredient: [T; 1usize],
        furnace_fuel: [T; 1usize],
        furnace_output: [T; 1usize],
    },
}
impl<T> InventoryBacking<T> {
    pub fn player() -> Self
    where
        T: Default,
    {
        InventoryBacking::Player {
            crafting_input: Default::default(),
            crafting_output: Default::default(),
            helmet: Default::default(),
            chestplate: Default::default(),
            leggings: Default::default(),
            boots: Default::default(),
            storage: Default::default(),
            hotbar: Default::default(),
            offhand: Default::default(),
        }
    }
    pub fn chest() -> Self
    where
        T: Default,
    {
        InventoryBacking::Chest {
            storage: Default::default(),
        }
    }
    pub fn crafting_table() -> Self
    where
        T: Default,
    {
        InventoryBacking::CraftingTable {
            crafting_input: Default::default(),
            crafting_output: Default::default(),
        }
    }
    pub fn furnace() -> Self
    where
        T: Default,
    {
        InventoryBacking::Furnace {
            furnace_ingredient: Default::default(),
            furnace_fuel: Default::default(),
            furnace_output: Default::default(),
        }
    }
    pub fn area_slice(&self, area: Area) -> Option<&[T]> {
        match (self, area) {
            (
                InventoryBacking::Player {
                    crafting_input,
                    crafting_output,
                    helmet,
                    chestplate,
                    leggings,
                    boots,
                    storage,
                    hotbar,
                    offhand,
                },
                Area::CraftingInput,
            ) => Some(crafting_input),
            (
                InventoryBacking::Player {
                    crafting_input,
                    crafting_output,
                    helmet,
                    chestplate,
                    leggings,
                    boots,
                    storage,
                    hotbar,
                    offhand,
                },
                Area::CraftingOutput,
            ) => Some(crafting_output),
            (
                InventoryBacking::Player {
                    crafting_input,
                    crafting_output,
                    helmet,
                    chestplate,
                    leggings,
                    boots,
                    storage,
                    hotbar,
                    offhand,
                },
                Area::Helmet,
            ) => Some(helmet),
            (
                InventoryBacking::Player {
                    crafting_input,
                    crafting_output,
                    helmet,
                    chestplate,
                    leggings,
                    boots,
                    storage,
                    hotbar,
                    offhand,
                },
                Area::Chestplate,
            ) => Some(chestplate),
            (
                InventoryBacking::Player {
                    crafting_input,
                    crafting_output,
                    helmet,
                    chestplate,
                    leggings,
                    boots,
                    storage,
                    hotbar,
                    offhand,
                },
                Area::Leggings,
            ) => Some(leggings),
            (
                InventoryBacking::Player {
                    crafting_input,
                    crafting_output,
                    helmet,
                    chestplate,
                    leggings,
                    boots,
                    storage,
                    hotbar,
                    offhand,
                },
                Area::Boots,
            ) => Some(boots),
            (
                InventoryBacking::Player {
                    crafting_input,
                    crafting_output,
                    helmet,
                    chestplate,
                    leggings,
                    boots,
                    storage,
                    hotbar,
                    offhand,
                },
                Area::Storage,
            ) => Some(storage),
            (
                InventoryBacking::Player {
                    crafting_input,
                    crafting_output,
                    helmet,
                    chestplate,
                    leggings,
                    boots,
                    storage,
                    hotbar,
                    offhand,
                },
                Area::Hotbar,
            ) => Some(hotbar),
            (
                InventoryBacking::Player {
                    crafting_input,
                    crafting_output,
                    helmet,
                    chestplate,
                    leggings,
                    boots,
                    storage,
                    hotbar,
                    offhand,
                },
                Area::Offhand,
            ) => Some(offhand),
            (InventoryBacking::Chest { storage }, Area::Storage) => Some(storage),
            (
                InventoryBacking::CraftingTable {
                    crafting_input,
                    crafting_output,
                },
                Area::CraftingInput,
            ) => Some(crafting_input),
            (
                InventoryBacking::CraftingTable {
                    crafting_input,
                    crafting_output,
                },
                Area::CraftingOutput,
            ) => Some(crafting_output),
            (
                InventoryBacking::Furnace {
                    furnace_ingredient,
                    furnace_fuel,
                    furnace_output,
                },
                Area::FurnaceIngredient,
            ) => Some(furnace_ingredient),
            (
                InventoryBacking::Furnace {
                    furnace_ingredient,
                    furnace_fuel,
                    furnace_output,
                },
                Area::FurnaceFuel,
            ) => Some(furnace_fuel),
            (
                InventoryBacking::Furnace {
                    furnace_ingredient,
                    furnace_fuel,
                    furnace_output,
                },
                Area::FurnaceOutput,
            ) => Some(furnace_output),
            _ => None,
        }
    }
    pub fn areas(&self) -> &'static [Area] {
        match self {
            InventoryBacking::Player { .. } => &[
                Area::CraftingInput,
                Area::CraftingOutput,
                Area::Helmet,
                Area::Chestplate,
                Area::Leggings,
                Area::Boots,
                Area::Storage,
                Area::Hotbar,
                Area::Offhand,
            ],
            InventoryBacking::Chest { .. } => &[Area::Storage],
            InventoryBacking::CraftingTable { .. } => &[Area::CraftingInput, Area::CraftingOutput],
            InventoryBacking::Furnace { .. } => &[
                Area::FurnaceIngredient,
                Area::FurnaceFuel,
                Area::FurnaceOutput,
            ],
        }
    }
}
impl crate::Inventory {
    pub fn player() -> Self {
        Self {
            inner: std::rc::Rc::new(crate::Inner {
                backing: InventoryBacking::player(),
                slot_mutated_callback: std::cell::RefCell::new(None),
            }),
        }
    }
    pub fn chest() -> Self {
        Self {
            inner: std::rc::Rc::new(crate::Inner {
                backing: InventoryBacking::chest(),
                slot_mutated_callback: std::cell::RefCell::new(None),
            }),
        }
    }
    pub fn crafting_table() -> Self {
        Self {
            inner: std::rc::Rc::new(crate::Inner {
                backing: InventoryBacking::crafting_table(),
                slot_mutated_callback: std::cell::RefCell::new(None),
            }),
        }
    }
    pub fn furnace() -> Self {
        Self {
            inner: std::rc::Rc::new(crate::Inner {
                backing: InventoryBacking::furnace(),
                slot_mutated_callback: std::cell::RefCell::new(None),
            }),
        }
    }
}
