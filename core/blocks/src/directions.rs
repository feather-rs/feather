use crate::{FacingCardinal, FacingCardinalAndDown, FacingCubic};

impl FacingCardinal {
    pub fn opposite(self) -> FacingCardinal {
        match self {
            FacingCardinal::East => FacingCardinal::West,
            FacingCardinal::West => FacingCardinal::East,
            FacingCardinal::North => FacingCardinal::South,
            FacingCardinal::South => FacingCardinal::North,
        }
    }

    pub fn right(self) -> FacingCardinal {
        match self {
            FacingCardinal::South => FacingCardinal::West,
            FacingCardinal::West => FacingCardinal::North,
            FacingCardinal::North => FacingCardinal::East,
            FacingCardinal::East => FacingCardinal::South,
        }
    }

    pub fn left(self) -> FacingCardinal {
        match self {
            FacingCardinal::South => FacingCardinal::East,
            FacingCardinal::West => FacingCardinal::South,
            FacingCardinal::North => FacingCardinal::West,
            FacingCardinal::East => FacingCardinal::North,
        }
    }

    pub fn is_horizontal(self) -> bool {
        true
    }

    pub fn to_facing_cardinal_and_down(self) -> FacingCardinalAndDown {
        match self {
            FacingCardinal::East => FacingCardinalAndDown::West,
            FacingCardinal::West => FacingCardinalAndDown::East,
            FacingCardinal::North => FacingCardinalAndDown::South,
            FacingCardinal::South => FacingCardinalAndDown::North,
        }
    }

    pub fn to_facing_cubic(self) -> FacingCubic {
        match self {
            FacingCardinal::East => FacingCubic::West,
            FacingCardinal::West => FacingCubic::East,
            FacingCardinal::North => FacingCubic::South,
            FacingCardinal::South => FacingCubic::North,
        }
    }
}

impl FacingCardinalAndDown {
    pub fn opposite(self) -> FacingCardinalAndDown {
        match self {
            FacingCardinalAndDown::East => FacingCardinalAndDown::West,
            FacingCardinalAndDown::West => FacingCardinalAndDown::East,
            FacingCardinalAndDown::North => FacingCardinalAndDown::South,
            FacingCardinalAndDown::South => FacingCardinalAndDown::North,
            FacingCardinalAndDown::Down => panic!("FacingCardinalAndDown::Down has no opposite"),
        }
    }

    pub fn is_horizontal(self) -> bool {
        self != FacingCardinalAndDown::Down
    }

    pub fn to_facing_cardinal(self) -> FacingCardinal {
        match self {
            FacingCardinalAndDown::East => FacingCardinal::West,
            FacingCardinalAndDown::West => FacingCardinal::East,
            FacingCardinalAndDown::North => FacingCardinal::South,
            FacingCardinalAndDown::South => FacingCardinal::North,
            FacingCardinalAndDown::Down => {
                panic!("FacingCardinalAndDown::Down cannot be converted to FacingCardinal")
            }
        }
    }

    pub fn to_facing_cubic(self) -> FacingCubic {
        match self {
            FacingCardinalAndDown::East => FacingCubic::West,
            FacingCardinalAndDown::West => FacingCubic::East,
            FacingCardinalAndDown::North => FacingCubic::South,
            FacingCardinalAndDown::South => FacingCubic::North,
            FacingCardinalAndDown::Down => FacingCubic::Down,
        }
    }
}

impl FacingCubic {
    pub fn opposite(self) -> FacingCubic {
        match self {
            FacingCubic::Up => FacingCubic::Down,
            FacingCubic::Down => FacingCubic::Up,
            FacingCubic::East => FacingCubic::West,
            FacingCubic::West => FacingCubic::East,
            FacingCubic::North => FacingCubic::South,
            FacingCubic::South => FacingCubic::North,
        }
    }

    pub fn is_horizontal(self) -> bool {
        match self {
            FacingCubic::Up | FacingCubic::Down => false,
            _ => true,
        }
    }

    pub fn to_facing_cardinal(self) -> FacingCardinal {
        match self {
            FacingCubic::East => FacingCardinal::West,
            FacingCubic::West => FacingCardinal::East,
            FacingCubic::North => FacingCardinal::South,
            FacingCubic::South => FacingCardinal::North,
            FacingCubic::Down => panic!("FacingCubic::Down cannot be converted to FacingCardinal"),
            FacingCubic::Up => panic!("FacingCubic::Up cannot be converted to FacingCardinal"),
        }
    }

    pub fn to_facing_cardinal_and_down(self) -> FacingCardinalAndDown {
        match self {
            FacingCubic::East => FacingCardinalAndDown::West,
            FacingCubic::West => FacingCardinalAndDown::East,
            FacingCubic::North => FacingCardinalAndDown::South,
            FacingCubic::South => FacingCardinalAndDown::North,
            FacingCubic::Down => FacingCardinalAndDown::Down,
            FacingCubic::Up => panic!("FacingCubic::Up cannot be converted to FacingCardinal"),
        }
    }
}
