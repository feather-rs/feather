#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Block {
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

impl Block {
    pub fn block_state_id(self) -> u16 {
        match self {
            Block::Air => 0,
            Block::Stone => 1,
            Block::Granite => 2,
            Block::PolishedGranite => 3,
            Block::Diorite => 4,
            Block::PolishedDiorite => 5,
            Block::Andesite => 6,
            Block::PolishedAndesite => 7,
            Block::GrassBlock(data) => {
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
            0 => Block::Air,
            1 => Block::Stone,
            2 => Block::Granite,
            3 => Block::PolishedGranite,
            4 => Block::Diorite,
            5 => Block::PolishedDiorite,
            6 => Block::Andesite,
            7 => Block::PolishedAndesite,
            8 => Block::GrassBlock(GrassBlockData { snowy: true }),
            9 => Block::GrassBlock(GrassBlockData { snowy: false }),
            _ => panic!("Unknown block state ID {}", id),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct GrassBlockData {
    snowy: bool,
}
