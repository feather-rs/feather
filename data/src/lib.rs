pub mod minecraft {
    pub mod lang {
        feather_data_macro::include_data!("minecraft/assets/minecraft/lang");
    }
    pub mod advancements {
        feather_data_macro::include_data!("minecraft/data/minecraft/advancements");
    }
    pub mod loot_tables {
        feather_data_macro::include_data!("minecraft/data/minecraft/loot_tables");
    }
    pub mod recipes {
        feather_data_macro::include_data!("minecraft/data/minecraft/recipes");
    }
    pub mod structures {
        feather_data_macro::include_data!("minecraft/data/minecraft/structures");
    }
    pub mod tags {
        feather_data_macro::include_data!("minecraft/data/minecraft/tags");
    }
    feather_data_macro::include_data!("minecraft/generated/reports");
}