use crate::BlockKind;

impl BlockKind {
    pub fn is_air(self) -> bool {
        matches!(
            self,
            BlockKind::Air | BlockKind::VoidAir | BlockKind::CaveAir
        )
    }

    pub fn opaque(self) -> bool {
        !self.transparent()
    }

    pub fn fluid(self) -> bool {
        matches!(self, BlockKind::Water | BlockKind::Lava)
    }
}
