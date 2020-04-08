macro_rules! blocks_store {
    () => {
        #[derive(Debug)]
        pub struct BlocksStore {
            air_base: crate::BlockId,
            stone_base: crate::BlockId,
            granite_base: crate::BlockId,
            polished_granite_base: crate::BlockId,
            diorite_base: crate::BlockId,
            polished_diorite_base: crate::BlockId,
            andesite_base: crate::BlockId,
            polished_andesite_base: crate::BlockId,
            grass_block_base: crate::BlockId,
            dirt_base: crate::BlockId,
            coarse_dirt_base: crate::BlockId,
            podzol_base: crate::BlockId,
            cobblestone_base: crate::BlockId,
            oak_planks_base: crate::BlockId,
            spruce_planks_base: crate::BlockId,
            birch_planks_base: crate::BlockId,
            jungle_planks_base: crate::BlockId,
            acacia_planks_base: crate::BlockId,
            dark_oak_planks_base: crate::BlockId,
            oak_sapling_base: crate::BlockId,
            spruce_sapling_base: crate::BlockId,
            birch_sapling_base: crate::BlockId,
            jungle_sapling_base: crate::BlockId,
            acacia_sapling_base: crate::BlockId,
            dark_oak_sapling_base: crate::BlockId,
            bedrock_base: crate::BlockId,
            water_base: crate::BlockId,
            lava_base: crate::BlockId,
            sand_base: crate::BlockId,
            red_sand_base: crate::BlockId,
            gravel_base: crate::BlockId,
            gold_ore_base: crate::BlockId,
            iron_ore_base: crate::BlockId,
            coal_ore_base: crate::BlockId,
            oak_log_base: crate::BlockId,
            spruce_log_base: crate::BlockId,
            birch_log_base: crate::BlockId,
            jungle_log_base: crate::BlockId,
            acacia_log_base: crate::BlockId,
            dark_oak_log_base: crate::BlockId,
            stripped_spruce_log_base: crate::BlockId,
            stripped_birch_log_base: crate::BlockId,
            stripped_jungle_log_base: crate::BlockId,
            stripped_acacia_log_base: crate::BlockId,
            stripped_dark_oak_log_base: crate::BlockId,
            stripped_oak_log_base: crate::BlockId,
            oak_wood_base: crate::BlockId,
            spruce_wood_base: crate::BlockId,
            birch_wood_base: crate::BlockId,
            jungle_wood_base: crate::BlockId,
            acacia_wood_base: crate::BlockId,
            dark_oak_wood_base: crate::BlockId,
            stripped_oak_wood_base: crate::BlockId,
            stripped_spruce_wood_base: crate::BlockId,
            stripped_birch_wood_base: crate::BlockId,
            stripped_jungle_wood_base: crate::BlockId,
            stripped_acacia_wood_base: crate::BlockId,
            stripped_dark_oak_wood_base: crate::BlockId,
            oak_leaves_base: crate::BlockId,
            spruce_leaves_base: crate::BlockId,
            birch_leaves_base: crate::BlockId,
            jungle_leaves_base: crate::BlockId,
            acacia_leaves_base: crate::BlockId,
            dark_oak_leaves_base: crate::BlockId,
            sponge_base: crate::BlockId,
            wet_sponge_base: crate::BlockId,
            glass_base: crate::BlockId,
            lapis_ore_base: crate::BlockId,
            lapis_block_base: crate::BlockId,
            dispenser_base: crate::BlockId,
            sandstone_base: crate::BlockId,
            chiseled_sandstone_base: crate::BlockId,
            cut_sandstone_base: crate::BlockId,
            note_block_base: crate::BlockId,
            white_bed_base: crate::BlockId,
            orange_bed_base: crate::BlockId,
            magenta_bed_base: crate::BlockId,
            light_blue_bed_base: crate::BlockId,
            yellow_bed_base: crate::BlockId,
            lime_bed_base: crate::BlockId,
            pink_bed_base: crate::BlockId,
            gray_bed_base: crate::BlockId,
            light_gray_bed_base: crate::BlockId,
            cyan_bed_base: crate::BlockId,
            purple_bed_base: crate::BlockId,
            blue_bed_base: crate::BlockId,
            brown_bed_base: crate::BlockId,
            green_bed_base: crate::BlockId,
            red_bed_base: crate::BlockId,
            black_bed_base: crate::BlockId,
            powered_rail_base: crate::BlockId,
            detector_rail_base: crate::BlockId,
            sticky_piston_base: crate::BlockId,
            cobweb_base: crate::BlockId,
            grass_base: crate::BlockId,
            fern_base: crate::BlockId,
            dead_bush_base: crate::BlockId,
            seagrass_base: crate::BlockId,
            tall_seagrass_base: crate::BlockId,
            piston_base: crate::BlockId,
            piston_head_base: crate::BlockId,
            white_wool_base: crate::BlockId,
            orange_wool_base: crate::BlockId,
            magenta_wool_base: crate::BlockId,
            light_blue_wool_base: crate::BlockId,
            yellow_wool_base: crate::BlockId,
            lime_wool_base: crate::BlockId,
            pink_wool_base: crate::BlockId,
            gray_wool_base: crate::BlockId,
            light_gray_wool_base: crate::BlockId,
            cyan_wool_base: crate::BlockId,
            purple_wool_base: crate::BlockId,
            blue_wool_base: crate::BlockId,
            brown_wool_base: crate::BlockId,
            green_wool_base: crate::BlockId,
            red_wool_base: crate::BlockId,
            black_wool_base: crate::BlockId,
            moving_piston_base: crate::BlockId,
            dandelion_base: crate::BlockId,
            poppy_base: crate::BlockId,
            blue_orchid_base: crate::BlockId,
            allium_base: crate::BlockId,
            azure_bluet_base: crate::BlockId,
            red_tulip_base: crate::BlockId,
            orange_tulip_base: crate::BlockId,
            white_tulip_base: crate::BlockId,
            pink_tulip_base: crate::BlockId,
            oxeye_daisy_base: crate::BlockId,
            brown_mushroom_base: crate::BlockId,
            red_mushroom_base: crate::BlockId,
            gold_block_base: crate::BlockId,
            iron_block_base: crate::BlockId,
            bricks_base: crate::BlockId,
            tnt_base: crate::BlockId,
            bookshelf_base: crate::BlockId,
            mossy_cobblestone_base: crate::BlockId,
            obsidian_base: crate::BlockId,
            torch_base: crate::BlockId,
            wall_torch_base: crate::BlockId,
            fire_base: crate::BlockId,
            spawner_base: crate::BlockId,
            oak_stairs_base: crate::BlockId,
            chest_base: crate::BlockId,
            redstone_wire_base: crate::BlockId,
            diamond_ore_base: crate::BlockId,
            diamond_block_base: crate::BlockId,
            crafting_table_base: crate::BlockId,
            wheat_base: crate::BlockId,
            farmland_base: crate::BlockId,
            furnace_base: crate::BlockId,
            sign_base: crate::BlockId,
            oak_door_base: crate::BlockId,
            ladder_base: crate::BlockId,
            rail_base: crate::BlockId,
            cobblestone_stairs_base: crate::BlockId,
            wall_sign_base: crate::BlockId,
            lever_base: crate::BlockId,
            stone_pressure_plate_base: crate::BlockId,
            iron_door_base: crate::BlockId,
            oak_pressure_plate_base: crate::BlockId,
            spruce_pressure_plate_base: crate::BlockId,
            birch_pressure_plate_base: crate::BlockId,
            jungle_pressure_plate_base: crate::BlockId,
            acacia_pressure_plate_base: crate::BlockId,
            dark_oak_pressure_plate_base: crate::BlockId,
            redstone_ore_base: crate::BlockId,
            redstone_torch_base: crate::BlockId,
            redstone_wall_torch_base: crate::BlockId,
            stone_button_base: crate::BlockId,
            snow_base: crate::BlockId,
            ice_base: crate::BlockId,
            snow_block_base: crate::BlockId,
            cactus_base: crate::BlockId,
            clay_base: crate::BlockId,
            sugar_cane_base: crate::BlockId,
            jukebox_base: crate::BlockId,
            oak_fence_base: crate::BlockId,
            pumpkin_base: crate::BlockId,
            netherrack_base: crate::BlockId,
            soul_sand_base: crate::BlockId,
            glowstone_base: crate::BlockId,
            nether_portal_base: crate::BlockId,
            carved_pumpkin_base: crate::BlockId,
            jack_o_lantern_base: crate::BlockId,
            cake_base: crate::BlockId,
            repeater_base: crate::BlockId,
            white_stained_glass_base: crate::BlockId,
            orange_stained_glass_base: crate::BlockId,
            magenta_stained_glass_base: crate::BlockId,
            light_blue_stained_glass_base: crate::BlockId,
            yellow_stained_glass_base: crate::BlockId,
            lime_stained_glass_base: crate::BlockId,
            pink_stained_glass_base: crate::BlockId,
            gray_stained_glass_base: crate::BlockId,
            light_gray_stained_glass_base: crate::BlockId,
            cyan_stained_glass_base: crate::BlockId,
            purple_stained_glass_base: crate::BlockId,
            blue_stained_glass_base: crate::BlockId,
            brown_stained_glass_base: crate::BlockId,
            green_stained_glass_base: crate::BlockId,
            red_stained_glass_base: crate::BlockId,
            black_stained_glass_base: crate::BlockId,
            oak_trapdoor_base: crate::BlockId,
            spruce_trapdoor_base: crate::BlockId,
            birch_trapdoor_base: crate::BlockId,
            jungle_trapdoor_base: crate::BlockId,
            acacia_trapdoor_base: crate::BlockId,
            dark_oak_trapdoor_base: crate::BlockId,
            infested_stone_base: crate::BlockId,
            infested_cobblestone_base: crate::BlockId,
            infested_stone_bricks_base: crate::BlockId,
            infested_mossy_stone_bricks_base: crate::BlockId,
            infested_cracked_stone_bricks_base: crate::BlockId,
            infested_chiseled_stone_bricks_base: crate::BlockId,
            stone_bricks_base: crate::BlockId,
            mossy_stone_bricks_base: crate::BlockId,
            cracked_stone_bricks_base: crate::BlockId,
            chiseled_stone_bricks_base: crate::BlockId,
            brown_mushroom_block_base: crate::BlockId,
            red_mushroom_block_base: crate::BlockId,
            mushroom_stem_base: crate::BlockId,
            iron_bars_base: crate::BlockId,
            glass_pane_base: crate::BlockId,
            melon_base: crate::BlockId,
            attached_pumpkin_stem_base: crate::BlockId,
            attached_melon_stem_base: crate::BlockId,
            pumpkin_stem_base: crate::BlockId,
            melon_stem_base: crate::BlockId,
            vine_base: crate::BlockId,
            oak_fence_gate_base: crate::BlockId,
            brick_stairs_base: crate::BlockId,
            stone_brick_stairs_base: crate::BlockId,
            mycelium_base: crate::BlockId,
            lily_pad_base: crate::BlockId,
            nether_bricks_base: crate::BlockId,
            nether_brick_fence_base: crate::BlockId,
            nether_brick_stairs_base: crate::BlockId,
            nether_wart_base: crate::BlockId,
            enchanting_table_base: crate::BlockId,
            brewing_stand_base: crate::BlockId,
            cauldron_base: crate::BlockId,
            end_portal_base: crate::BlockId,
            end_portal_frame_base: crate::BlockId,
            end_stone_base: crate::BlockId,
            dragon_egg_base: crate::BlockId,
            redstone_lamp_base: crate::BlockId,
            cocoa_base: crate::BlockId,
            sandstone_stairs_base: crate::BlockId,
            emerald_ore_base: crate::BlockId,
            ender_chest_base: crate::BlockId,
            tripwire_hook_base: crate::BlockId,
            tripwire_base: crate::BlockId,
            emerald_block_base: crate::BlockId,
            spruce_stairs_base: crate::BlockId,
            birch_stairs_base: crate::BlockId,
            jungle_stairs_base: crate::BlockId,
            command_block_base: crate::BlockId,
            beacon_base: crate::BlockId,
            cobblestone_wall_base: crate::BlockId,
            mossy_cobblestone_wall_base: crate::BlockId,
            flower_pot_base: crate::BlockId,
            potted_oak_sapling_base: crate::BlockId,
            potted_spruce_sapling_base: crate::BlockId,
            potted_birch_sapling_base: crate::BlockId,
            potted_jungle_sapling_base: crate::BlockId,
            potted_acacia_sapling_base: crate::BlockId,
            potted_dark_oak_sapling_base: crate::BlockId,
            potted_fern_base: crate::BlockId,
            potted_dandelion_base: crate::BlockId,
            potted_poppy_base: crate::BlockId,
            potted_blue_orchid_base: crate::BlockId,
            potted_allium_base: crate::BlockId,
            potted_azure_bluet_base: crate::BlockId,
            potted_red_tulip_base: crate::BlockId,
            potted_orange_tulip_base: crate::BlockId,
            potted_white_tulip_base: crate::BlockId,
            potted_pink_tulip_base: crate::BlockId,
            potted_oxeye_daisy_base: crate::BlockId,
            potted_red_mushroom_base: crate::BlockId,
            potted_brown_mushroom_base: crate::BlockId,
            potted_dead_bush_base: crate::BlockId,
            potted_cactus_base: crate::BlockId,
            carrots_base: crate::BlockId,
            potatoes_base: crate::BlockId,
            oak_button_base: crate::BlockId,
            spruce_button_base: crate::BlockId,
            birch_button_base: crate::BlockId,
            jungle_button_base: crate::BlockId,
            acacia_button_base: crate::BlockId,
            dark_oak_button_base: crate::BlockId,
            skeleton_wall_skull_base: crate::BlockId,
            skeleton_skull_base: crate::BlockId,
            wither_skeleton_wall_skull_base: crate::BlockId,
            wither_skeleton_skull_base: crate::BlockId,
            zombie_wall_head_base: crate::BlockId,
            zombie_head_base: crate::BlockId,
            player_wall_head_base: crate::BlockId,
            player_head_base: crate::BlockId,
            creeper_wall_head_base: crate::BlockId,
            creeper_head_base: crate::BlockId,
            dragon_wall_head_base: crate::BlockId,
            dragon_head_base: crate::BlockId,
            anvil_base: crate::BlockId,
            chipped_anvil_base: crate::BlockId,
            damaged_anvil_base: crate::BlockId,
            trapped_chest_base: crate::BlockId,
            light_weighted_pressure_plate_base: crate::BlockId,
            heavy_weighted_pressure_plate_base: crate::BlockId,
            comparator_base: crate::BlockId,
            daylight_detector_base: crate::BlockId,
            redstone_block_base: crate::BlockId,
            nether_quartz_ore_base: crate::BlockId,
            hopper_base: crate::BlockId,
            quartz_block_base: crate::BlockId,
            chiseled_quartz_block_base: crate::BlockId,
            quartz_pillar_base: crate::BlockId,
            quartz_stairs_base: crate::BlockId,
            activator_rail_base: crate::BlockId,
            dropper_base: crate::BlockId,
            white_terracotta_base: crate::BlockId,
            orange_terracotta_base: crate::BlockId,
            magenta_terracotta_base: crate::BlockId,
            light_blue_terracotta_base: crate::BlockId,
            yellow_terracotta_base: crate::BlockId,
            lime_terracotta_base: crate::BlockId,
            pink_terracotta_base: crate::BlockId,
            gray_terracotta_base: crate::BlockId,
            light_gray_terracotta_base: crate::BlockId,
            cyan_terracotta_base: crate::BlockId,
            purple_terracotta_base: crate::BlockId,
            blue_terracotta_base: crate::BlockId,
            brown_terracotta_base: crate::BlockId,
            green_terracotta_base: crate::BlockId,
            red_terracotta_base: crate::BlockId,
            black_terracotta_base: crate::BlockId,
            white_stained_glass_pane_base: crate::BlockId,
            orange_stained_glass_pane_base: crate::BlockId,
            magenta_stained_glass_pane_base: crate::BlockId,
            light_blue_stained_glass_pane_base: crate::BlockId,
            yellow_stained_glass_pane_base: crate::BlockId,
            lime_stained_glass_pane_base: crate::BlockId,
            pink_stained_glass_pane_base: crate::BlockId,
            gray_stained_glass_pane_base: crate::BlockId,
            light_gray_stained_glass_pane_base: crate::BlockId,
            cyan_stained_glass_pane_base: crate::BlockId,
            purple_stained_glass_pane_base: crate::BlockId,
            blue_stained_glass_pane_base: crate::BlockId,
            brown_stained_glass_pane_base: crate::BlockId,
            green_stained_glass_pane_base: crate::BlockId,
            red_stained_glass_pane_base: crate::BlockId,
            black_stained_glass_pane_base: crate::BlockId,
            acacia_stairs_base: crate::BlockId,
            dark_oak_stairs_base: crate::BlockId,
            slime_block_base: crate::BlockId,
            barrier_base: crate::BlockId,
            iron_trapdoor_base: crate::BlockId,
            prismarine_base: crate::BlockId,
            prismarine_bricks_base: crate::BlockId,
            dark_prismarine_base: crate::BlockId,
            prismarine_stairs_base: crate::BlockId,
            prismarine_brick_stairs_base: crate::BlockId,
            dark_prismarine_stairs_base: crate::BlockId,
            prismarine_slab_base: crate::BlockId,
            prismarine_brick_slab_base: crate::BlockId,
            dark_prismarine_slab_base: crate::BlockId,
            sea_lantern_base: crate::BlockId,
            hay_block_base: crate::BlockId,
            white_carpet_base: crate::BlockId,
            orange_carpet_base: crate::BlockId,
            magenta_carpet_base: crate::BlockId,
            light_blue_carpet_base: crate::BlockId,
            yellow_carpet_base: crate::BlockId,
            lime_carpet_base: crate::BlockId,
            pink_carpet_base: crate::BlockId,
            gray_carpet_base: crate::BlockId,
            light_gray_carpet_base: crate::BlockId,
            cyan_carpet_base: crate::BlockId,
            purple_carpet_base: crate::BlockId,
            blue_carpet_base: crate::BlockId,
            brown_carpet_base: crate::BlockId,
            green_carpet_base: crate::BlockId,
            red_carpet_base: crate::BlockId,
            black_carpet_base: crate::BlockId,
            terracotta_base: crate::BlockId,
            coal_block_base: crate::BlockId,
            packed_ice_base: crate::BlockId,
            sunflower_base: crate::BlockId,
            lilac_base: crate::BlockId,
            rose_bush_base: crate::BlockId,
            peony_base: crate::BlockId,
            tall_grass_base: crate::BlockId,
            large_fern_base: crate::BlockId,
            white_banner_base: crate::BlockId,
            orange_banner_base: crate::BlockId,
            magenta_banner_base: crate::BlockId,
            light_blue_banner_base: crate::BlockId,
            yellow_banner_base: crate::BlockId,
            lime_banner_base: crate::BlockId,
            pink_banner_base: crate::BlockId,
            gray_banner_base: crate::BlockId,
            light_gray_banner_base: crate::BlockId,
            cyan_banner_base: crate::BlockId,
            purple_banner_base: crate::BlockId,
            blue_banner_base: crate::BlockId,
            brown_banner_base: crate::BlockId,
            green_banner_base: crate::BlockId,
            red_banner_base: crate::BlockId,
            black_banner_base: crate::BlockId,
            white_wall_banner_base: crate::BlockId,
            orange_wall_banner_base: crate::BlockId,
            magenta_wall_banner_base: crate::BlockId,
            light_blue_wall_banner_base: crate::BlockId,
            yellow_wall_banner_base: crate::BlockId,
            lime_wall_banner_base: crate::BlockId,
            pink_wall_banner_base: crate::BlockId,
            gray_wall_banner_base: crate::BlockId,
            light_gray_wall_banner_base: crate::BlockId,
            cyan_wall_banner_base: crate::BlockId,
            purple_wall_banner_base: crate::BlockId,
            blue_wall_banner_base: crate::BlockId,
            brown_wall_banner_base: crate::BlockId,
            green_wall_banner_base: crate::BlockId,
            red_wall_banner_base: crate::BlockId,
            black_wall_banner_base: crate::BlockId,
            red_sandstone_base: crate::BlockId,
            chiseled_red_sandstone_base: crate::BlockId,
            cut_red_sandstone_base: crate::BlockId,
            red_sandstone_stairs_base: crate::BlockId,
            oak_slab_base: crate::BlockId,
            spruce_slab_base: crate::BlockId,
            birch_slab_base: crate::BlockId,
            jungle_slab_base: crate::BlockId,
            acacia_slab_base: crate::BlockId,
            dark_oak_slab_base: crate::BlockId,
            stone_slab_base: crate::BlockId,
            sandstone_slab_base: crate::BlockId,
            petrified_oak_slab_base: crate::BlockId,
            cobblestone_slab_base: crate::BlockId,
            brick_slab_base: crate::BlockId,
            stone_brick_slab_base: crate::BlockId,
            nether_brick_slab_base: crate::BlockId,
            quartz_slab_base: crate::BlockId,
            red_sandstone_slab_base: crate::BlockId,
            purpur_slab_base: crate::BlockId,
            smooth_stone_base: crate::BlockId,
            smooth_sandstone_base: crate::BlockId,
            smooth_quartz_base: crate::BlockId,
            smooth_red_sandstone_base: crate::BlockId,
            spruce_fence_gate_base: crate::BlockId,
            birch_fence_gate_base: crate::BlockId,
            jungle_fence_gate_base: crate::BlockId,
            acacia_fence_gate_base: crate::BlockId,
            dark_oak_fence_gate_base: crate::BlockId,
            spruce_fence_base: crate::BlockId,
            birch_fence_base: crate::BlockId,
            jungle_fence_base: crate::BlockId,
            acacia_fence_base: crate::BlockId,
            dark_oak_fence_base: crate::BlockId,
            spruce_door_base: crate::BlockId,
            birch_door_base: crate::BlockId,
            jungle_door_base: crate::BlockId,
            acacia_door_base: crate::BlockId,
            dark_oak_door_base: crate::BlockId,
            end_rod_base: crate::BlockId,
            chorus_plant_base: crate::BlockId,
            chorus_flower_base: crate::BlockId,
            purpur_block_base: crate::BlockId,
            purpur_pillar_base: crate::BlockId,
            purpur_stairs_base: crate::BlockId,
            end_stone_bricks_base: crate::BlockId,
            beetroots_base: crate::BlockId,
            grass_path_base: crate::BlockId,
            end_gateway_base: crate::BlockId,
            repeating_command_block_base: crate::BlockId,
            chain_command_block_base: crate::BlockId,
            frosted_ice_base: crate::BlockId,
            magma_block_base: crate::BlockId,
            nether_wart_block_base: crate::BlockId,
            red_nether_bricks_base: crate::BlockId,
            bone_block_base: crate::BlockId,
            structure_void_base: crate::BlockId,
            observer_base: crate::BlockId,
            shulker_box_base: crate::BlockId,
            white_shulker_box_base: crate::BlockId,
            orange_shulker_box_base: crate::BlockId,
            magenta_shulker_box_base: crate::BlockId,
            light_blue_shulker_box_base: crate::BlockId,
            yellow_shulker_box_base: crate::BlockId,
            lime_shulker_box_base: crate::BlockId,
            pink_shulker_box_base: crate::BlockId,
            gray_shulker_box_base: crate::BlockId,
            light_gray_shulker_box_base: crate::BlockId,
            cyan_shulker_box_base: crate::BlockId,
            purple_shulker_box_base: crate::BlockId,
            blue_shulker_box_base: crate::BlockId,
            brown_shulker_box_base: crate::BlockId,
            green_shulker_box_base: crate::BlockId,
            red_shulker_box_base: crate::BlockId,
            black_shulker_box_base: crate::BlockId,
            white_glazed_terracotta_base: crate::BlockId,
            orange_glazed_terracotta_base: crate::BlockId,
            magenta_glazed_terracotta_base: crate::BlockId,
            light_blue_glazed_terracotta_base: crate::BlockId,
            yellow_glazed_terracotta_base: crate::BlockId,
            lime_glazed_terracotta_base: crate::BlockId,
            pink_glazed_terracotta_base: crate::BlockId,
            gray_glazed_terracotta_base: crate::BlockId,
            light_gray_glazed_terracotta_base: crate::BlockId,
            cyan_glazed_terracotta_base: crate::BlockId,
            purple_glazed_terracotta_base: crate::BlockId,
            blue_glazed_terracotta_base: crate::BlockId,
            brown_glazed_terracotta_base: crate::BlockId,
            green_glazed_terracotta_base: crate::BlockId,
            red_glazed_terracotta_base: crate::BlockId,
            black_glazed_terracotta_base: crate::BlockId,
            white_concrete_base: crate::BlockId,
            orange_concrete_base: crate::BlockId,
            magenta_concrete_base: crate::BlockId,
            light_blue_concrete_base: crate::BlockId,
            yellow_concrete_base: crate::BlockId,
            lime_concrete_base: crate::BlockId,
            pink_concrete_base: crate::BlockId,
            gray_concrete_base: crate::BlockId,
            light_gray_concrete_base: crate::BlockId,
            cyan_concrete_base: crate::BlockId,
            purple_concrete_base: crate::BlockId,
            blue_concrete_base: crate::BlockId,
            brown_concrete_base: crate::BlockId,
            green_concrete_base: crate::BlockId,
            red_concrete_base: crate::BlockId,
            black_concrete_base: crate::BlockId,
            white_concrete_powder_base: crate::BlockId,
            orange_concrete_powder_base: crate::BlockId,
            magenta_concrete_powder_base: crate::BlockId,
            light_blue_concrete_powder_base: crate::BlockId,
            yellow_concrete_powder_base: crate::BlockId,
            lime_concrete_powder_base: crate::BlockId,
            pink_concrete_powder_base: crate::BlockId,
            gray_concrete_powder_base: crate::BlockId,
            light_gray_concrete_powder_base: crate::BlockId,
            cyan_concrete_powder_base: crate::BlockId,
            purple_concrete_powder_base: crate::BlockId,
            blue_concrete_powder_base: crate::BlockId,
            brown_concrete_powder_base: crate::BlockId,
            green_concrete_powder_base: crate::BlockId,
            red_concrete_powder_base: crate::BlockId,
            black_concrete_powder_base: crate::BlockId,
            kelp_base: crate::BlockId,
            kelp_plant_base: crate::BlockId,
            dried_kelp_block_base: crate::BlockId,
            turtle_egg_base: crate::BlockId,
            dead_tube_coral_block_base: crate::BlockId,
            dead_brain_coral_block_base: crate::BlockId,
            dead_bubble_coral_block_base: crate::BlockId,
            dead_fire_coral_block_base: crate::BlockId,
            dead_horn_coral_block_base: crate::BlockId,
            tube_coral_block_base: crate::BlockId,
            brain_coral_block_base: crate::BlockId,
            bubble_coral_block_base: crate::BlockId,
            fire_coral_block_base: crate::BlockId,
            horn_coral_block_base: crate::BlockId,
            dead_tube_coral_base: crate::BlockId,
            dead_brain_coral_base: crate::BlockId,
            dead_bubble_coral_base: crate::BlockId,
            dead_fire_coral_base: crate::BlockId,
            dead_horn_coral_base: crate::BlockId,
            tube_coral_base: crate::BlockId,
            brain_coral_base: crate::BlockId,
            bubble_coral_base: crate::BlockId,
            fire_coral_base: crate::BlockId,
            horn_coral_base: crate::BlockId,
            dead_tube_coral_wall_fan_base: crate::BlockId,
            dead_brain_coral_wall_fan_base: crate::BlockId,
            dead_bubble_coral_wall_fan_base: crate::BlockId,
            dead_fire_coral_wall_fan_base: crate::BlockId,
            dead_horn_coral_wall_fan_base: crate::BlockId,
            tube_coral_wall_fan_base: crate::BlockId,
            brain_coral_wall_fan_base: crate::BlockId,
            bubble_coral_wall_fan_base: crate::BlockId,
            fire_coral_wall_fan_base: crate::BlockId,
            horn_coral_wall_fan_base: crate::BlockId,
            dead_tube_coral_fan_base: crate::BlockId,
            dead_brain_coral_fan_base: crate::BlockId,
            dead_bubble_coral_fan_base: crate::BlockId,
            dead_fire_coral_fan_base: crate::BlockId,
            dead_horn_coral_fan_base: crate::BlockId,
            tube_coral_fan_base: crate::BlockId,
            brain_coral_fan_base: crate::BlockId,
            bubble_coral_fan_base: crate::BlockId,
            fire_coral_fan_base: crate::BlockId,
            horn_coral_fan_base: crate::BlockId,
            sea_pickle_base: crate::BlockId,
            blue_ice_base: crate::BlockId,
            conduit_base: crate::BlockId,
            void_air_base: crate::BlockId,
            cave_air_base: crate::BlockId,
            bubble_column_base: crate::BlockId,
            structure_block_base: crate::BlockId,
        }
        impl BlocksStore {
            pub fn air(&self) -> crate::BlockId {
                self.air_base
            }
            pub fn stone(&self) -> crate::BlockId {
                self.stone_base
            }
            pub fn granite(&self) -> crate::BlockId {
                self.granite_base
            }
            pub fn polished_granite(&self) -> crate::BlockId {
                self.polished_granite_base
            }
            pub fn diorite(&self) -> crate::BlockId {
                self.diorite_base
            }
            pub fn polished_diorite(&self) -> crate::BlockId {
                self.polished_diorite_base
            }
            pub fn andesite(&self) -> crate::BlockId {
                self.andesite_base
            }
            pub fn polished_andesite(&self) -> crate::BlockId {
                self.polished_andesite_base
            }
            pub fn grass_block(&self) -> crate::BlockId {
                self.grass_block_base
            }
            pub fn dirt(&self) -> crate::BlockId {
                self.dirt_base
            }
            pub fn coarse_dirt(&self) -> crate::BlockId {
                self.coarse_dirt_base
            }
            pub fn podzol(&self) -> crate::BlockId {
                self.podzol_base
            }
            pub fn cobblestone(&self) -> crate::BlockId {
                self.cobblestone_base
            }
            pub fn oak_planks(&self) -> crate::BlockId {
                self.oak_planks_base
            }
            pub fn spruce_planks(&self) -> crate::BlockId {
                self.spruce_planks_base
            }
            pub fn birch_planks(&self) -> crate::BlockId {
                self.birch_planks_base
            }
            pub fn jungle_planks(&self) -> crate::BlockId {
                self.jungle_planks_base
            }
            pub fn acacia_planks(&self) -> crate::BlockId {
                self.acacia_planks_base
            }
            pub fn dark_oak_planks(&self) -> crate::BlockId {
                self.dark_oak_planks_base
            }
            pub fn oak_sapling(&self) -> crate::BlockId {
                self.oak_sapling_base
            }
            pub fn spruce_sapling(&self) -> crate::BlockId {
                self.spruce_sapling_base
            }
            pub fn birch_sapling(&self) -> crate::BlockId {
                self.birch_sapling_base
            }
            pub fn jungle_sapling(&self) -> crate::BlockId {
                self.jungle_sapling_base
            }
            pub fn acacia_sapling(&self) -> crate::BlockId {
                self.acacia_sapling_base
            }
            pub fn dark_oak_sapling(&self) -> crate::BlockId {
                self.dark_oak_sapling_base
            }
            pub fn bedrock(&self) -> crate::BlockId {
                self.bedrock_base
            }
            pub fn water(&self) -> crate::BlockId {
                self.water_base
            }
            pub fn lava(&self) -> crate::BlockId {
                self.lava_base
            }
            pub fn sand(&self) -> crate::BlockId {
                self.sand_base
            }
            pub fn red_sand(&self) -> crate::BlockId {
                self.red_sand_base
            }
            pub fn gravel(&self) -> crate::BlockId {
                self.gravel_base
            }
            pub fn gold_ore(&self) -> crate::BlockId {
                self.gold_ore_base
            }
            pub fn iron_ore(&self) -> crate::BlockId {
                self.iron_ore_base
            }
            pub fn coal_ore(&self) -> crate::BlockId {
                self.coal_ore_base
            }
            pub fn oak_log(&self) -> crate::BlockId {
                self.oak_log_base
            }
            pub fn spruce_log(&self) -> crate::BlockId {
                self.spruce_log_base
            }
            pub fn birch_log(&self) -> crate::BlockId {
                self.birch_log_base
            }
            pub fn jungle_log(&self) -> crate::BlockId {
                self.jungle_log_base
            }
            pub fn acacia_log(&self) -> crate::BlockId {
                self.acacia_log_base
            }
            pub fn dark_oak_log(&self) -> crate::BlockId {
                self.dark_oak_log_base
            }
            pub fn stripped_spruce_log(&self) -> crate::BlockId {
                self.stripped_spruce_log_base
            }
            pub fn stripped_birch_log(&self) -> crate::BlockId {
                self.stripped_birch_log_base
            }
            pub fn stripped_jungle_log(&self) -> crate::BlockId {
                self.stripped_jungle_log_base
            }
            pub fn stripped_acacia_log(&self) -> crate::BlockId {
                self.stripped_acacia_log_base
            }
            pub fn stripped_dark_oak_log(&self) -> crate::BlockId {
                self.stripped_dark_oak_log_base
            }
            pub fn stripped_oak_log(&self) -> crate::BlockId {
                self.stripped_oak_log_base
            }
            pub fn oak_wood(&self) -> crate::BlockId {
                self.oak_wood_base
            }
            pub fn spruce_wood(&self) -> crate::BlockId {
                self.spruce_wood_base
            }
            pub fn birch_wood(&self) -> crate::BlockId {
                self.birch_wood_base
            }
            pub fn jungle_wood(&self) -> crate::BlockId {
                self.jungle_wood_base
            }
            pub fn acacia_wood(&self) -> crate::BlockId {
                self.acacia_wood_base
            }
            pub fn dark_oak_wood(&self) -> crate::BlockId {
                self.dark_oak_wood_base
            }
            pub fn stripped_oak_wood(&self) -> crate::BlockId {
                self.stripped_oak_wood_base
            }
            pub fn stripped_spruce_wood(&self) -> crate::BlockId {
                self.stripped_spruce_wood_base
            }
            pub fn stripped_birch_wood(&self) -> crate::BlockId {
                self.stripped_birch_wood_base
            }
            pub fn stripped_jungle_wood(&self) -> crate::BlockId {
                self.stripped_jungle_wood_base
            }
            pub fn stripped_acacia_wood(&self) -> crate::BlockId {
                self.stripped_acacia_wood_base
            }
            pub fn stripped_dark_oak_wood(&self) -> crate::BlockId {
                self.stripped_dark_oak_wood_base
            }
            pub fn oak_leaves(&self) -> crate::BlockId {
                self.oak_leaves_base
            }
            pub fn spruce_leaves(&self) -> crate::BlockId {
                self.spruce_leaves_base
            }
            pub fn birch_leaves(&self) -> crate::BlockId {
                self.birch_leaves_base
            }
            pub fn jungle_leaves(&self) -> crate::BlockId {
                self.jungle_leaves_base
            }
            pub fn acacia_leaves(&self) -> crate::BlockId {
                self.acacia_leaves_base
            }
            pub fn dark_oak_leaves(&self) -> crate::BlockId {
                self.dark_oak_leaves_base
            }
            pub fn sponge(&self) -> crate::BlockId {
                self.sponge_base
            }
            pub fn wet_sponge(&self) -> crate::BlockId {
                self.wet_sponge_base
            }
            pub fn glass(&self) -> crate::BlockId {
                self.glass_base
            }
            pub fn lapis_ore(&self) -> crate::BlockId {
                self.lapis_ore_base
            }
            pub fn lapis_block(&self) -> crate::BlockId {
                self.lapis_block_base
            }
            pub fn dispenser(&self) -> crate::BlockId {
                self.dispenser_base
            }
            pub fn sandstone(&self) -> crate::BlockId {
                self.sandstone_base
            }
            pub fn chiseled_sandstone(&self) -> crate::BlockId {
                self.chiseled_sandstone_base
            }
            pub fn cut_sandstone(&self) -> crate::BlockId {
                self.cut_sandstone_base
            }
            pub fn note_block(&self) -> crate::BlockId {
                self.note_block_base
            }
            pub fn white_bed(&self) -> crate::BlockId {
                self.white_bed_base
            }
            pub fn orange_bed(&self) -> crate::BlockId {
                self.orange_bed_base
            }
            pub fn magenta_bed(&self) -> crate::BlockId {
                self.magenta_bed_base
            }
            pub fn light_blue_bed(&self) -> crate::BlockId {
                self.light_blue_bed_base
            }
            pub fn yellow_bed(&self) -> crate::BlockId {
                self.yellow_bed_base
            }
            pub fn lime_bed(&self) -> crate::BlockId {
                self.lime_bed_base
            }
            pub fn pink_bed(&self) -> crate::BlockId {
                self.pink_bed_base
            }
            pub fn gray_bed(&self) -> crate::BlockId {
                self.gray_bed_base
            }
            pub fn light_gray_bed(&self) -> crate::BlockId {
                self.light_gray_bed_base
            }
            pub fn cyan_bed(&self) -> crate::BlockId {
                self.cyan_bed_base
            }
            pub fn purple_bed(&self) -> crate::BlockId {
                self.purple_bed_base
            }
            pub fn blue_bed(&self) -> crate::BlockId {
                self.blue_bed_base
            }
            pub fn brown_bed(&self) -> crate::BlockId {
                self.brown_bed_base
            }
            pub fn green_bed(&self) -> crate::BlockId {
                self.green_bed_base
            }
            pub fn red_bed(&self) -> crate::BlockId {
                self.red_bed_base
            }
            pub fn black_bed(&self) -> crate::BlockId {
                self.black_bed_base
            }
            pub fn powered_rail(&self) -> crate::BlockId {
                self.powered_rail_base
            }
            pub fn detector_rail(&self) -> crate::BlockId {
                self.detector_rail_base
            }
            pub fn sticky_piston(&self) -> crate::BlockId {
                self.sticky_piston_base
            }
            pub fn cobweb(&self) -> crate::BlockId {
                self.cobweb_base
            }
            pub fn grass(&self) -> crate::BlockId {
                self.grass_base
            }
            pub fn fern(&self) -> crate::BlockId {
                self.fern_base
            }
            pub fn dead_bush(&self) -> crate::BlockId {
                self.dead_bush_base
            }
            pub fn seagrass(&self) -> crate::BlockId {
                self.seagrass_base
            }
            pub fn tall_seagrass(&self) -> crate::BlockId {
                self.tall_seagrass_base
            }
            pub fn piston(&self) -> crate::BlockId {
                self.piston_base
            }
            pub fn piston_head(&self) -> crate::BlockId {
                self.piston_head_base
            }
            pub fn white_wool(&self) -> crate::BlockId {
                self.white_wool_base
            }
            pub fn orange_wool(&self) -> crate::BlockId {
                self.orange_wool_base
            }
            pub fn magenta_wool(&self) -> crate::BlockId {
                self.magenta_wool_base
            }
            pub fn light_blue_wool(&self) -> crate::BlockId {
                self.light_blue_wool_base
            }
            pub fn yellow_wool(&self) -> crate::BlockId {
                self.yellow_wool_base
            }
            pub fn lime_wool(&self) -> crate::BlockId {
                self.lime_wool_base
            }
            pub fn pink_wool(&self) -> crate::BlockId {
                self.pink_wool_base
            }
            pub fn gray_wool(&self) -> crate::BlockId {
                self.gray_wool_base
            }
            pub fn light_gray_wool(&self) -> crate::BlockId {
                self.light_gray_wool_base
            }
            pub fn cyan_wool(&self) -> crate::BlockId {
                self.cyan_wool_base
            }
            pub fn purple_wool(&self) -> crate::BlockId {
                self.purple_wool_base
            }
            pub fn blue_wool(&self) -> crate::BlockId {
                self.blue_wool_base
            }
            pub fn brown_wool(&self) -> crate::BlockId {
                self.brown_wool_base
            }
            pub fn green_wool(&self) -> crate::BlockId {
                self.green_wool_base
            }
            pub fn red_wool(&self) -> crate::BlockId {
                self.red_wool_base
            }
            pub fn black_wool(&self) -> crate::BlockId {
                self.black_wool_base
            }
            pub fn moving_piston(&self) -> crate::BlockId {
                self.moving_piston_base
            }
            pub fn dandelion(&self) -> crate::BlockId {
                self.dandelion_base
            }
            pub fn poppy(&self) -> crate::BlockId {
                self.poppy_base
            }
            pub fn blue_orchid(&self) -> crate::BlockId {
                self.blue_orchid_base
            }
            pub fn allium(&self) -> crate::BlockId {
                self.allium_base
            }
            pub fn azure_bluet(&self) -> crate::BlockId {
                self.azure_bluet_base
            }
            pub fn red_tulip(&self) -> crate::BlockId {
                self.red_tulip_base
            }
            pub fn orange_tulip(&self) -> crate::BlockId {
                self.orange_tulip_base
            }
            pub fn white_tulip(&self) -> crate::BlockId {
                self.white_tulip_base
            }
            pub fn pink_tulip(&self) -> crate::BlockId {
                self.pink_tulip_base
            }
            pub fn oxeye_daisy(&self) -> crate::BlockId {
                self.oxeye_daisy_base
            }
            pub fn brown_mushroom(&self) -> crate::BlockId {
                self.brown_mushroom_base
            }
            pub fn red_mushroom(&self) -> crate::BlockId {
                self.red_mushroom_base
            }
            pub fn gold_block(&self) -> crate::BlockId {
                self.gold_block_base
            }
            pub fn iron_block(&self) -> crate::BlockId {
                self.iron_block_base
            }
            pub fn bricks(&self) -> crate::BlockId {
                self.bricks_base
            }
            pub fn tnt(&self) -> crate::BlockId {
                self.tnt_base
            }
            pub fn bookshelf(&self) -> crate::BlockId {
                self.bookshelf_base
            }
            pub fn mossy_cobblestone(&self) -> crate::BlockId {
                self.mossy_cobblestone_base
            }
            pub fn obsidian(&self) -> crate::BlockId {
                self.obsidian_base
            }
            pub fn torch(&self) -> crate::BlockId {
                self.torch_base
            }
            pub fn wall_torch(&self) -> crate::BlockId {
                self.wall_torch_base
            }
            pub fn fire(&self) -> crate::BlockId {
                self.fire_base
            }
            pub fn spawner(&self) -> crate::BlockId {
                self.spawner_base
            }
            pub fn oak_stairs(&self) -> crate::BlockId {
                self.oak_stairs_base
            }
            pub fn chest(&self) -> crate::BlockId {
                self.chest_base
            }
            pub fn redstone_wire(&self) -> crate::BlockId {
                self.redstone_wire_base
            }
            pub fn diamond_ore(&self) -> crate::BlockId {
                self.diamond_ore_base
            }
            pub fn diamond_block(&self) -> crate::BlockId {
                self.diamond_block_base
            }
            pub fn crafting_table(&self) -> crate::BlockId {
                self.crafting_table_base
            }
            pub fn wheat(&self) -> crate::BlockId {
                self.wheat_base
            }
            pub fn farmland(&self) -> crate::BlockId {
                self.farmland_base
            }
            pub fn furnace(&self) -> crate::BlockId {
                self.furnace_base
            }
            pub fn sign(&self) -> crate::BlockId {
                self.sign_base
            }
            pub fn oak_door(&self) -> crate::BlockId {
                self.oak_door_base
            }
            pub fn ladder(&self) -> crate::BlockId {
                self.ladder_base
            }
            pub fn rail(&self) -> crate::BlockId {
                self.rail_base
            }
            pub fn cobblestone_stairs(&self) -> crate::BlockId {
                self.cobblestone_stairs_base
            }
            pub fn wall_sign(&self) -> crate::BlockId {
                self.wall_sign_base
            }
            pub fn lever(&self) -> crate::BlockId {
                self.lever_base
            }
            pub fn stone_pressure_plate(&self) -> crate::BlockId {
                self.stone_pressure_plate_base
            }
            pub fn iron_door(&self) -> crate::BlockId {
                self.iron_door_base
            }
            pub fn oak_pressure_plate(&self) -> crate::BlockId {
                self.oak_pressure_plate_base
            }
            pub fn spruce_pressure_plate(&self) -> crate::BlockId {
                self.spruce_pressure_plate_base
            }
            pub fn birch_pressure_plate(&self) -> crate::BlockId {
                self.birch_pressure_plate_base
            }
            pub fn jungle_pressure_plate(&self) -> crate::BlockId {
                self.jungle_pressure_plate_base
            }
            pub fn acacia_pressure_plate(&self) -> crate::BlockId {
                self.acacia_pressure_plate_base
            }
            pub fn dark_oak_pressure_plate(&self) -> crate::BlockId {
                self.dark_oak_pressure_plate_base
            }
            pub fn redstone_ore(&self) -> crate::BlockId {
                self.redstone_ore_base
            }
            pub fn redstone_torch(&self) -> crate::BlockId {
                self.redstone_torch_base
            }
            pub fn redstone_wall_torch(&self) -> crate::BlockId {
                self.redstone_wall_torch_base
            }
            pub fn stone_button(&self) -> crate::BlockId {
                self.stone_button_base
            }
            pub fn snow(&self) -> crate::BlockId {
                self.snow_base
            }
            pub fn ice(&self) -> crate::BlockId {
                self.ice_base
            }
            pub fn snow_block(&self) -> crate::BlockId {
                self.snow_block_base
            }
            pub fn cactus(&self) -> crate::BlockId {
                self.cactus_base
            }
            pub fn clay(&self) -> crate::BlockId {
                self.clay_base
            }
            pub fn sugar_cane(&self) -> crate::BlockId {
                self.sugar_cane_base
            }
            pub fn jukebox(&self) -> crate::BlockId {
                self.jukebox_base
            }
            pub fn oak_fence(&self) -> crate::BlockId {
                self.oak_fence_base
            }
            pub fn pumpkin(&self) -> crate::BlockId {
                self.pumpkin_base
            }
            pub fn netherrack(&self) -> crate::BlockId {
                self.netherrack_base
            }
            pub fn soul_sand(&self) -> crate::BlockId {
                self.soul_sand_base
            }
            pub fn glowstone(&self) -> crate::BlockId {
                self.glowstone_base
            }
            pub fn nether_portal(&self) -> crate::BlockId {
                self.nether_portal_base
            }
            pub fn carved_pumpkin(&self) -> crate::BlockId {
                self.carved_pumpkin_base
            }
            pub fn jack_o_lantern(&self) -> crate::BlockId {
                self.jack_o_lantern_base
            }
            pub fn cake(&self) -> crate::BlockId {
                self.cake_base
            }
            pub fn repeater(&self) -> crate::BlockId {
                self.repeater_base
            }
            pub fn white_stained_glass(&self) -> crate::BlockId {
                self.white_stained_glass_base
            }
            pub fn orange_stained_glass(&self) -> crate::BlockId {
                self.orange_stained_glass_base
            }
            pub fn magenta_stained_glass(&self) -> crate::BlockId {
                self.magenta_stained_glass_base
            }
            pub fn light_blue_stained_glass(&self) -> crate::BlockId {
                self.light_blue_stained_glass_base
            }
            pub fn yellow_stained_glass(&self) -> crate::BlockId {
                self.yellow_stained_glass_base
            }
            pub fn lime_stained_glass(&self) -> crate::BlockId {
                self.lime_stained_glass_base
            }
            pub fn pink_stained_glass(&self) -> crate::BlockId {
                self.pink_stained_glass_base
            }
            pub fn gray_stained_glass(&self) -> crate::BlockId {
                self.gray_stained_glass_base
            }
            pub fn light_gray_stained_glass(&self) -> crate::BlockId {
                self.light_gray_stained_glass_base
            }
            pub fn cyan_stained_glass(&self) -> crate::BlockId {
                self.cyan_stained_glass_base
            }
            pub fn purple_stained_glass(&self) -> crate::BlockId {
                self.purple_stained_glass_base
            }
            pub fn blue_stained_glass(&self) -> crate::BlockId {
                self.blue_stained_glass_base
            }
            pub fn brown_stained_glass(&self) -> crate::BlockId {
                self.brown_stained_glass_base
            }
            pub fn green_stained_glass(&self) -> crate::BlockId {
                self.green_stained_glass_base
            }
            pub fn red_stained_glass(&self) -> crate::BlockId {
                self.red_stained_glass_base
            }
            pub fn black_stained_glass(&self) -> crate::BlockId {
                self.black_stained_glass_base
            }
            pub fn oak_trapdoor(&self) -> crate::BlockId {
                self.oak_trapdoor_base
            }
            pub fn spruce_trapdoor(&self) -> crate::BlockId {
                self.spruce_trapdoor_base
            }
            pub fn birch_trapdoor(&self) -> crate::BlockId {
                self.birch_trapdoor_base
            }
            pub fn jungle_trapdoor(&self) -> crate::BlockId {
                self.jungle_trapdoor_base
            }
            pub fn acacia_trapdoor(&self) -> crate::BlockId {
                self.acacia_trapdoor_base
            }
            pub fn dark_oak_trapdoor(&self) -> crate::BlockId {
                self.dark_oak_trapdoor_base
            }
            pub fn infested_stone(&self) -> crate::BlockId {
                self.infested_stone_base
            }
            pub fn infested_cobblestone(&self) -> crate::BlockId {
                self.infested_cobblestone_base
            }
            pub fn infested_stone_bricks(&self) -> crate::BlockId {
                self.infested_stone_bricks_base
            }
            pub fn infested_mossy_stone_bricks(&self) -> crate::BlockId {
                self.infested_mossy_stone_bricks_base
            }
            pub fn infested_cracked_stone_bricks(&self) -> crate::BlockId {
                self.infested_cracked_stone_bricks_base
            }
            pub fn infested_chiseled_stone_bricks(&self) -> crate::BlockId {
                self.infested_chiseled_stone_bricks_base
            }
            pub fn stone_bricks(&self) -> crate::BlockId {
                self.stone_bricks_base
            }
            pub fn mossy_stone_bricks(&self) -> crate::BlockId {
                self.mossy_stone_bricks_base
            }
            pub fn cracked_stone_bricks(&self) -> crate::BlockId {
                self.cracked_stone_bricks_base
            }
            pub fn chiseled_stone_bricks(&self) -> crate::BlockId {
                self.chiseled_stone_bricks_base
            }
            pub fn brown_mushroom_block(&self) -> crate::BlockId {
                self.brown_mushroom_block_base
            }
            pub fn red_mushroom_block(&self) -> crate::BlockId {
                self.red_mushroom_block_base
            }
            pub fn mushroom_stem(&self) -> crate::BlockId {
                self.mushroom_stem_base
            }
            pub fn iron_bars(&self) -> crate::BlockId {
                self.iron_bars_base
            }
            pub fn glass_pane(&self) -> crate::BlockId {
                self.glass_pane_base
            }
            pub fn melon(&self) -> crate::BlockId {
                self.melon_base
            }
            pub fn attached_pumpkin_stem(&self) -> crate::BlockId {
                self.attached_pumpkin_stem_base
            }
            pub fn attached_melon_stem(&self) -> crate::BlockId {
                self.attached_melon_stem_base
            }
            pub fn pumpkin_stem(&self) -> crate::BlockId {
                self.pumpkin_stem_base
            }
            pub fn melon_stem(&self) -> crate::BlockId {
                self.melon_stem_base
            }
            pub fn vine(&self) -> crate::BlockId {
                self.vine_base
            }
            pub fn oak_fence_gate(&self) -> crate::BlockId {
                self.oak_fence_gate_base
            }
            pub fn brick_stairs(&self) -> crate::BlockId {
                self.brick_stairs_base
            }
            pub fn stone_brick_stairs(&self) -> crate::BlockId {
                self.stone_brick_stairs_base
            }
            pub fn mycelium(&self) -> crate::BlockId {
                self.mycelium_base
            }
            pub fn lily_pad(&self) -> crate::BlockId {
                self.lily_pad_base
            }
            pub fn nether_bricks(&self) -> crate::BlockId {
                self.nether_bricks_base
            }
            pub fn nether_brick_fence(&self) -> crate::BlockId {
                self.nether_brick_fence_base
            }
            pub fn nether_brick_stairs(&self) -> crate::BlockId {
                self.nether_brick_stairs_base
            }
            pub fn nether_wart(&self) -> crate::BlockId {
                self.nether_wart_base
            }
            pub fn enchanting_table(&self) -> crate::BlockId {
                self.enchanting_table_base
            }
            pub fn brewing_stand(&self) -> crate::BlockId {
                self.brewing_stand_base
            }
            pub fn cauldron(&self) -> crate::BlockId {
                self.cauldron_base
            }
            pub fn end_portal(&self) -> crate::BlockId {
                self.end_portal_base
            }
            pub fn end_portal_frame(&self) -> crate::BlockId {
                self.end_portal_frame_base
            }
            pub fn end_stone(&self) -> crate::BlockId {
                self.end_stone_base
            }
            pub fn dragon_egg(&self) -> crate::BlockId {
                self.dragon_egg_base
            }
            pub fn redstone_lamp(&self) -> crate::BlockId {
                self.redstone_lamp_base
            }
            pub fn cocoa(&self) -> crate::BlockId {
                self.cocoa_base
            }
            pub fn sandstone_stairs(&self) -> crate::BlockId {
                self.sandstone_stairs_base
            }
            pub fn emerald_ore(&self) -> crate::BlockId {
                self.emerald_ore_base
            }
            pub fn ender_chest(&self) -> crate::BlockId {
                self.ender_chest_base
            }
            pub fn tripwire_hook(&self) -> crate::BlockId {
                self.tripwire_hook_base
            }
            pub fn tripwire(&self) -> crate::BlockId {
                self.tripwire_base
            }
            pub fn emerald_block(&self) -> crate::BlockId {
                self.emerald_block_base
            }
            pub fn spruce_stairs(&self) -> crate::BlockId {
                self.spruce_stairs_base
            }
            pub fn birch_stairs(&self) -> crate::BlockId {
                self.birch_stairs_base
            }
            pub fn jungle_stairs(&self) -> crate::BlockId {
                self.jungle_stairs_base
            }
            pub fn command_block(&self) -> crate::BlockId {
                self.command_block_base
            }
            pub fn beacon(&self) -> crate::BlockId {
                self.beacon_base
            }
            pub fn cobblestone_wall(&self) -> crate::BlockId {
                self.cobblestone_wall_base
            }
            pub fn mossy_cobblestone_wall(&self) -> crate::BlockId {
                self.mossy_cobblestone_wall_base
            }
            pub fn flower_pot(&self) -> crate::BlockId {
                self.flower_pot_base
            }
            pub fn potted_oak_sapling(&self) -> crate::BlockId {
                self.potted_oak_sapling_base
            }
            pub fn potted_spruce_sapling(&self) -> crate::BlockId {
                self.potted_spruce_sapling_base
            }
            pub fn potted_birch_sapling(&self) -> crate::BlockId {
                self.potted_birch_sapling_base
            }
            pub fn potted_jungle_sapling(&self) -> crate::BlockId {
                self.potted_jungle_sapling_base
            }
            pub fn potted_acacia_sapling(&self) -> crate::BlockId {
                self.potted_acacia_sapling_base
            }
            pub fn potted_dark_oak_sapling(&self) -> crate::BlockId {
                self.potted_dark_oak_sapling_base
            }
            pub fn potted_fern(&self) -> crate::BlockId {
                self.potted_fern_base
            }
            pub fn potted_dandelion(&self) -> crate::BlockId {
                self.potted_dandelion_base
            }
            pub fn potted_poppy(&self) -> crate::BlockId {
                self.potted_poppy_base
            }
            pub fn potted_blue_orchid(&self) -> crate::BlockId {
                self.potted_blue_orchid_base
            }
            pub fn potted_allium(&self) -> crate::BlockId {
                self.potted_allium_base
            }
            pub fn potted_azure_bluet(&self) -> crate::BlockId {
                self.potted_azure_bluet_base
            }
            pub fn potted_red_tulip(&self) -> crate::BlockId {
                self.potted_red_tulip_base
            }
            pub fn potted_orange_tulip(&self) -> crate::BlockId {
                self.potted_orange_tulip_base
            }
            pub fn potted_white_tulip(&self) -> crate::BlockId {
                self.potted_white_tulip_base
            }
            pub fn potted_pink_tulip(&self) -> crate::BlockId {
                self.potted_pink_tulip_base
            }
            pub fn potted_oxeye_daisy(&self) -> crate::BlockId {
                self.potted_oxeye_daisy_base
            }
            pub fn potted_red_mushroom(&self) -> crate::BlockId {
                self.potted_red_mushroom_base
            }
            pub fn potted_brown_mushroom(&self) -> crate::BlockId {
                self.potted_brown_mushroom_base
            }
            pub fn potted_dead_bush(&self) -> crate::BlockId {
                self.potted_dead_bush_base
            }
            pub fn potted_cactus(&self) -> crate::BlockId {
                self.potted_cactus_base
            }
            pub fn carrots(&self) -> crate::BlockId {
                self.carrots_base
            }
            pub fn potatoes(&self) -> crate::BlockId {
                self.potatoes_base
            }
            pub fn oak_button(&self) -> crate::BlockId {
                self.oak_button_base
            }
            pub fn spruce_button(&self) -> crate::BlockId {
                self.spruce_button_base
            }
            pub fn birch_button(&self) -> crate::BlockId {
                self.birch_button_base
            }
            pub fn jungle_button(&self) -> crate::BlockId {
                self.jungle_button_base
            }
            pub fn acacia_button(&self) -> crate::BlockId {
                self.acacia_button_base
            }
            pub fn dark_oak_button(&self) -> crate::BlockId {
                self.dark_oak_button_base
            }
            pub fn skeleton_wall_skull(&self) -> crate::BlockId {
                self.skeleton_wall_skull_base
            }
            pub fn skeleton_skull(&self) -> crate::BlockId {
                self.skeleton_skull_base
            }
            pub fn wither_skeleton_wall_skull(&self) -> crate::BlockId {
                self.wither_skeleton_wall_skull_base
            }
            pub fn wither_skeleton_skull(&self) -> crate::BlockId {
                self.wither_skeleton_skull_base
            }
            pub fn zombie_wall_head(&self) -> crate::BlockId {
                self.zombie_wall_head_base
            }
            pub fn zombie_head(&self) -> crate::BlockId {
                self.zombie_head_base
            }
            pub fn player_wall_head(&self) -> crate::BlockId {
                self.player_wall_head_base
            }
            pub fn player_head(&self) -> crate::BlockId {
                self.player_head_base
            }
            pub fn creeper_wall_head(&self) -> crate::BlockId {
                self.creeper_wall_head_base
            }
            pub fn creeper_head(&self) -> crate::BlockId {
                self.creeper_head_base
            }
            pub fn dragon_wall_head(&self) -> crate::BlockId {
                self.dragon_wall_head_base
            }
            pub fn dragon_head(&self) -> crate::BlockId {
                self.dragon_head_base
            }
            pub fn anvil(&self) -> crate::BlockId {
                self.anvil_base
            }
            pub fn chipped_anvil(&self) -> crate::BlockId {
                self.chipped_anvil_base
            }
            pub fn damaged_anvil(&self) -> crate::BlockId {
                self.damaged_anvil_base
            }
            pub fn trapped_chest(&self) -> crate::BlockId {
                self.trapped_chest_base
            }
            pub fn light_weighted_pressure_plate(&self) -> crate::BlockId {
                self.light_weighted_pressure_plate_base
            }
            pub fn heavy_weighted_pressure_plate(&self) -> crate::BlockId {
                self.heavy_weighted_pressure_plate_base
            }
            pub fn comparator(&self) -> crate::BlockId {
                self.comparator_base
            }
            pub fn daylight_detector(&self) -> crate::BlockId {
                self.daylight_detector_base
            }
            pub fn redstone_block(&self) -> crate::BlockId {
                self.redstone_block_base
            }
            pub fn nether_quartz_ore(&self) -> crate::BlockId {
                self.nether_quartz_ore_base
            }
            pub fn hopper(&self) -> crate::BlockId {
                self.hopper_base
            }
            pub fn quartz_block(&self) -> crate::BlockId {
                self.quartz_block_base
            }
            pub fn chiseled_quartz_block(&self) -> crate::BlockId {
                self.chiseled_quartz_block_base
            }
            pub fn quartz_pillar(&self) -> crate::BlockId {
                self.quartz_pillar_base
            }
            pub fn quartz_stairs(&self) -> crate::BlockId {
                self.quartz_stairs_base
            }
            pub fn activator_rail(&self) -> crate::BlockId {
                self.activator_rail_base
            }
            pub fn dropper(&self) -> crate::BlockId {
                self.dropper_base
            }
            pub fn white_terracotta(&self) -> crate::BlockId {
                self.white_terracotta_base
            }
            pub fn orange_terracotta(&self) -> crate::BlockId {
                self.orange_terracotta_base
            }
            pub fn magenta_terracotta(&self) -> crate::BlockId {
                self.magenta_terracotta_base
            }
            pub fn light_blue_terracotta(&self) -> crate::BlockId {
                self.light_blue_terracotta_base
            }
            pub fn yellow_terracotta(&self) -> crate::BlockId {
                self.yellow_terracotta_base
            }
            pub fn lime_terracotta(&self) -> crate::BlockId {
                self.lime_terracotta_base
            }
            pub fn pink_terracotta(&self) -> crate::BlockId {
                self.pink_terracotta_base
            }
            pub fn gray_terracotta(&self) -> crate::BlockId {
                self.gray_terracotta_base
            }
            pub fn light_gray_terracotta(&self) -> crate::BlockId {
                self.light_gray_terracotta_base
            }
            pub fn cyan_terracotta(&self) -> crate::BlockId {
                self.cyan_terracotta_base
            }
            pub fn purple_terracotta(&self) -> crate::BlockId {
                self.purple_terracotta_base
            }
            pub fn blue_terracotta(&self) -> crate::BlockId {
                self.blue_terracotta_base
            }
            pub fn brown_terracotta(&self) -> crate::BlockId {
                self.brown_terracotta_base
            }
            pub fn green_terracotta(&self) -> crate::BlockId {
                self.green_terracotta_base
            }
            pub fn red_terracotta(&self) -> crate::BlockId {
                self.red_terracotta_base
            }
            pub fn black_terracotta(&self) -> crate::BlockId {
                self.black_terracotta_base
            }
            pub fn white_stained_glass_pane(&self) -> crate::BlockId {
                self.white_stained_glass_pane_base
            }
            pub fn orange_stained_glass_pane(&self) -> crate::BlockId {
                self.orange_stained_glass_pane_base
            }
            pub fn magenta_stained_glass_pane(&self) -> crate::BlockId {
                self.magenta_stained_glass_pane_base
            }
            pub fn light_blue_stained_glass_pane(&self) -> crate::BlockId {
                self.light_blue_stained_glass_pane_base
            }
            pub fn yellow_stained_glass_pane(&self) -> crate::BlockId {
                self.yellow_stained_glass_pane_base
            }
            pub fn lime_stained_glass_pane(&self) -> crate::BlockId {
                self.lime_stained_glass_pane_base
            }
            pub fn pink_stained_glass_pane(&self) -> crate::BlockId {
                self.pink_stained_glass_pane_base
            }
            pub fn gray_stained_glass_pane(&self) -> crate::BlockId {
                self.gray_stained_glass_pane_base
            }
            pub fn light_gray_stained_glass_pane(&self) -> crate::BlockId {
                self.light_gray_stained_glass_pane_base
            }
            pub fn cyan_stained_glass_pane(&self) -> crate::BlockId {
                self.cyan_stained_glass_pane_base
            }
            pub fn purple_stained_glass_pane(&self) -> crate::BlockId {
                self.purple_stained_glass_pane_base
            }
            pub fn blue_stained_glass_pane(&self) -> crate::BlockId {
                self.blue_stained_glass_pane_base
            }
            pub fn brown_stained_glass_pane(&self) -> crate::BlockId {
                self.brown_stained_glass_pane_base
            }
            pub fn green_stained_glass_pane(&self) -> crate::BlockId {
                self.green_stained_glass_pane_base
            }
            pub fn red_stained_glass_pane(&self) -> crate::BlockId {
                self.red_stained_glass_pane_base
            }
            pub fn black_stained_glass_pane(&self) -> crate::BlockId {
                self.black_stained_glass_pane_base
            }
            pub fn acacia_stairs(&self) -> crate::BlockId {
                self.acacia_stairs_base
            }
            pub fn dark_oak_stairs(&self) -> crate::BlockId {
                self.dark_oak_stairs_base
            }
            pub fn slime_block(&self) -> crate::BlockId {
                self.slime_block_base
            }
            pub fn barrier(&self) -> crate::BlockId {
                self.barrier_base
            }
            pub fn iron_trapdoor(&self) -> crate::BlockId {
                self.iron_trapdoor_base
            }
            pub fn prismarine(&self) -> crate::BlockId {
                self.prismarine_base
            }
            pub fn prismarine_bricks(&self) -> crate::BlockId {
                self.prismarine_bricks_base
            }
            pub fn dark_prismarine(&self) -> crate::BlockId {
                self.dark_prismarine_base
            }
            pub fn prismarine_stairs(&self) -> crate::BlockId {
                self.prismarine_stairs_base
            }
            pub fn prismarine_brick_stairs(&self) -> crate::BlockId {
                self.prismarine_brick_stairs_base
            }
            pub fn dark_prismarine_stairs(&self) -> crate::BlockId {
                self.dark_prismarine_stairs_base
            }
            pub fn prismarine_slab(&self) -> crate::BlockId {
                self.prismarine_slab_base
            }
            pub fn prismarine_brick_slab(&self) -> crate::BlockId {
                self.prismarine_brick_slab_base
            }
            pub fn dark_prismarine_slab(&self) -> crate::BlockId {
                self.dark_prismarine_slab_base
            }
            pub fn sea_lantern(&self) -> crate::BlockId {
                self.sea_lantern_base
            }
            pub fn hay_block(&self) -> crate::BlockId {
                self.hay_block_base
            }
            pub fn white_carpet(&self) -> crate::BlockId {
                self.white_carpet_base
            }
            pub fn orange_carpet(&self) -> crate::BlockId {
                self.orange_carpet_base
            }
            pub fn magenta_carpet(&self) -> crate::BlockId {
                self.magenta_carpet_base
            }
            pub fn light_blue_carpet(&self) -> crate::BlockId {
                self.light_blue_carpet_base
            }
            pub fn yellow_carpet(&self) -> crate::BlockId {
                self.yellow_carpet_base
            }
            pub fn lime_carpet(&self) -> crate::BlockId {
                self.lime_carpet_base
            }
            pub fn pink_carpet(&self) -> crate::BlockId {
                self.pink_carpet_base
            }
            pub fn gray_carpet(&self) -> crate::BlockId {
                self.gray_carpet_base
            }
            pub fn light_gray_carpet(&self) -> crate::BlockId {
                self.light_gray_carpet_base
            }
            pub fn cyan_carpet(&self) -> crate::BlockId {
                self.cyan_carpet_base
            }
            pub fn purple_carpet(&self) -> crate::BlockId {
                self.purple_carpet_base
            }
            pub fn blue_carpet(&self) -> crate::BlockId {
                self.blue_carpet_base
            }
            pub fn brown_carpet(&self) -> crate::BlockId {
                self.brown_carpet_base
            }
            pub fn green_carpet(&self) -> crate::BlockId {
                self.green_carpet_base
            }
            pub fn red_carpet(&self) -> crate::BlockId {
                self.red_carpet_base
            }
            pub fn black_carpet(&self) -> crate::BlockId {
                self.black_carpet_base
            }
            pub fn terracotta(&self) -> crate::BlockId {
                self.terracotta_base
            }
            pub fn coal_block(&self) -> crate::BlockId {
                self.coal_block_base
            }
            pub fn packed_ice(&self) -> crate::BlockId {
                self.packed_ice_base
            }
            pub fn sunflower(&self) -> crate::BlockId {
                self.sunflower_base
            }
            pub fn lilac(&self) -> crate::BlockId {
                self.lilac_base
            }
            pub fn rose_bush(&self) -> crate::BlockId {
                self.rose_bush_base
            }
            pub fn peony(&self) -> crate::BlockId {
                self.peony_base
            }
            pub fn tall_grass(&self) -> crate::BlockId {
                self.tall_grass_base
            }
            pub fn large_fern(&self) -> crate::BlockId {
                self.large_fern_base
            }
            pub fn white_banner(&self) -> crate::BlockId {
                self.white_banner_base
            }
            pub fn orange_banner(&self) -> crate::BlockId {
                self.orange_banner_base
            }
            pub fn magenta_banner(&self) -> crate::BlockId {
                self.magenta_banner_base
            }
            pub fn light_blue_banner(&self) -> crate::BlockId {
                self.light_blue_banner_base
            }
            pub fn yellow_banner(&self) -> crate::BlockId {
                self.yellow_banner_base
            }
            pub fn lime_banner(&self) -> crate::BlockId {
                self.lime_banner_base
            }
            pub fn pink_banner(&self) -> crate::BlockId {
                self.pink_banner_base
            }
            pub fn gray_banner(&self) -> crate::BlockId {
                self.gray_banner_base
            }
            pub fn light_gray_banner(&self) -> crate::BlockId {
                self.light_gray_banner_base
            }
            pub fn cyan_banner(&self) -> crate::BlockId {
                self.cyan_banner_base
            }
            pub fn purple_banner(&self) -> crate::BlockId {
                self.purple_banner_base
            }
            pub fn blue_banner(&self) -> crate::BlockId {
                self.blue_banner_base
            }
            pub fn brown_banner(&self) -> crate::BlockId {
                self.brown_banner_base
            }
            pub fn green_banner(&self) -> crate::BlockId {
                self.green_banner_base
            }
            pub fn red_banner(&self) -> crate::BlockId {
                self.red_banner_base
            }
            pub fn black_banner(&self) -> crate::BlockId {
                self.black_banner_base
            }
            pub fn white_wall_banner(&self) -> crate::BlockId {
                self.white_wall_banner_base
            }
            pub fn orange_wall_banner(&self) -> crate::BlockId {
                self.orange_wall_banner_base
            }
            pub fn magenta_wall_banner(&self) -> crate::BlockId {
                self.magenta_wall_banner_base
            }
            pub fn light_blue_wall_banner(&self) -> crate::BlockId {
                self.light_blue_wall_banner_base
            }
            pub fn yellow_wall_banner(&self) -> crate::BlockId {
                self.yellow_wall_banner_base
            }
            pub fn lime_wall_banner(&self) -> crate::BlockId {
                self.lime_wall_banner_base
            }
            pub fn pink_wall_banner(&self) -> crate::BlockId {
                self.pink_wall_banner_base
            }
            pub fn gray_wall_banner(&self) -> crate::BlockId {
                self.gray_wall_banner_base
            }
            pub fn light_gray_wall_banner(&self) -> crate::BlockId {
                self.light_gray_wall_banner_base
            }
            pub fn cyan_wall_banner(&self) -> crate::BlockId {
                self.cyan_wall_banner_base
            }
            pub fn purple_wall_banner(&self) -> crate::BlockId {
                self.purple_wall_banner_base
            }
            pub fn blue_wall_banner(&self) -> crate::BlockId {
                self.blue_wall_banner_base
            }
            pub fn brown_wall_banner(&self) -> crate::BlockId {
                self.brown_wall_banner_base
            }
            pub fn green_wall_banner(&self) -> crate::BlockId {
                self.green_wall_banner_base
            }
            pub fn red_wall_banner(&self) -> crate::BlockId {
                self.red_wall_banner_base
            }
            pub fn black_wall_banner(&self) -> crate::BlockId {
                self.black_wall_banner_base
            }
            pub fn red_sandstone(&self) -> crate::BlockId {
                self.red_sandstone_base
            }
            pub fn chiseled_red_sandstone(&self) -> crate::BlockId {
                self.chiseled_red_sandstone_base
            }
            pub fn cut_red_sandstone(&self) -> crate::BlockId {
                self.cut_red_sandstone_base
            }
            pub fn red_sandstone_stairs(&self) -> crate::BlockId {
                self.red_sandstone_stairs_base
            }
            pub fn oak_slab(&self) -> crate::BlockId {
                self.oak_slab_base
            }
            pub fn spruce_slab(&self) -> crate::BlockId {
                self.spruce_slab_base
            }
            pub fn birch_slab(&self) -> crate::BlockId {
                self.birch_slab_base
            }
            pub fn jungle_slab(&self) -> crate::BlockId {
                self.jungle_slab_base
            }
            pub fn acacia_slab(&self) -> crate::BlockId {
                self.acacia_slab_base
            }
            pub fn dark_oak_slab(&self) -> crate::BlockId {
                self.dark_oak_slab_base
            }
            pub fn stone_slab(&self) -> crate::BlockId {
                self.stone_slab_base
            }
            pub fn sandstone_slab(&self) -> crate::BlockId {
                self.sandstone_slab_base
            }
            pub fn petrified_oak_slab(&self) -> crate::BlockId {
                self.petrified_oak_slab_base
            }
            pub fn cobblestone_slab(&self) -> crate::BlockId {
                self.cobblestone_slab_base
            }
            pub fn brick_slab(&self) -> crate::BlockId {
                self.brick_slab_base
            }
            pub fn stone_brick_slab(&self) -> crate::BlockId {
                self.stone_brick_slab_base
            }
            pub fn nether_brick_slab(&self) -> crate::BlockId {
                self.nether_brick_slab_base
            }
            pub fn quartz_slab(&self) -> crate::BlockId {
                self.quartz_slab_base
            }
            pub fn red_sandstone_slab(&self) -> crate::BlockId {
                self.red_sandstone_slab_base
            }
            pub fn purpur_slab(&self) -> crate::BlockId {
                self.purpur_slab_base
            }
            pub fn smooth_stone(&self) -> crate::BlockId {
                self.smooth_stone_base
            }
            pub fn smooth_sandstone(&self) -> crate::BlockId {
                self.smooth_sandstone_base
            }
            pub fn smooth_quartz(&self) -> crate::BlockId {
                self.smooth_quartz_base
            }
            pub fn smooth_red_sandstone(&self) -> crate::BlockId {
                self.smooth_red_sandstone_base
            }
            pub fn spruce_fence_gate(&self) -> crate::BlockId {
                self.spruce_fence_gate_base
            }
            pub fn birch_fence_gate(&self) -> crate::BlockId {
                self.birch_fence_gate_base
            }
            pub fn jungle_fence_gate(&self) -> crate::BlockId {
                self.jungle_fence_gate_base
            }
            pub fn acacia_fence_gate(&self) -> crate::BlockId {
                self.acacia_fence_gate_base
            }
            pub fn dark_oak_fence_gate(&self) -> crate::BlockId {
                self.dark_oak_fence_gate_base
            }
            pub fn spruce_fence(&self) -> crate::BlockId {
                self.spruce_fence_base
            }
            pub fn birch_fence(&self) -> crate::BlockId {
                self.birch_fence_base
            }
            pub fn jungle_fence(&self) -> crate::BlockId {
                self.jungle_fence_base
            }
            pub fn acacia_fence(&self) -> crate::BlockId {
                self.acacia_fence_base
            }
            pub fn dark_oak_fence(&self) -> crate::BlockId {
                self.dark_oak_fence_base
            }
            pub fn spruce_door(&self) -> crate::BlockId {
                self.spruce_door_base
            }
            pub fn birch_door(&self) -> crate::BlockId {
                self.birch_door_base
            }
            pub fn jungle_door(&self) -> crate::BlockId {
                self.jungle_door_base
            }
            pub fn acacia_door(&self) -> crate::BlockId {
                self.acacia_door_base
            }
            pub fn dark_oak_door(&self) -> crate::BlockId {
                self.dark_oak_door_base
            }
            pub fn end_rod(&self) -> crate::BlockId {
                self.end_rod_base
            }
            pub fn chorus_plant(&self) -> crate::BlockId {
                self.chorus_plant_base
            }
            pub fn chorus_flower(&self) -> crate::BlockId {
                self.chorus_flower_base
            }
            pub fn purpur_block(&self) -> crate::BlockId {
                self.purpur_block_base
            }
            pub fn purpur_pillar(&self) -> crate::BlockId {
                self.purpur_pillar_base
            }
            pub fn purpur_stairs(&self) -> crate::BlockId {
                self.purpur_stairs_base
            }
            pub fn end_stone_bricks(&self) -> crate::BlockId {
                self.end_stone_bricks_base
            }
            pub fn beetroots(&self) -> crate::BlockId {
                self.beetroots_base
            }
            pub fn grass_path(&self) -> crate::BlockId {
                self.grass_path_base
            }
            pub fn end_gateway(&self) -> crate::BlockId {
                self.end_gateway_base
            }
            pub fn repeating_command_block(&self) -> crate::BlockId {
                self.repeating_command_block_base
            }
            pub fn chain_command_block(&self) -> crate::BlockId {
                self.chain_command_block_base
            }
            pub fn frosted_ice(&self) -> crate::BlockId {
                self.frosted_ice_base
            }
            pub fn magma_block(&self) -> crate::BlockId {
                self.magma_block_base
            }
            pub fn nether_wart_block(&self) -> crate::BlockId {
                self.nether_wart_block_base
            }
            pub fn red_nether_bricks(&self) -> crate::BlockId {
                self.red_nether_bricks_base
            }
            pub fn bone_block(&self) -> crate::BlockId {
                self.bone_block_base
            }
            pub fn structure_void(&self) -> crate::BlockId {
                self.structure_void_base
            }
            pub fn observer(&self) -> crate::BlockId {
                self.observer_base
            }
            pub fn shulker_box(&self) -> crate::BlockId {
                self.shulker_box_base
            }
            pub fn white_shulker_box(&self) -> crate::BlockId {
                self.white_shulker_box_base
            }
            pub fn orange_shulker_box(&self) -> crate::BlockId {
                self.orange_shulker_box_base
            }
            pub fn magenta_shulker_box(&self) -> crate::BlockId {
                self.magenta_shulker_box_base
            }
            pub fn light_blue_shulker_box(&self) -> crate::BlockId {
                self.light_blue_shulker_box_base
            }
            pub fn yellow_shulker_box(&self) -> crate::BlockId {
                self.yellow_shulker_box_base
            }
            pub fn lime_shulker_box(&self) -> crate::BlockId {
                self.lime_shulker_box_base
            }
            pub fn pink_shulker_box(&self) -> crate::BlockId {
                self.pink_shulker_box_base
            }
            pub fn gray_shulker_box(&self) -> crate::BlockId {
                self.gray_shulker_box_base
            }
            pub fn light_gray_shulker_box(&self) -> crate::BlockId {
                self.light_gray_shulker_box_base
            }
            pub fn cyan_shulker_box(&self) -> crate::BlockId {
                self.cyan_shulker_box_base
            }
            pub fn purple_shulker_box(&self) -> crate::BlockId {
                self.purple_shulker_box_base
            }
            pub fn blue_shulker_box(&self) -> crate::BlockId {
                self.blue_shulker_box_base
            }
            pub fn brown_shulker_box(&self) -> crate::BlockId {
                self.brown_shulker_box_base
            }
            pub fn green_shulker_box(&self) -> crate::BlockId {
                self.green_shulker_box_base
            }
            pub fn red_shulker_box(&self) -> crate::BlockId {
                self.red_shulker_box_base
            }
            pub fn black_shulker_box(&self) -> crate::BlockId {
                self.black_shulker_box_base
            }
            pub fn white_glazed_terracotta(&self) -> crate::BlockId {
                self.white_glazed_terracotta_base
            }
            pub fn orange_glazed_terracotta(&self) -> crate::BlockId {
                self.orange_glazed_terracotta_base
            }
            pub fn magenta_glazed_terracotta(&self) -> crate::BlockId {
                self.magenta_glazed_terracotta_base
            }
            pub fn light_blue_glazed_terracotta(&self) -> crate::BlockId {
                self.light_blue_glazed_terracotta_base
            }
            pub fn yellow_glazed_terracotta(&self) -> crate::BlockId {
                self.yellow_glazed_terracotta_base
            }
            pub fn lime_glazed_terracotta(&self) -> crate::BlockId {
                self.lime_glazed_terracotta_base
            }
            pub fn pink_glazed_terracotta(&self) -> crate::BlockId {
                self.pink_glazed_terracotta_base
            }
            pub fn gray_glazed_terracotta(&self) -> crate::BlockId {
                self.gray_glazed_terracotta_base
            }
            pub fn light_gray_glazed_terracotta(&self) -> crate::BlockId {
                self.light_gray_glazed_terracotta_base
            }
            pub fn cyan_glazed_terracotta(&self) -> crate::BlockId {
                self.cyan_glazed_terracotta_base
            }
            pub fn purple_glazed_terracotta(&self) -> crate::BlockId {
                self.purple_glazed_terracotta_base
            }
            pub fn blue_glazed_terracotta(&self) -> crate::BlockId {
                self.blue_glazed_terracotta_base
            }
            pub fn brown_glazed_terracotta(&self) -> crate::BlockId {
                self.brown_glazed_terracotta_base
            }
            pub fn green_glazed_terracotta(&self) -> crate::BlockId {
                self.green_glazed_terracotta_base
            }
            pub fn red_glazed_terracotta(&self) -> crate::BlockId {
                self.red_glazed_terracotta_base
            }
            pub fn black_glazed_terracotta(&self) -> crate::BlockId {
                self.black_glazed_terracotta_base
            }
            pub fn white_concrete(&self) -> crate::BlockId {
                self.white_concrete_base
            }
            pub fn orange_concrete(&self) -> crate::BlockId {
                self.orange_concrete_base
            }
            pub fn magenta_concrete(&self) -> crate::BlockId {
                self.magenta_concrete_base
            }
            pub fn light_blue_concrete(&self) -> crate::BlockId {
                self.light_blue_concrete_base
            }
            pub fn yellow_concrete(&self) -> crate::BlockId {
                self.yellow_concrete_base
            }
            pub fn lime_concrete(&self) -> crate::BlockId {
                self.lime_concrete_base
            }
            pub fn pink_concrete(&self) -> crate::BlockId {
                self.pink_concrete_base
            }
            pub fn gray_concrete(&self) -> crate::BlockId {
                self.gray_concrete_base
            }
            pub fn light_gray_concrete(&self) -> crate::BlockId {
                self.light_gray_concrete_base
            }
            pub fn cyan_concrete(&self) -> crate::BlockId {
                self.cyan_concrete_base
            }
            pub fn purple_concrete(&self) -> crate::BlockId {
                self.purple_concrete_base
            }
            pub fn blue_concrete(&self) -> crate::BlockId {
                self.blue_concrete_base
            }
            pub fn brown_concrete(&self) -> crate::BlockId {
                self.brown_concrete_base
            }
            pub fn green_concrete(&self) -> crate::BlockId {
                self.green_concrete_base
            }
            pub fn red_concrete(&self) -> crate::BlockId {
                self.red_concrete_base
            }
            pub fn black_concrete(&self) -> crate::BlockId {
                self.black_concrete_base
            }
            pub fn white_concrete_powder(&self) -> crate::BlockId {
                self.white_concrete_powder_base
            }
            pub fn orange_concrete_powder(&self) -> crate::BlockId {
                self.orange_concrete_powder_base
            }
            pub fn magenta_concrete_powder(&self) -> crate::BlockId {
                self.magenta_concrete_powder_base
            }
            pub fn light_blue_concrete_powder(&self) -> crate::BlockId {
                self.light_blue_concrete_powder_base
            }
            pub fn yellow_concrete_powder(&self) -> crate::BlockId {
                self.yellow_concrete_powder_base
            }
            pub fn lime_concrete_powder(&self) -> crate::BlockId {
                self.lime_concrete_powder_base
            }
            pub fn pink_concrete_powder(&self) -> crate::BlockId {
                self.pink_concrete_powder_base
            }
            pub fn gray_concrete_powder(&self) -> crate::BlockId {
                self.gray_concrete_powder_base
            }
            pub fn light_gray_concrete_powder(&self) -> crate::BlockId {
                self.light_gray_concrete_powder_base
            }
            pub fn cyan_concrete_powder(&self) -> crate::BlockId {
                self.cyan_concrete_powder_base
            }
            pub fn purple_concrete_powder(&self) -> crate::BlockId {
                self.purple_concrete_powder_base
            }
            pub fn blue_concrete_powder(&self) -> crate::BlockId {
                self.blue_concrete_powder_base
            }
            pub fn brown_concrete_powder(&self) -> crate::BlockId {
                self.brown_concrete_powder_base
            }
            pub fn green_concrete_powder(&self) -> crate::BlockId {
                self.green_concrete_powder_base
            }
            pub fn red_concrete_powder(&self) -> crate::BlockId {
                self.red_concrete_powder_base
            }
            pub fn black_concrete_powder(&self) -> crate::BlockId {
                self.black_concrete_powder_base
            }
            pub fn kelp(&self) -> crate::BlockId {
                self.kelp_base
            }
            pub fn kelp_plant(&self) -> crate::BlockId {
                self.kelp_plant_base
            }
            pub fn dried_kelp_block(&self) -> crate::BlockId {
                self.dried_kelp_block_base
            }
            pub fn turtle_egg(&self) -> crate::BlockId {
                self.turtle_egg_base
            }
            pub fn dead_tube_coral_block(&self) -> crate::BlockId {
                self.dead_tube_coral_block_base
            }
            pub fn dead_brain_coral_block(&self) -> crate::BlockId {
                self.dead_brain_coral_block_base
            }
            pub fn dead_bubble_coral_block(&self) -> crate::BlockId {
                self.dead_bubble_coral_block_base
            }
            pub fn dead_fire_coral_block(&self) -> crate::BlockId {
                self.dead_fire_coral_block_base
            }
            pub fn dead_horn_coral_block(&self) -> crate::BlockId {
                self.dead_horn_coral_block_base
            }
            pub fn tube_coral_block(&self) -> crate::BlockId {
                self.tube_coral_block_base
            }
            pub fn brain_coral_block(&self) -> crate::BlockId {
                self.brain_coral_block_base
            }
            pub fn bubble_coral_block(&self) -> crate::BlockId {
                self.bubble_coral_block_base
            }
            pub fn fire_coral_block(&self) -> crate::BlockId {
                self.fire_coral_block_base
            }
            pub fn horn_coral_block(&self) -> crate::BlockId {
                self.horn_coral_block_base
            }
            pub fn dead_tube_coral(&self) -> crate::BlockId {
                self.dead_tube_coral_base
            }
            pub fn dead_brain_coral(&self) -> crate::BlockId {
                self.dead_brain_coral_base
            }
            pub fn dead_bubble_coral(&self) -> crate::BlockId {
                self.dead_bubble_coral_base
            }
            pub fn dead_fire_coral(&self) -> crate::BlockId {
                self.dead_fire_coral_base
            }
            pub fn dead_horn_coral(&self) -> crate::BlockId {
                self.dead_horn_coral_base
            }
            pub fn tube_coral(&self) -> crate::BlockId {
                self.tube_coral_base
            }
            pub fn brain_coral(&self) -> crate::BlockId {
                self.brain_coral_base
            }
            pub fn bubble_coral(&self) -> crate::BlockId {
                self.bubble_coral_base
            }
            pub fn fire_coral(&self) -> crate::BlockId {
                self.fire_coral_base
            }
            pub fn horn_coral(&self) -> crate::BlockId {
                self.horn_coral_base
            }
            pub fn dead_tube_coral_wall_fan(&self) -> crate::BlockId {
                self.dead_tube_coral_wall_fan_base
            }
            pub fn dead_brain_coral_wall_fan(&self) -> crate::BlockId {
                self.dead_brain_coral_wall_fan_base
            }
            pub fn dead_bubble_coral_wall_fan(&self) -> crate::BlockId {
                self.dead_bubble_coral_wall_fan_base
            }
            pub fn dead_fire_coral_wall_fan(&self) -> crate::BlockId {
                self.dead_fire_coral_wall_fan_base
            }
            pub fn dead_horn_coral_wall_fan(&self) -> crate::BlockId {
                self.dead_horn_coral_wall_fan_base
            }
            pub fn tube_coral_wall_fan(&self) -> crate::BlockId {
                self.tube_coral_wall_fan_base
            }
            pub fn brain_coral_wall_fan(&self) -> crate::BlockId {
                self.brain_coral_wall_fan_base
            }
            pub fn bubble_coral_wall_fan(&self) -> crate::BlockId {
                self.bubble_coral_wall_fan_base
            }
            pub fn fire_coral_wall_fan(&self) -> crate::BlockId {
                self.fire_coral_wall_fan_base
            }
            pub fn horn_coral_wall_fan(&self) -> crate::BlockId {
                self.horn_coral_wall_fan_base
            }
            pub fn dead_tube_coral_fan(&self) -> crate::BlockId {
                self.dead_tube_coral_fan_base
            }
            pub fn dead_brain_coral_fan(&self) -> crate::BlockId {
                self.dead_brain_coral_fan_base
            }
            pub fn dead_bubble_coral_fan(&self) -> crate::BlockId {
                self.dead_bubble_coral_fan_base
            }
            pub fn dead_fire_coral_fan(&self) -> crate::BlockId {
                self.dead_fire_coral_fan_base
            }
            pub fn dead_horn_coral_fan(&self) -> crate::BlockId {
                self.dead_horn_coral_fan_base
            }
            pub fn tube_coral_fan(&self) -> crate::BlockId {
                self.tube_coral_fan_base
            }
            pub fn brain_coral_fan(&self) -> crate::BlockId {
                self.brain_coral_fan_base
            }
            pub fn bubble_coral_fan(&self) -> crate::BlockId {
                self.bubble_coral_fan_base
            }
            pub fn fire_coral_fan(&self) -> crate::BlockId {
                self.fire_coral_fan_base
            }
            pub fn horn_coral_fan(&self) -> crate::BlockId {
                self.horn_coral_fan_base
            }
            pub fn sea_pickle(&self) -> crate::BlockId {
                self.sea_pickle_base
            }
            pub fn blue_ice(&self) -> crate::BlockId {
                self.blue_ice_base
            }
            pub fn conduit(&self) -> crate::BlockId {
                self.conduit_base
            }
            pub fn void_air(&self) -> crate::BlockId {
                self.void_air_base
            }
            pub fn cave_air(&self) -> crate::BlockId {
                self.cave_air_base
            }
            pub fn bubble_column(&self) -> crate::BlockId {
                self.bubble_column_base
            }
            pub fn structure_block(&self) -> crate::BlockId {
                self.structure_block_base
            }
        }
    };
}
