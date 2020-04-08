use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::convert::TryFrom;
use strum::*;
use strum_macros::*;

#[derive(Copy, Clone, Debug, EnumIter, EnumCount, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum ChestFacing {
    North,
    South,
    East,
    West,
}

impl TryFrom<u16> for ChestFacing {
    type Error = anyhow::Error;
    fn try_from(x: u16) -> anyhow::Result<Self> {
        match x {
            0 => Ok(ChestFacing::North),
            1 => Ok(ChestFacing::South),
            2 => Ok(ChestFacing::East),
            3 => Ok(ChestFacing::West),
            x => Err(anyhow::anyhow!("invalid ChestFacing value {}", x)),
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, EnumCount, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum ChestKind {
    Single,
    Left,
    Right,
}

impl TryFrom<u16> for ChestKind {
    type Error = anyhow::Error;

    fn try_from(value: u16) -> anyhow::Result<Self> {
        match value {
            0 => Ok(ChestKind::Single),
            1 => Ok(ChestKind::Left),
            2 => Ok(ChestKind::Right),
            x => Err(anyhow::anyhow!("invalid ChestKind value {}", x)),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Chest {
    pub facing: ChestFacing,
    pub kind: ChestKind,
    pub waterlogged: bool,
}

impl Chest {
    pub fn possible_states() -> Vec<Self> {
        ChestFacing::iter()
            .flat_map(|state| std::iter::repeat(state).zip(ChestKind::iter()))
            .flat_map(|state| std::iter::repeat(state).zip([true, false].iter().copied()))
            .map(|((facing, kind), waterlogged)| Chest {
                facing,
                kind,
                waterlogged,
            })
            .collect()
    }

    pub fn id_offset(self) -> u16 {
        let facing_stride = ChestKind::count() as u16 * 2;
        let kind_stride = 2;
        let waterlogged_stride = 1;

        self.facing as u16 * facing_stride
            + self.kind as u16 * kind_stride
            + self.waterlogged as u16 * waterlogged_stride
    }

    pub fn from_id_offset(mut offset: u16) -> anyhow::Result<Self> {
        let facing_stride = ChestKind::count() as u16 * 2;
        let kind_stride = 2;
        let waterlogged_stride = 1;

        let facing_sub = offset / facing_stride;
        offset -= facing_sub * facing_stride;
        let facing = ChestFacing::try_from(facing_sub)?;

        let kind_sub = offset / kind_stride;
        offset -= kind_sub * kind_stride;
        let kind = ChestKind::try_from(kind_sub)?;

        let waterlogged_sub = offset / waterlogged_stride;
        offset -= waterlogged_sub * waterlogged_stride;
        let waterlogged = match waterlogged_sub {
            1 => true,
            0 => false,
            x => anyhow::bail!("invalid bool value {}", x),
        };
        debug_assert_eq!(offset, 0);

        Ok(Self {
            facing,
            kind,
            waterlogged,
        })
    }

    pub fn minecraft_id_offset_v1_13_2(self) -> u16 {
        static MAP: Lazy<HashMap<Chest, u16>> = Lazy::new(|| {
            Chest::possible_states()
                .into_iter()
                .zip([0, 1, 2, 3, 4, 5].iter().copied())
                .collect()
        });

        *MAP.get(&self).unwrap()
    }

    pub fn apply_waterlogged(offset: u16, waterlogged: bool) -> u16 {
        let mut state = Self::from_id_offset(offset).unwrap();
        state.waterlogged = waterlogged;
        state.id_offset()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn chest() {
        let mut offsets = HashSet::new();
        for state in Chest::possible_states() {
            let offset = state.id_offset();
            assert_eq!(Chest::from_id_offset(offset).unwrap(), state);
            assert!(offsets.insert(offset));
        }
    }
}
