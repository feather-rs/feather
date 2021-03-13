#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct EntityId {
    index: u32,
    generation: u32,
}

impl EntityId {
    pub fn to_bits(self) -> u64 {
        ((self.index as u64) << 32) | (self.generation as u64)
    }

    pub fn from_bits(bits: u64) -> Self {
        let index = (bits >> 32) as u32;
        let generation = bits as u32;
        Self { index, generation }
    }

    pub fn index(self) -> u32 {
        self.index
    }

    pub fn generation(self) -> u32 {
        self.generation
    }
}
