use feather_blocks::{
    AcaciaLeavesData, AcaciaLogAxis, AcaciaLogData, BirchLeavesData, BirchLogAxis, BirchLogData,
    Block, DarkOakLeavesData, DarkOakLogAxis, DarkOakLogData, JungleLeavesData, JungleLogAxis,
    JungleLogData, OakLeavesData, OakLogAxis, OakLogData, SpruceLeavesData, SpruceLogAxis,
    SpruceLogData,
};

/// Type of a tree.
#[derive(Debug, Clone, Copy)]
pub enum TreeType {
    Oak,
    DarkOak,
    Birch,
    Spruce,
    Acacia,
    Jungle,
}

impl TreeType {
    /// Returns the block corresponding to a log of this tree.
    pub fn log_block(self) -> Block {
        match self {
            TreeType::Oak => Block::OakLog(OakLogData {
                axis: OakLogAxis::Y,
            }),
            TreeType::DarkOak => Block::DarkOakLog(DarkOakLogData {
                axis: DarkOakLogAxis::Y,
            }),
            TreeType::Birch => Block::BirchLog(BirchLogData {
                axis: BirchLogAxis::Y,
            }),
            TreeType::Spruce => Block::SpruceLog(SpruceLogData {
                axis: SpruceLogAxis::Y,
            }),
            TreeType::Acacia => Block::AcaciaLog(AcaciaLogData {
                axis: AcaciaLogAxis::Y,
            }),
            TreeType::Jungle => Block::JungleLog(JungleLogData {
                axis: JungleLogAxis::Y,
            }),
        }
    }

    /// Returns the block corresponding to a leaf of this tree,
    /// at the given distance away from the nearest log.
    pub fn leaf_block(self, distance: i32) -> Block {
        match self {
            TreeType::Oak => Block::OakLeaves(OakLeavesData {
                distance,
                persistent: true,
            }),
            TreeType::DarkOak => Block::DarkOakLeaves(DarkOakLeavesData {
                distance,
                persistent: true,
            }),
            TreeType::Birch => Block::BirchLeaves(BirchLeavesData {
                distance,
                persistent: true,
            }),
            TreeType::Spruce => Block::SpruceLeaves(SpruceLeavesData {
                distance,
                persistent: true,
            }),
            TreeType::Acacia => Block::AcaciaLeaves(AcaciaLeavesData {
                distance,
                persistent: true,
            }),
            TreeType::Jungle => Block::JungleLeaves(JungleLeavesData {
                distance,
                persistent: true,
            }),
        }
    }
}
