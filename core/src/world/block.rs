#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum BlockType {
    Air,
    Stone,
    Granite,
    PolishedGranite,
    Diorite,
    PolishedDiorite,
    Andesite,
    PolishedAndesite,
    GrassBlock(GrassBlockData),
}

impl BlockType {
    pub fn block_state_id(&self) -> u16 {
        match self {
            BlockType::Air => 0,
            BlockType::Stone => 1,
            BlockType::Granite => 2,
            BlockType::PolishedGranite => 3,
            BlockType::Diorite => 4,
            BlockType::PolishedDiorite => 5,
            BlockType::Andesite => 6,
            BlockType::PolishedAndesite => 7,
            BlockType::GrassBlock(data) => {
                if data.snowy {
                    8
                } else {
                    9
                }
            }
        }
    }

    pub fn from_block_state_id(id: u16) -> Self {
        match id {
            0 => BlockType::Air,
            1 => BlockType::Stone,
            2 => BlockType::Granite,
            3 => BlockType::PolishedGranite,
            4 => BlockType::Diorite,
            5 => BlockType::PolishedDiorite,
            6 => BlockType::Andesite,
            7 => BlockType::PolishedAndesite,
            8 => BlockType::GrassBlock(GrassBlockData { snowy: true }),
            9 => BlockType::GrassBlock(GrassBlockData { snowy: false }),
            _ => panic!("Unknown block state ID {}", id),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct GrassBlockData {
    snowy: bool,
}
