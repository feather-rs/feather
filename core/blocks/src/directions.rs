use crate::{AxisXyz, FacingCardinal, FacingCardinalAndDown, FacingCubic};
use feather_util::BlockPosition;

impl FacingCardinal {
    pub fn opposite(self) -> FacingCardinal {
        match self {
            FacingCardinal::North => FacingCardinal::South,
            FacingCardinal::East => FacingCardinal::West,
            FacingCardinal::South => FacingCardinal::North,
            FacingCardinal::West => FacingCardinal::East,
        }
    }

    pub fn right(self) -> FacingCardinal {
        match self {
            FacingCardinal::North => FacingCardinal::East,
            FacingCardinal::East => FacingCardinal::South,
            FacingCardinal::South => FacingCardinal::West,
            FacingCardinal::West => FacingCardinal::North,
        }
    }

    pub fn left(self) -> FacingCardinal {
        match self {
            FacingCardinal::North => FacingCardinal::West,
            FacingCardinal::East => FacingCardinal::North,
            FacingCardinal::South => FacingCardinal::East,
            FacingCardinal::West => FacingCardinal::South,
        }
    }

    pub fn is_horizontal(self) -> bool {
        true
    }

    pub fn to_facing_cardinal_and_down(self) -> FacingCardinalAndDown {
        match self {
            FacingCardinal::North => FacingCardinalAndDown::North,
            FacingCardinal::East => FacingCardinalAndDown::East,
            FacingCardinal::South => FacingCardinalAndDown::South,
            FacingCardinal::West => FacingCardinalAndDown::West,
        }
    }

    pub fn to_facing_cubic(self) -> FacingCubic {
        match self {
            FacingCardinal::North => FacingCubic::North,
            FacingCardinal::East => FacingCubic::East,
            FacingCardinal::South => FacingCubic::South,
            FacingCardinal::West => FacingCubic::West,
        }
    }

    pub fn axis(self) -> AxisXyz {
        self.to_facing_cubic().axis()
    }

    pub fn offset(self) -> BlockPosition {
        self.to_facing_cubic().offset()
    }
}

impl FacingCardinalAndDown {
    pub fn opposite(self) -> Option<FacingCardinalAndDown> {
        match self {
            FacingCardinalAndDown::North => Some(FacingCardinalAndDown::South),
            FacingCardinalAndDown::East => Some(FacingCardinalAndDown::West),
            FacingCardinalAndDown::South => Some(FacingCardinalAndDown::North),
            FacingCardinalAndDown::West => Some(FacingCardinalAndDown::East),
            _ => None,
        }
    }

    pub fn is_horizontal(self) -> bool {
        self != FacingCardinalAndDown::Down
    }

    pub fn to_facing_cardinal(self) -> Option<FacingCardinal> {
        match self {
            FacingCardinalAndDown::North => Some(FacingCardinal::North),
            FacingCardinalAndDown::East => Some(FacingCardinal::East),
            FacingCardinalAndDown::South => Some(FacingCardinal::South),
            FacingCardinalAndDown::West => Some(FacingCardinal::West),
            _ => None,
        }
    }

    pub fn to_facing_cubic(self) -> FacingCubic {
        match self {
            FacingCardinalAndDown::North => FacingCubic::North,
            FacingCardinalAndDown::East => FacingCubic::East,
            FacingCardinalAndDown::South => FacingCubic::South,
            FacingCardinalAndDown::West => FacingCubic::West,
            FacingCardinalAndDown::Down => FacingCubic::Down,
        }
    }

    pub fn axis(self) -> AxisXyz {
        self.to_facing_cubic().axis()
    }

    pub fn offset(self) -> BlockPosition {
        self.to_facing_cubic().offset()
    }
}

impl FacingCubic {
    pub fn opposite(self) -> FacingCubic {
        match self {
            FacingCubic::North => FacingCubic::South,
            FacingCubic::East => FacingCubic::West,
            FacingCubic::South => FacingCubic::North,
            FacingCubic::West => FacingCubic::East,
            FacingCubic::Up => FacingCubic::Down,
            FacingCubic::Down => FacingCubic::Up,
        }
    }

    pub fn is_horizontal(self) -> bool {
        match self {
            FacingCubic::Up | FacingCubic::Down => false,
            _ => true,
        }
    }

    pub fn to_facing_cardinal(self) -> Option<FacingCardinal> {
        match self {
            FacingCubic::North => Some(FacingCardinal::North),
            FacingCubic::East => Some(FacingCardinal::East),
            FacingCubic::South => Some(FacingCardinal::South),
            FacingCubic::West => Some(FacingCardinal::West),
            _ => None,
        }
    }

    pub fn to_facing_cardinal_and_down(self) -> Option<FacingCardinalAndDown> {
        match self {
            FacingCubic::North => Some(FacingCardinalAndDown::North),
            FacingCubic::East => Some(FacingCardinalAndDown::East),
            FacingCubic::South => Some(FacingCardinalAndDown::South),
            FacingCubic::West => Some(FacingCardinalAndDown::West),
            FacingCubic::Down => Some(FacingCardinalAndDown::Down),
            _ => None,
        }
    }

    pub fn axis(self) -> AxisXyz {
        match self {
            FacingCubic::East | FacingCubic::West => AxisXyz::X,
            FacingCubic::Up | FacingCubic::Down => AxisXyz::Y,
            FacingCubic::North | FacingCubic::South => AxisXyz::Z,
        }
    }

    pub fn offset(self) -> BlockPosition {
        match self {
            FacingCubic::North => BlockPosition { x: 0, y: 0, z: -1 },
            FacingCubic::East => BlockPosition { x: 1, y: 0, z: 0 },
            FacingCubic::South => BlockPosition { x: 0, y: 0, z: 1 },
            FacingCubic::West => BlockPosition { x: -1, y: 0, z: 0 },
            FacingCubic::Up => BlockPosition { x: 0, y: 1, z: 0 },
            FacingCubic::Down => BlockPosition { x: 0, y: -1, z: 0 },
        }
    }
}
