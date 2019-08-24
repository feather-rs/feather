use feather_blocks::Block;
use feather_items::Item;

mod mappings;

pub trait ItemToBlock {
    fn to_block(self) -> Option<Block>;
}

impl ItemToBlock for Item {
    fn to_block(self) -> Option<Block> {
        mappings::item_to_block(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use feather_blocks::{AcaciaWoodAxis, AcaciaWoodData};

    #[test]
    fn test_basic() {
        let items = [
            Item::EnderPearl,
            Item::Stone,
            Item::AcaciaWood,
            Item::Cobblestone,
            Item::Bone,
        ];
        let blocks = [
            None,
            Some(Block::Stone),
            Some(Block::AcaciaWood(AcaciaWoodData {
                axis: AcaciaWoodAxis::Y,
            })),
            Some(Block::Cobblestone),
            None,
        ];

        for (item, block) in items.iter().zip(blocks.iter()) {
            assert_eq!(item.to_block(), block.clone());
        }
    }
}
