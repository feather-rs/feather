use crate::BlockKind;
use serde::Deserialize;
use std::convert::TryFrom;
#[derive(Debug, Deserialize)]
pub struct BlockTable {
    age: Vec<(u16, u16)>,
    attached: Vec<(u16, u16)>,
    axis_xyz: Vec<(u16, u16)>,
    axis_xz: Vec<(u16, u16)>,
    bites: Vec<(u16, u16)>,
    chest_kind: Vec<(u16, u16)>,
    comparator_mode: Vec<(u16, u16)>,
    conditional: Vec<(u16, u16)>,
    delay: Vec<(u16, u16)>,
    disarmed: Vec<(u16, u16)>,
    distance: Vec<(u16, u16)>,
    down: Vec<(u16, u16)>,
    drag: Vec<(u16, u16)>,
    east_connected: Vec<(u16, u16)>,
    east_wire: Vec<(u16, u16)>,
    eggs: Vec<(u16, u16)>,
    enabled: Vec<(u16, u16)>,
    extended: Vec<(u16, u16)>,
    eye: Vec<(u16, u16)>,
    face: Vec<(u16, u16)>,
    facing_cardinal: Vec<(u16, u16)>,
    facing_cardinal_and_down: Vec<(u16, u16)>,
    facing_cubic: Vec<(u16, u16)>,
    half: Vec<(u16, u16)>,
    has_bottle_0: Vec<(u16, u16)>,
    has_bottle_1: Vec<(u16, u16)>,
    has_bottle_2: Vec<(u16, u16)>,
    has_record: Vec<(u16, u16)>,
    hatch: Vec<(u16, u16)>,
    hinge: Vec<(u16, u16)>,
    in_wall: Vec<(u16, u16)>,
    instrument: Vec<(u16, u16)>,
    inverted: Vec<(u16, u16)>,
    layers: Vec<(u16, u16)>,
    level: Vec<(u16, u16)>,
    lit: Vec<(u16, u16)>,
    locked: Vec<(u16, u16)>,
    moisture: Vec<(u16, u16)>,
    north_connected: Vec<(u16, u16)>,
    north_wire: Vec<(u16, u16)>,
    note: Vec<(u16, u16)>,
    occupied: Vec<(u16, u16)>,
    open: Vec<(u16, u16)>,
    part: Vec<(u16, u16)>,
    persistent: Vec<(u16, u16)>,
    pickles: Vec<(u16, u16)>,
    piston_kind: Vec<(u16, u16)>,
    power: Vec<(u16, u16)>,
    powered: Vec<(u16, u16)>,
    powered_rail_shape: Vec<(u16, u16)>,
    rail_shape: Vec<(u16, u16)>,
    rotation: Vec<(u16, u16)>,
    short: Vec<(u16, u16)>,
    slab_kind: Vec<(u16, u16)>,
    snowy: Vec<(u16, u16)>,
    south_connected: Vec<(u16, u16)>,
    south_wire: Vec<(u16, u16)>,
    stage: Vec<(u16, u16)>,
    stairs_shape: Vec<(u16, u16)>,
    structure_block_mode: Vec<(u16, u16)>,
    triggered: Vec<(u16, u16)>,
    unstable: Vec<(u16, u16)>,
    up: Vec<(u16, u16)>,
    waterlogged: Vec<(u16, u16)>,
    west_connected: Vec<(u16, u16)>,
    west_wire: Vec<(u16, u16)>,
}
impl BlockTable {
    #[doc = "Retrieves the `age` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn age(&self, kind: BlockKind, state: u16) -> Option<i32> {
        let (offset_coefficient, stride) = self.age[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some({ x as i32 })
    }
    #[doc = "Updates the state value for the given block kind such that its `age` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_age(&self, kind: BlockKind, state: u16, value: i32) -> Option<u16> {
        let (offset_coefficient, stride) = self.age[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let base = state % offset_coefficient;
        let multiplier = state / offset_coefficient;
        let mut new = (value as u16 - (base / stride)) * stride + base;
        new += multiplier * offset_coefficient;
        Some(new)
    }
    #[doc = "Retrieves the `attached` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn attached(&self, kind: BlockKind, state: u16) -> Option<bool> {
        let (offset_coefficient, stride) = self.attached[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(if x == 0 { false } else { true })
    }
    #[doc = "Updates the state value for the given block kind such that its `attached` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_attached(&self, kind: BlockKind, state: u16, value: bool) -> Option<u16> {
        let (offset_coefficient, stride) = self.attached[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let base = state % offset_coefficient;
        let multiplier = state / offset_coefficient;
        let mut new = (value as u16 - (base / stride)) * stride + base;
        new += multiplier * offset_coefficient;
        Some(new)
    }
    #[doc = "Retrieves the `axis_xyz` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn axis_xyz(&self, kind: BlockKind, state: u16) -> Option<AxisXyz> {
        let (offset_coefficient, stride) = self.axis_xyz[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(AxisXyz::try_from(x).expect("invalid block state"))
    }
    #[doc = "Updates the state value for the given block kind such that its `axis_xyz` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_axis_xyz(&self, kind: BlockKind, state: u16, value: AxisXyz) -> Option<u16> {
        let (offset_coefficient, stride) = self.axis_xyz[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let base = state % offset_coefficient;
        let multiplier = state / offset_coefficient;
        let mut new = (value as u16 - (base / stride)) * stride + base;
        new += multiplier * offset_coefficient;
        Some(new)
    }
    #[doc = "Retrieves the `axis_xz` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn axis_xz(&self, kind: BlockKind, state: u16) -> Option<AxisXz> {
        let (offset_coefficient, stride) = self.axis_xz[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(AxisXz::try_from(x).expect("invalid block state"))
    }
    #[doc = "Updates the state value for the given block kind such that its `axis_xz` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_axis_xz(&self, kind: BlockKind, state: u16, value: AxisXz) -> Option<u16> {
        let (offset_coefficient, stride) = self.axis_xz[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let base = state % offset_coefficient;
        let multiplier = state / offset_coefficient;
        let mut new = (value as u16 - (base / stride)) * stride + base;
        new += multiplier * offset_coefficient;
        Some(new)
    }
    #[doc = "Retrieves the `bites` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn bites(&self, kind: BlockKind, state: u16) -> Option<i32> {
        let (offset_coefficient, stride) = self.bites[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some({ x as i32 })
    }
    #[doc = "Updates the state value for the given block kind such that its `bites` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_bites(&self, kind: BlockKind, state: u16, value: i32) -> Option<u16> {
        let (offset_coefficient, stride) = self.bites[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let base = state % offset_coefficient;
        let multiplier = state / offset_coefficient;
        let mut new = (value as u16 - (base / stride)) * stride + base;
        new += multiplier * offset_coefficient;
        Some(new)
    }
    #[doc = "Retrieves the `chest_kind` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn chest_kind(&self, kind: BlockKind, state: u16) -> Option<ChestKind> {
        let (offset_coefficient, stride) = self.chest_kind[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(ChestKind::try_from(x).expect("invalid block state"))
    }
    #[doc = "Updates the state value for the given block kind such that its `chest_kind` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_chest_kind(&self, kind: BlockKind, state: u16, value: ChestKind) -> Option<u16> {
        let (offset_coefficient, stride) = self.chest_kind[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let base = state % offset_coefficient;
        let multiplier = state / offset_coefficient;
        let mut new = (value as u16 - (base / stride)) * stride + base;
        new += multiplier * offset_coefficient;
        Some(new)
    }
    #[doc = "Retrieves the `comparator_mode` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn comparator_mode(&self, kind: BlockKind, state: u16) -> Option<ComparatorMode> {
        let (offset_coefficient, stride) = self.comparator_mode[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(ComparatorMode::try_from(x).expect("invalid block state"))
    }
    #[doc = "Updates the state value for the given block kind such that its `comparator_mode` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_comparator_mode(
        &self,
        kind: BlockKind,
        state: u16,
        value: ComparatorMode,
    ) -> Option<u16> {
        let (offset_coefficient, stride) = self.comparator_mode[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let base = state % offset_coefficient;
        let multiplier = state / offset_coefficient;
        let mut new = (value as u16 - (base / stride)) * stride + base;
        new += multiplier * offset_coefficient;
        Some(new)
    }
    #[doc = "Retrieves the `conditional` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn conditional(&self, kind: BlockKind, state: u16) -> Option<bool> {
        let (offset_coefficient, stride) = self.conditional[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(if x == 0 { false } else { true })
    }
    #[doc = "Updates the state value for the given block kind such that its `conditional` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_conditional(&self, kind: BlockKind, state: u16, value: bool) -> Option<u16> {
        let (offset_coefficient, stride) = self.conditional[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let base = state % offset_coefficient;
        let multiplier = state / offset_coefficient;
        let mut new = (value as u16 - (base / stride)) * stride + base;
        new += multiplier * offset_coefficient;
        Some(new)
    }
    #[doc = "Retrieves the `delay` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn delay(&self, kind: BlockKind, state: u16) -> Option<i32> {
        let (offset_coefficient, stride) = self.delay[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some({ x as i32 })
    }
    #[doc = "Updates the state value for the given block kind such that its `delay` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_delay(&self, kind: BlockKind, state: u16, value: i32) -> Option<u16> {
        let (offset_coefficient, stride) = self.delay[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let base = state % offset_coefficient;
        let multiplier = state / offset_coefficient;
        let mut new = (value as u16 - (base / stride)) * stride + base;
        new += multiplier * offset_coefficient;
        Some(new)
    }
    #[doc = "Retrieves the `disarmed` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn disarmed(&self, kind: BlockKind, state: u16) -> Option<bool> {
        let (offset_coefficient, stride) = self.disarmed[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(if x == 0 { false } else { true })
    }
    #[doc = "Updates the state value for the given block kind such that its `disarmed` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_disarmed(&self, kind: BlockKind, state: u16, value: bool) -> Option<u16> {
        let (offset_coefficient, stride) = self.disarmed[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let base = state % offset_coefficient;
        let multiplier = state / offset_coefficient;
        let mut new = (value as u16 - (base / stride)) * stride + base;
        new += multiplier * offset_coefficient;
        Some(new)
    }
    #[doc = "Retrieves the `distance` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn distance(&self, kind: BlockKind, state: u16) -> Option<i32> {
        let (offset_coefficient, stride) = self.distance[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some({ x as i32 })
    }
    #[doc = "Updates the state value for the given block kind such that its `distance` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_distance(&self, kind: BlockKind, state: u16, value: i32) -> Option<u16> {
        let (offset_coefficient, stride) = self.distance[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let base = state % offset_coefficient;
        let multiplier = state / offset_coefficient;
        let mut new = (value as u16 - (base / stride)) * stride + base;
        new += multiplier * offset_coefficient;
        Some(new)
    }
    #[doc = "Retrieves the `down` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn down(&self, kind: BlockKind, state: u16) -> Option<bool> {
        let (offset_coefficient, stride) = self.down[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(if x == 0 { false } else { true })
    }
    #[doc = "Updates the state value for the given block kind such that its `down` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_down(&self, kind: BlockKind, state: u16, value: bool) -> Option<u16> {
        let (offset_coefficient, stride) = self.down[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let base = state % offset_coefficient;
        let multiplier = state / offset_coefficient;
        let mut new = (value as u16 - (base / stride)) * stride + base;
        new += multiplier * offset_coefficient;
        Some(new)
    }
    #[doc = "Retrieves the `drag` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn drag(&self, kind: BlockKind, state: u16) -> Option<bool> {
        let (offset_coefficient, stride) = self.drag[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(if x == 0 { false } else { true })
    }
    #[doc = "Updates the state value for the given block kind such that its `drag` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_drag(&self, kind: BlockKind, state: u16, value: bool) -> Option<u16> {
        let (offset_coefficient, stride) = self.drag[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let base = state % offset_coefficient;
        let multiplier = state / offset_coefficient;
        let mut new = (value as u16 - (base / stride)) * stride + base;
        new += multiplier * offset_coefficient;
        Some(new)
    }
    #[doc = "Retrieves the `east_connected` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn east_connected(&self, kind: BlockKind, state: u16) -> Option<bool> {
        let (offset_coefficient, stride) = self.east_connected[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(if x == 0 { false } else { true })
    }
    #[doc = "Updates the state value for the given block kind such that its `east_connected` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_east_connected(&self, kind: BlockKind, state: u16, value: bool) -> Option<u16> {
        let (offset_coefficient, stride) = self.east_connected[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let base = state % offset_coefficient;
        let multiplier = state / offset_coefficient;
        let mut new = (value as u16 - (base / stride)) * stride + base;
        new += multiplier * offset_coefficient;
        Some(new)
    }
    #[doc = "Retrieves the `east_wire` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn east_wire(&self, kind: BlockKind, state: u16) -> Option<EastWire> {
        let (offset_coefficient, stride) = self.east_wire[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(EastWire::try_from(x).expect("invalid block state"))
    }
    #[doc = "Updates the state value for the given block kind such that its `east_wire` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_east_wire(&self, kind: BlockKind, state: u16, value: EastWire) -> Option<u16> {
        let (offset_coefficient, stride) = self.east_wire[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let base = state % offset_coefficient;
        let multiplier = state / offset_coefficient;
        let mut new = (value as u16 - (base / stride)) * stride + base;
        new += multiplier * offset_coefficient;
        Some(new)
    }
    #[doc = "Retrieves the `eggs` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn eggs(&self, kind: BlockKind, state: u16) -> Option<i32> {
        let (offset_coefficient, stride) = self.eggs[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some({ x as i32 })
    }
    #[doc = "Updates the state value for the given block kind such that its `eggs` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_eggs(&self, kind: BlockKind, state: u16, value: i32) -> Option<u16> {
        let (offset_coefficient, stride) = self.eggs[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let base = state % offset_coefficient;
        let multiplier = state / offset_coefficient;
        let mut new = (value as u16 - (base / stride)) * stride + base;
        new += multiplier * offset_coefficient;
        Some(new)
    }
    #[doc = "Retrieves the `enabled` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn enabled(&self, kind: BlockKind, state: u16) -> Option<bool> {
        let (offset_coefficient, stride) = self.enabled[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(if x == 0 { false } else { true })
    }
    #[doc = "Updates the state value for the given block kind such that its `enabled` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_enabled(&self, kind: BlockKind, state: u16, value: bool) -> Option<u16> {
        let (offset_coefficient, stride) = self.enabled[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let base = state % offset_coefficient;
        let multiplier = state / offset_coefficient;
        let mut new = (value as u16 - (base / stride)) * stride + base;
        new += multiplier * offset_coefficient;
        Some(new)
    }
    #[doc = "Retrieves the `extended` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn extended(&self, kind: BlockKind, state: u16) -> Option<bool> {
        let (offset_coefficient, stride) = self.extended[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(if x == 0 { false } else { true })
    }
    #[doc = "Updates the state value for the given block kind such that its `extended` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_extended(&self, kind: BlockKind, state: u16, value: bool) -> Option<u16> {
        let (offset_coefficient, stride) = self.extended[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let base = state % offset_coefficient;
        let multiplier = state / offset_coefficient;
        let mut new = (value as u16 - (base / stride)) * stride + base;
        new += multiplier * offset_coefficient;
        Some(new)
    }
    #[doc = "Retrieves the `eye` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn eye(&self, kind: BlockKind, state: u16) -> Option<bool> {
        let (offset_coefficient, stride) = self.eye[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(if x == 0 { false } else { true })
    }
    #[doc = "Updates the state value for the given block kind such that its `eye` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_eye(&self, kind: BlockKind, state: u16, value: bool) -> Option<u16> {
        let (offset_coefficient, stride) = self.eye[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let base = state % offset_coefficient;
        let multiplier = state / offset_coefficient;
        let mut new = (value as u16 - (base / stride)) * stride + base;
        new += multiplier * offset_coefficient;
        Some(new)
    }
    #[doc = "Retrieves the `face` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn face(&self, kind: BlockKind, state: u16) -> Option<Face> {
        let (offset_coefficient, stride) = self.face[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(Face::try_from(x).expect("invalid block state"))
    }
    #[doc = "Updates the state value for the given block kind such that its `face` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_face(&self, kind: BlockKind, state: u16, value: Face) -> Option<u16> {
        let (offset_coefficient, stride) = self.face[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let base = state % offset_coefficient;
        let multiplier = state / offset_coefficient;
        let mut new = (value as u16 - (base / stride)) * stride + base;
        new += multiplier * offset_coefficient;
        Some(new)
    }
    #[doc = "Retrieves the `facing_cardinal` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn facing_cardinal(&self, kind: BlockKind, state: u16) -> Option<FacingCardinal> {
        let (offset_coefficient, stride) = self.facing_cardinal[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(FacingCardinal::try_from(x).expect("invalid block state"))
    }
    #[doc = "Updates the state value for the given block kind such that its `facing_cardinal` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_facing_cardinal(
        &self,
        kind: BlockKind,
        state: u16,
        value: FacingCardinal,
    ) -> Option<u16> {
        let (offset_coefficient, stride) = self.facing_cardinal[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let base = state % offset_coefficient;
        let multiplier = state / offset_coefficient;
        let mut new = (value as u16 - (base / stride)) * stride + base;
        new += multiplier * offset_coefficient;
        Some(new)
    }
    #[doc = "Retrieves the `facing_cardinal_and_down` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn facing_cardinal_and_down(
        &self,
        kind: BlockKind,
        state: u16,
    ) -> Option<FacingCardinalAndDown> {
        let (offset_coefficient, stride) = self.facing_cardinal_and_down[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(FacingCardinalAndDown::try_from(x).expect("invalid block state"))
    }
    #[doc = "Updates the state value for the given block kind such that its `facing_cardinal_and_down` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_facing_cardinal_and_down(
        &self,
        kind: BlockKind,
        state: u16,
        value: FacingCardinalAndDown,
    ) -> Option<u16> {
        let (offset_coefficient, stride) = self.facing_cardinal_and_down[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let base = state % offset_coefficient;
        let multiplier = state / offset_coefficient;
        let mut new = (value as u16 - (base / stride)) * stride + base;
        new += multiplier * offset_coefficient;
        Some(new)
    }
    #[doc = "Retrieves the `facing_cubic` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn facing_cubic(&self, kind: BlockKind, state: u16) -> Option<FacingCubic> {
        let (offset_coefficient, stride) = self.facing_cubic[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(FacingCubic::try_from(x).expect("invalid block state"))
    }
    #[doc = "Updates the state value for the given block kind such that its `facing_cubic` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_facing_cubic(&self, kind: BlockKind, state: u16, value: FacingCubic) -> Option<u16> {
        let (offset_coefficient, stride) = self.facing_cubic[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let base = state % offset_coefficient;
        let multiplier = state / offset_coefficient;
        let mut new = (value as u16 - (base / stride)) * stride + base;
        new += multiplier * offset_coefficient;
        Some(new)
    }
    #[doc = "Retrieves the `half` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn half(&self, kind: BlockKind, state: u16) -> Option<Half> {
        let (offset_coefficient, stride) = self.half[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(Half::try_from(x).expect("invalid block state"))
    }
    #[doc = "Updates the state value for the given block kind such that its `half` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_half(&self, kind: BlockKind, state: u16, value: Half) -> Option<u16> {
        let (offset_coefficient, stride) = self.half[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let base = state % offset_coefficient;
        let multiplier = state / offset_coefficient;
        let mut new = (value as u16 - (base / stride)) * stride + base;
        new += multiplier * offset_coefficient;
        Some(new)
    }
    #[doc = "Retrieves the `has_bottle_0` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn has_bottle_0(&self, kind: BlockKind, state: u16) -> Option<bool> {
        let (offset_coefficient, stride) = self.has_bottle_0[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(if x == 0 { false } else { true })
    }
    #[doc = "Updates the state value for the given block kind such that its `has_bottle_0` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_has_bottle_0(&self, kind: BlockKind, state: u16, value: bool) -> Option<u16> {
        let (offset_coefficient, stride) = self.has_bottle_0[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let base = state % offset_coefficient;
        let multiplier = state / offset_coefficient;
        let mut new = (value as u16 - (base / stride)) * stride + base;
        new += multiplier * offset_coefficient;
        Some(new)
    }
    #[doc = "Retrieves the `has_bottle_1` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn has_bottle_1(&self, kind: BlockKind, state: u16) -> Option<bool> {
        let (offset_coefficient, stride) = self.has_bottle_1[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(if x == 0 { false } else { true })
    }
    #[doc = "Updates the state value for the given block kind such that its `has_bottle_1` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_has_bottle_1(&self, kind: BlockKind, state: u16, value: bool) -> Option<u16> {
        let (offset_coefficient, stride) = self.has_bottle_1[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let base = state % offset_coefficient;
        let multiplier = state / offset_coefficient;
        let mut new = (value as u16 - (base / stride)) * stride + base;
        new += multiplier * offset_coefficient;
        Some(new)
    }
    #[doc = "Retrieves the `has_bottle_2` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn has_bottle_2(&self, kind: BlockKind, state: u16) -> Option<bool> {
        let (offset_coefficient, stride) = self.has_bottle_2[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(if x == 0 { false } else { true })
    }
    #[doc = "Updates the state value for the given block kind such that its `has_bottle_2` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_has_bottle_2(&self, kind: BlockKind, state: u16, value: bool) -> Option<u16> {
        let (offset_coefficient, stride) = self.has_bottle_2[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let base = state % offset_coefficient;
        let multiplier = state / offset_coefficient;
        let mut new = (value as u16 - (base / stride)) * stride + base;
        new += multiplier * offset_coefficient;
        Some(new)
    }
    #[doc = "Retrieves the `has_record` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn has_record(&self, kind: BlockKind, state: u16) -> Option<bool> {
        let (offset_coefficient, stride) = self.has_record[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(if x == 0 { false } else { true })
    }
    #[doc = "Updates the state value for the given block kind such that its `has_record` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_has_record(&self, kind: BlockKind, state: u16, value: bool) -> Option<u16> {
        let (offset_coefficient, stride) = self.has_record[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let base = state % offset_coefficient;
        let multiplier = state / offset_coefficient;
        let mut new = (value as u16 - (base / stride)) * stride + base;
        new += multiplier * offset_coefficient;
        Some(new)
    }
    #[doc = "Retrieves the `hatch` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn hatch(&self, kind: BlockKind, state: u16) -> Option<i32> {
        let (offset_coefficient, stride) = self.hatch[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some({ x as i32 })
    }
    #[doc = "Updates the state value for the given block kind such that its `hatch` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_hatch(&self, kind: BlockKind, state: u16, value: i32) -> Option<u16> {
        let (offset_coefficient, stride) = self.hatch[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let base = state % offset_coefficient;
        let multiplier = state / offset_coefficient;
        let mut new = (value as u16 - (base / stride)) * stride + base;
        new += multiplier * offset_coefficient;
        Some(new)
    }
    #[doc = "Retrieves the `hinge` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn hinge(&self, kind: BlockKind, state: u16) -> Option<Hinge> {
        let (offset_coefficient, stride) = self.hinge[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(Hinge::try_from(x).expect("invalid block state"))
    }
    #[doc = "Updates the state value for the given block kind such that its `hinge` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_hinge(&self, kind: BlockKind, state: u16, value: Hinge) -> Option<u16> {
        let (offset_coefficient, stride) = self.hinge[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let base = state % offset_coefficient;
        let multiplier = state / offset_coefficient;
        let mut new = (value as u16 - (base / stride)) * stride + base;
        new += multiplier * offset_coefficient;
        Some(new)
    }
    #[doc = "Retrieves the `in_wall` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn in_wall(&self, kind: BlockKind, state: u16) -> Option<bool> {
        let (offset_coefficient, stride) = self.in_wall[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(if x == 0 { false } else { true })
    }
    #[doc = "Updates the state value for the given block kind such that its `in_wall` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_in_wall(&self, kind: BlockKind, state: u16, value: bool) -> Option<u16> {
        let (offset_coefficient, stride) = self.in_wall[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let base = state % offset_coefficient;
        let multiplier = state / offset_coefficient;
        let mut new = (value as u16 - (base / stride)) * stride + base;
        new += multiplier * offset_coefficient;
        Some(new)
    }
    #[doc = "Retrieves the `instrument` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn instrument(&self, kind: BlockKind, state: u16) -> Option<Instrument> {
        let (offset_coefficient, stride) = self.instrument[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(Instrument::try_from(x).expect("invalid block state"))
    }
    #[doc = "Updates the state value for the given block kind such that its `instrument` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_instrument(&self, kind: BlockKind, state: u16, value: Instrument) -> Option<u16> {
        let (offset_coefficient, stride) = self.instrument[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let base = state % offset_coefficient;
        let multiplier = state / offset_coefficient;
        let mut new = (value as u16 - (base / stride)) * stride + base;
        new += multiplier * offset_coefficient;
        Some(new)
    }
    #[doc = "Retrieves the `inverted` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn inverted(&self, kind: BlockKind, state: u16) -> Option<bool> {
        let (offset_coefficient, stride) = self.inverted[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(if x == 0 { false } else { true })
    }
    #[doc = "Updates the state value for the given block kind such that its `inverted` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_inverted(&self, kind: BlockKind, state: u16, value: bool) -> Option<u16> {
        let (offset_coefficient, stride) = self.inverted[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let base = state % offset_coefficient;
        let multiplier = state / offset_coefficient;
        let mut new = (value as u16 - (base / stride)) * stride + base;
        new += multiplier * offset_coefficient;
        Some(new)
    }
    #[doc = "Retrieves the `layers` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn layers(&self, kind: BlockKind, state: u16) -> Option<i32> {
        let (offset_coefficient, stride) = self.layers[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some({ x as i32 })
    }
    #[doc = "Updates the state value for the given block kind such that its `layers` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_layers(&self, kind: BlockKind, state: u16, value: i32) -> Option<u16> {
        let (offset_coefficient, stride) = self.layers[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let base = state % offset_coefficient;
        let multiplier = state / offset_coefficient;
        let mut new = (value as u16 - (base / stride)) * stride + base;
        new += multiplier * offset_coefficient;
        Some(new)
    }
    #[doc = "Retrieves the `level` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn level(&self, kind: BlockKind, state: u16) -> Option<i32> {
        let (offset_coefficient, stride) = self.level[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some({ x as i32 })
    }
    #[doc = "Updates the state value for the given block kind such that its `level` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_level(&self, kind: BlockKind, state: u16, value: i32) -> Option<u16> {
        let (offset_coefficient, stride) = self.level[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let base = state % offset_coefficient;
        let multiplier = state / offset_coefficient;
        let mut new = (value as u16 - (base / stride)) * stride + base;
        new += multiplier * offset_coefficient;
        Some(new)
    }
    #[doc = "Retrieves the `lit` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn lit(&self, kind: BlockKind, state: u16) -> Option<bool> {
        let (offset_coefficient, stride) = self.lit[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(if x == 0 { false } else { true })
    }
    #[doc = "Updates the state value for the given block kind such that its `lit` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_lit(&self, kind: BlockKind, state: u16, value: bool) -> Option<u16> {
        let (offset_coefficient, stride) = self.lit[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let base = state % offset_coefficient;
        let multiplier = state / offset_coefficient;
        let mut new = (value as u16 - (base / stride)) * stride + base;
        new += multiplier * offset_coefficient;
        Some(new)
    }
    #[doc = "Retrieves the `locked` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn locked(&self, kind: BlockKind, state: u16) -> Option<bool> {
        let (offset_coefficient, stride) = self.locked[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(if x == 0 { false } else { true })
    }
    #[doc = "Updates the state value for the given block kind such that its `locked` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_locked(&self, kind: BlockKind, state: u16, value: bool) -> Option<u16> {
        let (offset_coefficient, stride) = self.locked[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let base = state % offset_coefficient;
        let multiplier = state / offset_coefficient;
        let mut new = (value as u16 - (base / stride)) * stride + base;
        new += multiplier * offset_coefficient;
        Some(new)
    }
    #[doc = "Retrieves the `moisture` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn moisture(&self, kind: BlockKind, state: u16) -> Option<i32> {
        let (offset_coefficient, stride) = self.moisture[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some({ x as i32 })
    }
    #[doc = "Updates the state value for the given block kind such that its `moisture` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_moisture(&self, kind: BlockKind, state: u16, value: i32) -> Option<u16> {
        let (offset_coefficient, stride) = self.moisture[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let base = state % offset_coefficient;
        let multiplier = state / offset_coefficient;
        let mut new = (value as u16 - (base / stride)) * stride + base;
        new += multiplier * offset_coefficient;
        Some(new)
    }
    #[doc = "Retrieves the `north_connected` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn north_connected(&self, kind: BlockKind, state: u16) -> Option<bool> {
        let (offset_coefficient, stride) = self.north_connected[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(if x == 0 { false } else { true })
    }
    #[doc = "Updates the state value for the given block kind such that its `north_connected` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_north_connected(&self, kind: BlockKind, state: u16, value: bool) -> Option<u16> {
        let (offset_coefficient, stride) = self.north_connected[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let base = state % offset_coefficient;
        let multiplier = state / offset_coefficient;
        let mut new = (value as u16 - (base / stride)) * stride + base;
        new += multiplier * offset_coefficient;
        Some(new)
    }
    #[doc = "Retrieves the `north_wire` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn north_wire(&self, kind: BlockKind, state: u16) -> Option<NorthWire> {
        let (offset_coefficient, stride) = self.north_wire[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(NorthWire::try_from(x).expect("invalid block state"))
    }
    #[doc = "Updates the state value for the given block kind such that its `north_wire` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_north_wire(&self, kind: BlockKind, state: u16, value: NorthWire) -> Option<u16> {
        let (offset_coefficient, stride) = self.north_wire[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let base = state % offset_coefficient;
        let multiplier = state / offset_coefficient;
        let mut new = (value as u16 - (base / stride)) * stride + base;
        new += multiplier * offset_coefficient;
        Some(new)
    }
    #[doc = "Retrieves the `note` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn note(&self, kind: BlockKind, state: u16) -> Option<i32> {
        let (offset_coefficient, stride) = self.note[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some({ x as i32 })
    }
    #[doc = "Updates the state value for the given block kind such that its `note` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_note(&self, kind: BlockKind, state: u16, value: i32) -> Option<u16> {
        let (offset_coefficient, stride) = self.note[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let base = state % offset_coefficient;
        let multiplier = state / offset_coefficient;
        let mut new = (value as u16 - (base / stride)) * stride + base;
        new += multiplier * offset_coefficient;
        Some(new)
    }
    #[doc = "Retrieves the `occupied` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn occupied(&self, kind: BlockKind, state: u16) -> Option<bool> {
        let (offset_coefficient, stride) = self.occupied[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(if x == 0 { false } else { true })
    }
    #[doc = "Updates the state value for the given block kind such that its `occupied` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_occupied(&self, kind: BlockKind, state: u16, value: bool) -> Option<u16> {
        let (offset_coefficient, stride) = self.occupied[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let base = state % offset_coefficient;
        let multiplier = state / offset_coefficient;
        let mut new = (value as u16 - (base / stride)) * stride + base;
        new += multiplier * offset_coefficient;
        Some(new)
    }
    #[doc = "Retrieves the `open` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn open(&self, kind: BlockKind, state: u16) -> Option<bool> {
        let (offset_coefficient, stride) = self.open[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(if x == 0 { false } else { true })
    }
    #[doc = "Updates the state value for the given block kind such that its `open` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_open(&self, kind: BlockKind, state: u16, value: bool) -> Option<u16> {
        let (offset_coefficient, stride) = self.open[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let base = state % offset_coefficient;
        let multiplier = state / offset_coefficient;
        let mut new = (value as u16 - (base / stride)) * stride + base;
        new += multiplier * offset_coefficient;
        Some(new)
    }
    #[doc = "Retrieves the `part` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn part(&self, kind: BlockKind, state: u16) -> Option<Part> {
        let (offset_coefficient, stride) = self.part[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(Part::try_from(x).expect("invalid block state"))
    }
    #[doc = "Updates the state value for the given block kind such that its `part` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_part(&self, kind: BlockKind, state: u16, value: Part) -> Option<u16> {
        let (offset_coefficient, stride) = self.part[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let base = state % offset_coefficient;
        let multiplier = state / offset_coefficient;
        let mut new = (value as u16 - (base / stride)) * stride + base;
        new += multiplier * offset_coefficient;
        Some(new)
    }
    #[doc = "Retrieves the `persistent` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn persistent(&self, kind: BlockKind, state: u16) -> Option<bool> {
        let (offset_coefficient, stride) = self.persistent[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(if x == 0 { false } else { true })
    }
    #[doc = "Updates the state value for the given block kind such that its `persistent` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_persistent(&self, kind: BlockKind, state: u16, value: bool) -> Option<u16> {
        let (offset_coefficient, stride) = self.persistent[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let base = state % offset_coefficient;
        let multiplier = state / offset_coefficient;
        let mut new = (value as u16 - (base / stride)) * stride + base;
        new += multiplier * offset_coefficient;
        Some(new)
    }
    #[doc = "Retrieves the `pickles` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn pickles(&self, kind: BlockKind, state: u16) -> Option<i32> {
        let (offset_coefficient, stride) = self.pickles[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some({ x as i32 })
    }
    #[doc = "Updates the state value for the given block kind such that its `pickles` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_pickles(&self, kind: BlockKind, state: u16, value: i32) -> Option<u16> {
        let (offset_coefficient, stride) = self.pickles[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let base = state % offset_coefficient;
        let multiplier = state / offset_coefficient;
        let mut new = (value as u16 - (base / stride)) * stride + base;
        new += multiplier * offset_coefficient;
        Some(new)
    }
    #[doc = "Retrieves the `piston_kind` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn piston_kind(&self, kind: BlockKind, state: u16) -> Option<PistonKind> {
        let (offset_coefficient, stride) = self.piston_kind[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(PistonKind::try_from(x).expect("invalid block state"))
    }
    #[doc = "Updates the state value for the given block kind such that its `piston_kind` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_piston_kind(&self, kind: BlockKind, state: u16, value: PistonKind) -> Option<u16> {
        let (offset_coefficient, stride) = self.piston_kind[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let base = state % offset_coefficient;
        let multiplier = state / offset_coefficient;
        let mut new = (value as u16 - (base / stride)) * stride + base;
        new += multiplier * offset_coefficient;
        Some(new)
    }
    #[doc = "Retrieves the `power` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn power(&self, kind: BlockKind, state: u16) -> Option<i32> {
        let (offset_coefficient, stride) = self.power[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some({ x as i32 })
    }
    #[doc = "Updates the state value for the given block kind such that its `power` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_power(&self, kind: BlockKind, state: u16, value: i32) -> Option<u16> {
        let (offset_coefficient, stride) = self.power[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let base = state % offset_coefficient;
        let multiplier = state / offset_coefficient;
        let mut new = (value as u16 - (base / stride)) * stride + base;
        new += multiplier * offset_coefficient;
        Some(new)
    }
    #[doc = "Retrieves the `powered` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn powered(&self, kind: BlockKind, state: u16) -> Option<bool> {
        let (offset_coefficient, stride) = self.powered[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(if x == 0 { false } else { true })
    }
    #[doc = "Updates the state value for the given block kind such that its `powered` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_powered(&self, kind: BlockKind, state: u16, value: bool) -> Option<u16> {
        let (offset_coefficient, stride) = self.powered[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let base = state % offset_coefficient;
        let multiplier = state / offset_coefficient;
        let mut new = (value as u16 - (base / stride)) * stride + base;
        new += multiplier * offset_coefficient;
        Some(new)
    }
    #[doc = "Retrieves the `powered_rail_shape` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn powered_rail_shape(&self, kind: BlockKind, state: u16) -> Option<PoweredRailShape> {
        let (offset_coefficient, stride) = self.powered_rail_shape[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(PoweredRailShape::try_from(x).expect("invalid block state"))
    }
    #[doc = "Updates the state value for the given block kind such that its `powered_rail_shape` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_powered_rail_shape(
        &self,
        kind: BlockKind,
        state: u16,
        value: PoweredRailShape,
    ) -> Option<u16> {
        let (offset_coefficient, stride) = self.powered_rail_shape[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let base = state % offset_coefficient;
        let multiplier = state / offset_coefficient;
        let mut new = (value as u16 - (base / stride)) * stride + base;
        new += multiplier * offset_coefficient;
        Some(new)
    }
    #[doc = "Retrieves the `rail_shape` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn rail_shape(&self, kind: BlockKind, state: u16) -> Option<RailShape> {
        let (offset_coefficient, stride) = self.rail_shape[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(RailShape::try_from(x).expect("invalid block state"))
    }
    #[doc = "Updates the state value for the given block kind such that its `rail_shape` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_rail_shape(&self, kind: BlockKind, state: u16, value: RailShape) -> Option<u16> {
        let (offset_coefficient, stride) = self.rail_shape[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let base = state % offset_coefficient;
        let multiplier = state / offset_coefficient;
        let mut new = (value as u16 - (base / stride)) * stride + base;
        new += multiplier * offset_coefficient;
        Some(new)
    }
    #[doc = "Retrieves the `rotation` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn rotation(&self, kind: BlockKind, state: u16) -> Option<i32> {
        let (offset_coefficient, stride) = self.rotation[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some({ x as i32 })
    }
    #[doc = "Updates the state value for the given block kind such that its `rotation` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_rotation(&self, kind: BlockKind, state: u16, value: i32) -> Option<u16> {
        let (offset_coefficient, stride) = self.rotation[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let base = state % offset_coefficient;
        let multiplier = state / offset_coefficient;
        let mut new = (value as u16 - (base / stride)) * stride + base;
        new += multiplier * offset_coefficient;
        Some(new)
    }
    #[doc = "Retrieves the `short` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn short(&self, kind: BlockKind, state: u16) -> Option<bool> {
        let (offset_coefficient, stride) = self.short[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(if x == 0 { false } else { true })
    }
    #[doc = "Updates the state value for the given block kind such that its `short` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_short(&self, kind: BlockKind, state: u16, value: bool) -> Option<u16> {
        let (offset_coefficient, stride) = self.short[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let base = state % offset_coefficient;
        let multiplier = state / offset_coefficient;
        let mut new = (value as u16 - (base / stride)) * stride + base;
        new += multiplier * offset_coefficient;
        Some(new)
    }
    #[doc = "Retrieves the `slab_kind` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn slab_kind(&self, kind: BlockKind, state: u16) -> Option<SlabKind> {
        let (offset_coefficient, stride) = self.slab_kind[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(SlabKind::try_from(x).expect("invalid block state"))
    }
    #[doc = "Updates the state value for the given block kind such that its `slab_kind` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_slab_kind(&self, kind: BlockKind, state: u16, value: SlabKind) -> Option<u16> {
        let (offset_coefficient, stride) = self.slab_kind[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let base = state % offset_coefficient;
        let multiplier = state / offset_coefficient;
        let mut new = (value as u16 - (base / stride)) * stride + base;
        new += multiplier * offset_coefficient;
        Some(new)
    }
    #[doc = "Retrieves the `snowy` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn snowy(&self, kind: BlockKind, state: u16) -> Option<bool> {
        let (offset_coefficient, stride) = self.snowy[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(if x == 0 { false } else { true })
    }
    #[doc = "Updates the state value for the given block kind such that its `snowy` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_snowy(&self, kind: BlockKind, state: u16, value: bool) -> Option<u16> {
        let (offset_coefficient, stride) = self.snowy[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let base = state % offset_coefficient;
        let multiplier = state / offset_coefficient;
        let mut new = (value as u16 - (base / stride)) * stride + base;
        new += multiplier * offset_coefficient;
        Some(new)
    }
    #[doc = "Retrieves the `south_connected` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn south_connected(&self, kind: BlockKind, state: u16) -> Option<bool> {
        let (offset_coefficient, stride) = self.south_connected[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(if x == 0 { false } else { true })
    }
    #[doc = "Updates the state value for the given block kind such that its `south_connected` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_south_connected(&self, kind: BlockKind, state: u16, value: bool) -> Option<u16> {
        let (offset_coefficient, stride) = self.south_connected[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let base = state % offset_coefficient;
        let multiplier = state / offset_coefficient;
        let mut new = (value as u16 - (base / stride)) * stride + base;
        new += multiplier * offset_coefficient;
        Some(new)
    }
    #[doc = "Retrieves the `south_wire` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn south_wire(&self, kind: BlockKind, state: u16) -> Option<SouthWire> {
        let (offset_coefficient, stride) = self.south_wire[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(SouthWire::try_from(x).expect("invalid block state"))
    }
    #[doc = "Updates the state value for the given block kind such that its `south_wire` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_south_wire(&self, kind: BlockKind, state: u16, value: SouthWire) -> Option<u16> {
        let (offset_coefficient, stride) = self.south_wire[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let base = state % offset_coefficient;
        let multiplier = state / offset_coefficient;
        let mut new = (value as u16 - (base / stride)) * stride + base;
        new += multiplier * offset_coefficient;
        Some(new)
    }
    #[doc = "Retrieves the `stage` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn stage(&self, kind: BlockKind, state: u16) -> Option<i32> {
        let (offset_coefficient, stride) = self.stage[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some({ x as i32 })
    }
    #[doc = "Updates the state value for the given block kind such that its `stage` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_stage(&self, kind: BlockKind, state: u16, value: i32) -> Option<u16> {
        let (offset_coefficient, stride) = self.stage[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let base = state % offset_coefficient;
        let multiplier = state / offset_coefficient;
        let mut new = (value as u16 - (base / stride)) * stride + base;
        new += multiplier * offset_coefficient;
        Some(new)
    }
    #[doc = "Retrieves the `stairs_shape` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn stairs_shape(&self, kind: BlockKind, state: u16) -> Option<StairsShape> {
        let (offset_coefficient, stride) = self.stairs_shape[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(StairsShape::try_from(x).expect("invalid block state"))
    }
    #[doc = "Updates the state value for the given block kind such that its `stairs_shape` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_stairs_shape(&self, kind: BlockKind, state: u16, value: StairsShape) -> Option<u16> {
        let (offset_coefficient, stride) = self.stairs_shape[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let base = state % offset_coefficient;
        let multiplier = state / offset_coefficient;
        let mut new = (value as u16 - (base / stride)) * stride + base;
        new += multiplier * offset_coefficient;
        Some(new)
    }
    #[doc = "Retrieves the `structure_block_mode` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn structure_block_mode(&self, kind: BlockKind, state: u16) -> Option<StructureBlockMode> {
        let (offset_coefficient, stride) = self.structure_block_mode[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(StructureBlockMode::try_from(x).expect("invalid block state"))
    }
    #[doc = "Updates the state value for the given block kind such that its `structure_block_mode` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_structure_block_mode(
        &self,
        kind: BlockKind,
        state: u16,
        value: StructureBlockMode,
    ) -> Option<u16> {
        let (offset_coefficient, stride) = self.structure_block_mode[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let base = state % offset_coefficient;
        let multiplier = state / offset_coefficient;
        let mut new = (value as u16 - (base / stride)) * stride + base;
        new += multiplier * offset_coefficient;
        Some(new)
    }
    #[doc = "Retrieves the `triggered` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn triggered(&self, kind: BlockKind, state: u16) -> Option<bool> {
        let (offset_coefficient, stride) = self.triggered[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(if x == 0 { false } else { true })
    }
    #[doc = "Updates the state value for the given block kind such that its `triggered` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_triggered(&self, kind: BlockKind, state: u16, value: bool) -> Option<u16> {
        let (offset_coefficient, stride) = self.triggered[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let base = state % offset_coefficient;
        let multiplier = state / offset_coefficient;
        let mut new = (value as u16 - (base / stride)) * stride + base;
        new += multiplier * offset_coefficient;
        Some(new)
    }
    #[doc = "Retrieves the `unstable` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn unstable(&self, kind: BlockKind, state: u16) -> Option<bool> {
        let (offset_coefficient, stride) = self.unstable[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(if x == 0 { false } else { true })
    }
    #[doc = "Updates the state value for the given block kind such that its `unstable` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_unstable(&self, kind: BlockKind, state: u16, value: bool) -> Option<u16> {
        let (offset_coefficient, stride) = self.unstable[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let base = state % offset_coefficient;
        let multiplier = state / offset_coefficient;
        let mut new = (value as u16 - (base / stride)) * stride + base;
        new += multiplier * offset_coefficient;
        Some(new)
    }
    #[doc = "Retrieves the `up` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn up(&self, kind: BlockKind, state: u16) -> Option<bool> {
        let (offset_coefficient, stride) = self.up[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(if x == 0 { false } else { true })
    }
    #[doc = "Updates the state value for the given block kind such that its `up` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_up(&self, kind: BlockKind, state: u16, value: bool) -> Option<u16> {
        let (offset_coefficient, stride) = self.up[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let base = state % offset_coefficient;
        let multiplier = state / offset_coefficient;
        let mut new = (value as u16 - (base / stride)) * stride + base;
        new += multiplier * offset_coefficient;
        Some(new)
    }
    #[doc = "Retrieves the `waterlogged` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn waterlogged(&self, kind: BlockKind, state: u16) -> Option<bool> {
        let (offset_coefficient, stride) = self.waterlogged[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(if x == 0 { false } else { true })
    }
    #[doc = "Updates the state value for the given block kind such that its `waterlogged` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_waterlogged(&self, kind: BlockKind, state: u16, value: bool) -> Option<u16> {
        let (offset_coefficient, stride) = self.waterlogged[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let base = state % offset_coefficient;
        let multiplier = state / offset_coefficient;
        let mut new = (value as u16 - (base / stride)) * stride + base;
        new += multiplier * offset_coefficient;
        Some(new)
    }
    #[doc = "Retrieves the `west_connected` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn west_connected(&self, kind: BlockKind, state: u16) -> Option<bool> {
        let (offset_coefficient, stride) = self.west_connected[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(if x == 0 { false } else { true })
    }
    #[doc = "Updates the state value for the given block kind such that its `west_connected` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_west_connected(&self, kind: BlockKind, state: u16, value: bool) -> Option<u16> {
        let (offset_coefficient, stride) = self.west_connected[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let base = state % offset_coefficient;
        let multiplier = state / offset_coefficient;
        let mut new = (value as u16 - (base / stride)) * stride + base;
        new += multiplier * offset_coefficient;
        Some(new)
    }
    #[doc = "Retrieves the `west_wire` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn west_wire(&self, kind: BlockKind, state: u16) -> Option<WestWire> {
        let (offset_coefficient, stride) = self.west_wire[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(WestWire::try_from(x).expect("invalid block state"))
    }
    #[doc = "Updates the state value for the given block kind such that its `west_wire` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_west_wire(&self, kind: BlockKind, state: u16, value: WestWire) -> Option<u16> {
        let (offset_coefficient, stride) = self.west_wire[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let base = state % offset_coefficient;
        let multiplier = state / offset_coefficient;
        let mut new = (value as u16 - (base / stride)) * stride + base;
        new += multiplier * offset_coefficient;
        Some(new)
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum AxisXyz {
    X,
    Y,
    Z,
}
impl TryFrom<u16> for AxisXyz {
    type Error = anyhow::Error;
    fn try_from(value: u16) -> anyhow::Result<Self> {
        match value {
            0u16 => Ok(AxisXyz::X),
            1u16 => Ok(AxisXyz::Y),
            2u16 => Ok(AxisXyz::Z),
            x => Err(anyhow::anyhow!("invalid value {} for AxisXyz", x)),
        }
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum AxisXz {
    X,
    Z,
}
impl TryFrom<u16> for AxisXz {
    type Error = anyhow::Error;
    fn try_from(value: u16) -> anyhow::Result<Self> {
        match value {
            0u16 => Ok(AxisXz::X),
            1u16 => Ok(AxisXz::Z),
            x => Err(anyhow::anyhow!("invalid value {} for AxisXz", x)),
        }
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
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
            0u16 => Ok(ChestKind::Single),
            1u16 => Ok(ChestKind::Left),
            2u16 => Ok(ChestKind::Right),
            x => Err(anyhow::anyhow!("invalid value {} for ChestKind", x)),
        }
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum ComparatorMode {
    Compare,
    Subtract,
}
impl TryFrom<u16> for ComparatorMode {
    type Error = anyhow::Error;
    fn try_from(value: u16) -> anyhow::Result<Self> {
        match value {
            0u16 => Ok(ComparatorMode::Compare),
            1u16 => Ok(ComparatorMode::Subtract),
            x => Err(anyhow::anyhow!("invalid value {} for ComparatorMode", x)),
        }
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum EastWire {
    Up,
    Side,
    None,
}
impl TryFrom<u16> for EastWire {
    type Error = anyhow::Error;
    fn try_from(value: u16) -> anyhow::Result<Self> {
        match value {
            0u16 => Ok(EastWire::Up),
            1u16 => Ok(EastWire::Side),
            2u16 => Ok(EastWire::None),
            x => Err(anyhow::anyhow!("invalid value {} for EastWire", x)),
        }
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum Face {
    Floor,
    Wall,
    Ceiling,
}
impl TryFrom<u16> for Face {
    type Error = anyhow::Error;
    fn try_from(value: u16) -> anyhow::Result<Self> {
        match value {
            0u16 => Ok(Face::Floor),
            1u16 => Ok(Face::Wall),
            2u16 => Ok(Face::Ceiling),
            x => Err(anyhow::anyhow!("invalid value {} for Face", x)),
        }
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum FacingCardinal {
    North,
    South,
    West,
    East,
}
impl TryFrom<u16> for FacingCardinal {
    type Error = anyhow::Error;
    fn try_from(value: u16) -> anyhow::Result<Self> {
        match value {
            0u16 => Ok(FacingCardinal::North),
            1u16 => Ok(FacingCardinal::South),
            2u16 => Ok(FacingCardinal::West),
            3u16 => Ok(FacingCardinal::East),
            x => Err(anyhow::anyhow!("invalid value {} for FacingCardinal", x)),
        }
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum FacingCardinalAndDown {
    Down,
    North,
    South,
    West,
    East,
}
impl TryFrom<u16> for FacingCardinalAndDown {
    type Error = anyhow::Error;
    fn try_from(value: u16) -> anyhow::Result<Self> {
        match value {
            0u16 => Ok(FacingCardinalAndDown::Down),
            1u16 => Ok(FacingCardinalAndDown::North),
            2u16 => Ok(FacingCardinalAndDown::South),
            3u16 => Ok(FacingCardinalAndDown::West),
            4u16 => Ok(FacingCardinalAndDown::East),
            x => Err(anyhow::anyhow!(
                "invalid value {} for FacingCardinalAndDown",
                x
            )),
        }
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum FacingCubic {
    North,
    East,
    South,
    West,
    Up,
    Down,
}
impl TryFrom<u16> for FacingCubic {
    type Error = anyhow::Error;
    fn try_from(value: u16) -> anyhow::Result<Self> {
        match value {
            0u16 => Ok(FacingCubic::North),
            1u16 => Ok(FacingCubic::East),
            2u16 => Ok(FacingCubic::South),
            3u16 => Ok(FacingCubic::West),
            4u16 => Ok(FacingCubic::Up),
            5u16 => Ok(FacingCubic::Down),
            x => Err(anyhow::anyhow!("invalid value {} for FacingCubic", x)),
        }
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum Half {
    Upper,
    Lower,
}
impl TryFrom<u16> for Half {
    type Error = anyhow::Error;
    fn try_from(value: u16) -> anyhow::Result<Self> {
        match value {
            0u16 => Ok(Half::Upper),
            1u16 => Ok(Half::Lower),
            x => Err(anyhow::anyhow!("invalid value {} for Half", x)),
        }
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum Hinge {
    Left,
    Right,
}
impl TryFrom<u16> for Hinge {
    type Error = anyhow::Error;
    fn try_from(value: u16) -> anyhow::Result<Self> {
        match value {
            0u16 => Ok(Hinge::Left),
            1u16 => Ok(Hinge::Right),
            x => Err(anyhow::anyhow!("invalid value {} for Hinge", x)),
        }
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum Instrument {
    Harp,
    Basedrum,
    Snare,
    Hat,
    Bass,
    Flute,
    Bell,
    Guitar,
    Chime,
    Xylophone,
}
impl TryFrom<u16> for Instrument {
    type Error = anyhow::Error;
    fn try_from(value: u16) -> anyhow::Result<Self> {
        match value {
            0u16 => Ok(Instrument::Harp),
            1u16 => Ok(Instrument::Basedrum),
            2u16 => Ok(Instrument::Snare),
            3u16 => Ok(Instrument::Hat),
            4u16 => Ok(Instrument::Bass),
            5u16 => Ok(Instrument::Flute),
            6u16 => Ok(Instrument::Bell),
            7u16 => Ok(Instrument::Guitar),
            8u16 => Ok(Instrument::Chime),
            9u16 => Ok(Instrument::Xylophone),
            x => Err(anyhow::anyhow!("invalid value {} for Instrument", x)),
        }
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum NorthWire {
    Up,
    Side,
    None,
}
impl TryFrom<u16> for NorthWire {
    type Error = anyhow::Error;
    fn try_from(value: u16) -> anyhow::Result<Self> {
        match value {
            0u16 => Ok(NorthWire::Up),
            1u16 => Ok(NorthWire::Side),
            2u16 => Ok(NorthWire::None),
            x => Err(anyhow::anyhow!("invalid value {} for NorthWire", x)),
        }
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum Part {
    Head,
    Foot,
}
impl TryFrom<u16> for Part {
    type Error = anyhow::Error;
    fn try_from(value: u16) -> anyhow::Result<Self> {
        match value {
            0u16 => Ok(Part::Head),
            1u16 => Ok(Part::Foot),
            x => Err(anyhow::anyhow!("invalid value {} for Part", x)),
        }
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum PistonKind {
    Normal,
    Sticky,
}
impl TryFrom<u16> for PistonKind {
    type Error = anyhow::Error;
    fn try_from(value: u16) -> anyhow::Result<Self> {
        match value {
            0u16 => Ok(PistonKind::Normal),
            1u16 => Ok(PistonKind::Sticky),
            x => Err(anyhow::anyhow!("invalid value {} for PistonKind", x)),
        }
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum PoweredRailShape {
    NorthSouth,
    EastWest,
    AscendingEast,
    AscendingWest,
    AscendingNorth,
    AscendingSouth,
}
impl TryFrom<u16> for PoweredRailShape {
    type Error = anyhow::Error;
    fn try_from(value: u16) -> anyhow::Result<Self> {
        match value {
            0u16 => Ok(PoweredRailShape::NorthSouth),
            1u16 => Ok(PoweredRailShape::EastWest),
            2u16 => Ok(PoweredRailShape::AscendingEast),
            3u16 => Ok(PoweredRailShape::AscendingWest),
            4u16 => Ok(PoweredRailShape::AscendingNorth),
            5u16 => Ok(PoweredRailShape::AscendingSouth),
            x => Err(anyhow::anyhow!("invalid value {} for PoweredRailShape", x)),
        }
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum RailShape {
    NorthSouth,
    EastWest,
    AscendingEast,
    AscendingWest,
    AscendingNorth,
    AscendingSouth,
    SouthEast,
    SouthWest,
    NorthWest,
    NorthEast,
}
impl TryFrom<u16> for RailShape {
    type Error = anyhow::Error;
    fn try_from(value: u16) -> anyhow::Result<Self> {
        match value {
            0u16 => Ok(RailShape::NorthSouth),
            1u16 => Ok(RailShape::EastWest),
            2u16 => Ok(RailShape::AscendingEast),
            3u16 => Ok(RailShape::AscendingWest),
            4u16 => Ok(RailShape::AscendingNorth),
            5u16 => Ok(RailShape::AscendingSouth),
            6u16 => Ok(RailShape::SouthEast),
            7u16 => Ok(RailShape::SouthWest),
            8u16 => Ok(RailShape::NorthWest),
            9u16 => Ok(RailShape::NorthEast),
            x => Err(anyhow::anyhow!("invalid value {} for RailShape", x)),
        }
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum SlabKind {
    Top,
    Bottom,
    Double,
}
impl TryFrom<u16> for SlabKind {
    type Error = anyhow::Error;
    fn try_from(value: u16) -> anyhow::Result<Self> {
        match value {
            0u16 => Ok(SlabKind::Top),
            1u16 => Ok(SlabKind::Bottom),
            2u16 => Ok(SlabKind::Double),
            x => Err(anyhow::anyhow!("invalid value {} for SlabKind", x)),
        }
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum SouthWire {
    Up,
    Side,
    None,
}
impl TryFrom<u16> for SouthWire {
    type Error = anyhow::Error;
    fn try_from(value: u16) -> anyhow::Result<Self> {
        match value {
            0u16 => Ok(SouthWire::Up),
            1u16 => Ok(SouthWire::Side),
            2u16 => Ok(SouthWire::None),
            x => Err(anyhow::anyhow!("invalid value {} for SouthWire", x)),
        }
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum StairsShape {
    Straight,
    InnerLeft,
    InnerRight,
    OuterLeft,
    OuterRight,
}
impl TryFrom<u16> for StairsShape {
    type Error = anyhow::Error;
    fn try_from(value: u16) -> anyhow::Result<Self> {
        match value {
            0u16 => Ok(StairsShape::Straight),
            1u16 => Ok(StairsShape::InnerLeft),
            2u16 => Ok(StairsShape::InnerRight),
            3u16 => Ok(StairsShape::OuterLeft),
            4u16 => Ok(StairsShape::OuterRight),
            x => Err(anyhow::anyhow!("invalid value {} for StairsShape", x)),
        }
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum StructureBlockMode {
    Save,
    Load,
    Corner,
    Data,
}
impl TryFrom<u16> for StructureBlockMode {
    type Error = anyhow::Error;
    fn try_from(value: u16) -> anyhow::Result<Self> {
        match value {
            0u16 => Ok(StructureBlockMode::Save),
            1u16 => Ok(StructureBlockMode::Load),
            2u16 => Ok(StructureBlockMode::Corner),
            3u16 => Ok(StructureBlockMode::Data),
            x => Err(anyhow::anyhow!(
                "invalid value {} for StructureBlockMode",
                x
            )),
        }
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum WestWire {
    Up,
    Side,
    None,
}
impl TryFrom<u16> for WestWire {
    type Error = anyhow::Error;
    fn try_from(value: u16) -> anyhow::Result<Self> {
        match value {
            0u16 => Ok(WestWire::Up),
            1u16 => Ok(WestWire::Side),
            2u16 => Ok(WestWire::None),
            x => Err(anyhow::anyhow!("invalid value {} for WestWire", x)),
        }
    }
}
