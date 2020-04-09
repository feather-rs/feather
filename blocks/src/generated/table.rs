use crate::BlockKind;
use std::convert::TryFrom;
#[derive(Debug)]
pub struct BlockTable {
    age: Vec<(u16, u16)>,
    attached: Vec<(u16, u16)>,
    axis_xyz: Vec<(u16, u16)>,
    axis_xz: Vec<(u16, u16)>,
    bites: Vec<(u16, u16)>,
    conditional: Vec<(u16, u16)>,
    delay: Vec<(u16, u16)>,
    disarmed: Vec<(u16, u16)>,
    distance: Vec<(u16, u16)>,
    down: Vec<(u16, u16)>,
    drag: Vec<(u16, u16)>,
    east_tf: Vec<(u16, u16)>,
    east_usn: Vec<(u16, u16)>,
    eggs: Vec<(u16, u16)>,
    enabled: Vec<(u16, u16)>,
    extended: Vec<(u16, u16)>,
    eye: Vec<(u16, u16)>,
    face: Vec<(u16, u16)>,
    facing_dnswe: Vec<(u16, u16)>,
    facing_neswud: Vec<(u16, u16)>,
    facing_nswe: Vec<(u16, u16)>,
    half_tb: Vec<(u16, u16)>,
    half_ul: Vec<(u16, u16)>,
    has_bottle_0: Vec<(u16, u16)>,
    has_bottle_1: Vec<(u16, u16)>,
    has_bottle_2: Vec<(u16, u16)>,
    has_record: Vec<(u16, u16)>,
    hatch: Vec<(u16, u16)>,
    hinge: Vec<(u16, u16)>,
    in_wall: Vec<(u16, u16)>,
    instrument: Vec<(u16, u16)>,
    inverted: Vec<(u16, u16)>,
    kind_ns: Vec<(u16, u16)>,
    kind_slr: Vec<(u16, u16)>,
    kind_tbd: Vec<(u16, u16)>,
    layers: Vec<(u16, u16)>,
    level: Vec<(u16, u16)>,
    lit: Vec<(u16, u16)>,
    locked: Vec<(u16, u16)>,
    mode_cs: Vec<(u16, u16)>,
    mode_slcd: Vec<(u16, u16)>,
    moisture: Vec<(u16, u16)>,
    north_tf: Vec<(u16, u16)>,
    north_usn: Vec<(u16, u16)>,
    note: Vec<(u16, u16)>,
    occupied: Vec<(u16, u16)>,
    open: Vec<(u16, u16)>,
    part: Vec<(u16, u16)>,
    persistent: Vec<(u16, u16)>,
    pickles: Vec<(u16, u16)>,
    power: Vec<(u16, u16)>,
    powered: Vec<(u16, u16)>,
    rotation: Vec<(u16, u16)>,
    shape_neaaaa: Vec<(u16, u16)>,
    shape_neaaaassnn: Vec<(u16, u16)>,
    shape_siioo: Vec<(u16, u16)>,
    short: Vec<(u16, u16)>,
    snowy: Vec<(u16, u16)>,
    south_tf: Vec<(u16, u16)>,
    south_usn: Vec<(u16, u16)>,
    stage: Vec<(u16, u16)>,
    triggered: Vec<(u16, u16)>,
    unstable: Vec<(u16, u16)>,
    up: Vec<(u16, u16)>,
    waterlogged: Vec<(u16, u16)>,
    west_tf: Vec<(u16, u16)>,
    west_usn: Vec<(u16, u16)>,
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
    #[doc = "Retrieves the `east_tf` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn east_tf(&self, kind: BlockKind, state: u16) -> Option<bool> {
        let (offset_coefficient, stride) = self.east_tf[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(if x == 0 { false } else { true })
    }
    #[doc = "Updates the state value for the given block kind such that its `east_tf` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_east_tf(&self, kind: BlockKind, state: u16, value: bool) -> Option<u16> {
        let (offset_coefficient, stride) = self.east_tf[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let base = state % offset_coefficient;
        let multiplier = state / offset_coefficient;
        let mut new = (value as u16 - (base / stride)) * stride + base;
        new += multiplier * offset_coefficient;
        Some(new)
    }
    #[doc = "Retrieves the `east_usn` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn east_usn(&self, kind: BlockKind, state: u16) -> Option<EastUsn> {
        let (offset_coefficient, stride) = self.east_usn[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(EastUsn::try_from(x).expect("invalid block state"))
    }
    #[doc = "Updates the state value for the given block kind such that its `east_usn` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_east_usn(&self, kind: BlockKind, state: u16, value: EastUsn) -> Option<u16> {
        let (offset_coefficient, stride) = self.east_usn[kind as u16 as usize];
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
    #[doc = "Retrieves the `facing_dnswe` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn facing_dnswe(&self, kind: BlockKind, state: u16) -> Option<FacingDnswe> {
        let (offset_coefficient, stride) = self.facing_dnswe[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(FacingDnswe::try_from(x).expect("invalid block state"))
    }
    #[doc = "Updates the state value for the given block kind such that its `facing_dnswe` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_facing_dnswe(&self, kind: BlockKind, state: u16, value: FacingDnswe) -> Option<u16> {
        let (offset_coefficient, stride) = self.facing_dnswe[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let base = state % offset_coefficient;
        let multiplier = state / offset_coefficient;
        let mut new = (value as u16 - (base / stride)) * stride + base;
        new += multiplier * offset_coefficient;
        Some(new)
    }
    #[doc = "Retrieves the `facing_neswud` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn facing_neswud(&self, kind: BlockKind, state: u16) -> Option<FacingNeswud> {
        let (offset_coefficient, stride) = self.facing_neswud[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(FacingNeswud::try_from(x).expect("invalid block state"))
    }
    #[doc = "Updates the state value for the given block kind such that its `facing_neswud` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_facing_neswud(
        &self,
        kind: BlockKind,
        state: u16,
        value: FacingNeswud,
    ) -> Option<u16> {
        let (offset_coefficient, stride) = self.facing_neswud[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let base = state % offset_coefficient;
        let multiplier = state / offset_coefficient;
        let mut new = (value as u16 - (base / stride)) * stride + base;
        new += multiplier * offset_coefficient;
        Some(new)
    }
    #[doc = "Retrieves the `facing_nswe` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn facing_nswe(&self, kind: BlockKind, state: u16) -> Option<FacingNswe> {
        let (offset_coefficient, stride) = self.facing_nswe[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(FacingNswe::try_from(x).expect("invalid block state"))
    }
    #[doc = "Updates the state value for the given block kind such that its `facing_nswe` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_facing_nswe(&self, kind: BlockKind, state: u16, value: FacingNswe) -> Option<u16> {
        let (offset_coefficient, stride) = self.facing_nswe[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let base = state % offset_coefficient;
        let multiplier = state / offset_coefficient;
        let mut new = (value as u16 - (base / stride)) * stride + base;
        new += multiplier * offset_coefficient;
        Some(new)
    }
    #[doc = "Retrieves the `half_tb` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn half_tb(&self, kind: BlockKind, state: u16) -> Option<HalfTb> {
        let (offset_coefficient, stride) = self.half_tb[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(HalfTb::try_from(x).expect("invalid block state"))
    }
    #[doc = "Updates the state value for the given block kind such that its `half_tb` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_half_tb(&self, kind: BlockKind, state: u16, value: HalfTb) -> Option<u16> {
        let (offset_coefficient, stride) = self.half_tb[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let base = state % offset_coefficient;
        let multiplier = state / offset_coefficient;
        let mut new = (value as u16 - (base / stride)) * stride + base;
        new += multiplier * offset_coefficient;
        Some(new)
    }
    #[doc = "Retrieves the `half_ul` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn half_ul(&self, kind: BlockKind, state: u16) -> Option<HalfUl> {
        let (offset_coefficient, stride) = self.half_ul[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(HalfUl::try_from(x).expect("invalid block state"))
    }
    #[doc = "Updates the state value for the given block kind such that its `half_ul` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_half_ul(&self, kind: BlockKind, state: u16, value: HalfUl) -> Option<u16> {
        let (offset_coefficient, stride) = self.half_ul[kind as u16 as usize];
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
    #[doc = "Retrieves the `kind_ns` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn kind_ns(&self, kind: BlockKind, state: u16) -> Option<KindNs> {
        let (offset_coefficient, stride) = self.kind_ns[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(KindNs::try_from(x).expect("invalid block state"))
    }
    #[doc = "Updates the state value for the given block kind such that its `kind_ns` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_kind_ns(&self, kind: BlockKind, state: u16, value: KindNs) -> Option<u16> {
        let (offset_coefficient, stride) = self.kind_ns[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let base = state % offset_coefficient;
        let multiplier = state / offset_coefficient;
        let mut new = (value as u16 - (base / stride)) * stride + base;
        new += multiplier * offset_coefficient;
        Some(new)
    }
    #[doc = "Retrieves the `kind_slr` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn kind_slr(&self, kind: BlockKind, state: u16) -> Option<KindSlr> {
        let (offset_coefficient, stride) = self.kind_slr[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(KindSlr::try_from(x).expect("invalid block state"))
    }
    #[doc = "Updates the state value for the given block kind such that its `kind_slr` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_kind_slr(&self, kind: BlockKind, state: u16, value: KindSlr) -> Option<u16> {
        let (offset_coefficient, stride) = self.kind_slr[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let base = state % offset_coefficient;
        let multiplier = state / offset_coefficient;
        let mut new = (value as u16 - (base / stride)) * stride + base;
        new += multiplier * offset_coefficient;
        Some(new)
    }
    #[doc = "Retrieves the `kind_tbd` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn kind_tbd(&self, kind: BlockKind, state: u16) -> Option<KindTbd> {
        let (offset_coefficient, stride) = self.kind_tbd[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(KindTbd::try_from(x).expect("invalid block state"))
    }
    #[doc = "Updates the state value for the given block kind such that its `kind_tbd` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_kind_tbd(&self, kind: BlockKind, state: u16, value: KindTbd) -> Option<u16> {
        let (offset_coefficient, stride) = self.kind_tbd[kind as u16 as usize];
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
    #[doc = "Retrieves the `mode_cs` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn mode_cs(&self, kind: BlockKind, state: u16) -> Option<ModeCs> {
        let (offset_coefficient, stride) = self.mode_cs[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(ModeCs::try_from(x).expect("invalid block state"))
    }
    #[doc = "Updates the state value for the given block kind such that its `mode_cs` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_mode_cs(&self, kind: BlockKind, state: u16, value: ModeCs) -> Option<u16> {
        let (offset_coefficient, stride) = self.mode_cs[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let base = state % offset_coefficient;
        let multiplier = state / offset_coefficient;
        let mut new = (value as u16 - (base / stride)) * stride + base;
        new += multiplier * offset_coefficient;
        Some(new)
    }
    #[doc = "Retrieves the `mode_slcd` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn mode_slcd(&self, kind: BlockKind, state: u16) -> Option<ModeSlcd> {
        let (offset_coefficient, stride) = self.mode_slcd[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(ModeSlcd::try_from(x).expect("invalid block state"))
    }
    #[doc = "Updates the state value for the given block kind such that its `mode_slcd` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_mode_slcd(&self, kind: BlockKind, state: u16, value: ModeSlcd) -> Option<u16> {
        let (offset_coefficient, stride) = self.mode_slcd[kind as u16 as usize];
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
    #[doc = "Retrieves the `north_tf` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn north_tf(&self, kind: BlockKind, state: u16) -> Option<bool> {
        let (offset_coefficient, stride) = self.north_tf[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(if x == 0 { false } else { true })
    }
    #[doc = "Updates the state value for the given block kind such that its `north_tf` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_north_tf(&self, kind: BlockKind, state: u16, value: bool) -> Option<u16> {
        let (offset_coefficient, stride) = self.north_tf[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let base = state % offset_coefficient;
        let multiplier = state / offset_coefficient;
        let mut new = (value as u16 - (base / stride)) * stride + base;
        new += multiplier * offset_coefficient;
        Some(new)
    }
    #[doc = "Retrieves the `north_usn` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn north_usn(&self, kind: BlockKind, state: u16) -> Option<NorthUsn> {
        let (offset_coefficient, stride) = self.north_usn[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(NorthUsn::try_from(x).expect("invalid block state"))
    }
    #[doc = "Updates the state value for the given block kind such that its `north_usn` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_north_usn(&self, kind: BlockKind, state: u16, value: NorthUsn) -> Option<u16> {
        let (offset_coefficient, stride) = self.north_usn[kind as u16 as usize];
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
    #[doc = "Retrieves the `shape_neaaaa` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn shape_neaaaa(&self, kind: BlockKind, state: u16) -> Option<ShapeNeaaaa> {
        let (offset_coefficient, stride) = self.shape_neaaaa[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(ShapeNeaaaa::try_from(x).expect("invalid block state"))
    }
    #[doc = "Updates the state value for the given block kind such that its `shape_neaaaa` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_shape_neaaaa(&self, kind: BlockKind, state: u16, value: ShapeNeaaaa) -> Option<u16> {
        let (offset_coefficient, stride) = self.shape_neaaaa[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let base = state % offset_coefficient;
        let multiplier = state / offset_coefficient;
        let mut new = (value as u16 - (base / stride)) * stride + base;
        new += multiplier * offset_coefficient;
        Some(new)
    }
    #[doc = "Retrieves the `shape_neaaaassnn` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn shape_neaaaassnn(&self, kind: BlockKind, state: u16) -> Option<ShapeNeaaaassnn> {
        let (offset_coefficient, stride) = self.shape_neaaaassnn[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(ShapeNeaaaassnn::try_from(x).expect("invalid block state"))
    }
    #[doc = "Updates the state value for the given block kind such that its `shape_neaaaassnn` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_shape_neaaaassnn(
        &self,
        kind: BlockKind,
        state: u16,
        value: ShapeNeaaaassnn,
    ) -> Option<u16> {
        let (offset_coefficient, stride) = self.shape_neaaaassnn[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let base = state % offset_coefficient;
        let multiplier = state / offset_coefficient;
        let mut new = (value as u16 - (base / stride)) * stride + base;
        new += multiplier * offset_coefficient;
        Some(new)
    }
    #[doc = "Retrieves the `shape_siioo` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn shape_siioo(&self, kind: BlockKind, state: u16) -> Option<ShapeSiioo> {
        let (offset_coefficient, stride) = self.shape_siioo[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(ShapeSiioo::try_from(x).expect("invalid block state"))
    }
    #[doc = "Updates the state value for the given block kind such that its `shape_siioo` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_shape_siioo(&self, kind: BlockKind, state: u16, value: ShapeSiioo) -> Option<u16> {
        let (offset_coefficient, stride) = self.shape_siioo[kind as u16 as usize];
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
    #[doc = "Retrieves the `south_tf` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn south_tf(&self, kind: BlockKind, state: u16) -> Option<bool> {
        let (offset_coefficient, stride) = self.south_tf[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(if x == 0 { false } else { true })
    }
    #[doc = "Updates the state value for the given block kind such that its `south_tf` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_south_tf(&self, kind: BlockKind, state: u16, value: bool) -> Option<u16> {
        let (offset_coefficient, stride) = self.south_tf[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let base = state % offset_coefficient;
        let multiplier = state / offset_coefficient;
        let mut new = (value as u16 - (base / stride)) * stride + base;
        new += multiplier * offset_coefficient;
        Some(new)
    }
    #[doc = "Retrieves the `south_usn` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn south_usn(&self, kind: BlockKind, state: u16) -> Option<SouthUsn> {
        let (offset_coefficient, stride) = self.south_usn[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(SouthUsn::try_from(x).expect("invalid block state"))
    }
    #[doc = "Updates the state value for the given block kind such that its `south_usn` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_south_usn(&self, kind: BlockKind, state: u16, value: SouthUsn) -> Option<u16> {
        let (offset_coefficient, stride) = self.south_usn[kind as u16 as usize];
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
    #[doc = "Retrieves the `west_tf` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn west_tf(&self, kind: BlockKind, state: u16) -> Option<bool> {
        let (offset_coefficient, stride) = self.west_tf[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(if x == 0 { false } else { true })
    }
    #[doc = "Updates the state value for the given block kind such that its `west_tf` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_west_tf(&self, kind: BlockKind, state: u16, value: bool) -> Option<u16> {
        let (offset_coefficient, stride) = self.west_tf[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let base = state % offset_coefficient;
        let multiplier = state / offset_coefficient;
        let mut new = (value as u16 - (base / stride)) * stride + base;
        new += multiplier * offset_coefficient;
        Some(new)
    }
    #[doc = "Retrieves the `west_usn` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn west_usn(&self, kind: BlockKind, state: u16) -> Option<WestUsn> {
        let (offset_coefficient, stride) = self.west_usn[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(WestUsn::try_from(x).expect("invalid block state"))
    }
    #[doc = "Updates the state value for the given block kind such that its `west_usn` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_west_usn(&self, kind: BlockKind, state: u16, value: WestUsn) -> Option<u16> {
        let (offset_coefficient, stride) = self.west_usn[kind as u16 as usize];
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
pub enum EastUsn {
    Up,
    Side,
    None,
}
impl TryFrom<u16> for EastUsn {
    type Error = anyhow::Error;
    fn try_from(value: u16) -> anyhow::Result<Self> {
        match value {
            0u16 => Ok(EastUsn::Up),
            1u16 => Ok(EastUsn::Side),
            2u16 => Ok(EastUsn::None),
            x => Err(anyhow::anyhow!("invalid value {} for EastUsn", x)),
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
pub enum FacingDnswe {
    Down,
    North,
    South,
    West,
    East,
}
impl TryFrom<u16> for FacingDnswe {
    type Error = anyhow::Error;
    fn try_from(value: u16) -> anyhow::Result<Self> {
        match value {
            0u16 => Ok(FacingDnswe::Down),
            1u16 => Ok(FacingDnswe::North),
            2u16 => Ok(FacingDnswe::South),
            3u16 => Ok(FacingDnswe::West),
            4u16 => Ok(FacingDnswe::East),
            x => Err(anyhow::anyhow!("invalid value {} for FacingDnswe", x)),
        }
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum FacingNeswud {
    North,
    East,
    South,
    West,
    Up,
    Down,
}
impl TryFrom<u16> for FacingNeswud {
    type Error = anyhow::Error;
    fn try_from(value: u16) -> anyhow::Result<Self> {
        match value {
            0u16 => Ok(FacingNeswud::North),
            1u16 => Ok(FacingNeswud::East),
            2u16 => Ok(FacingNeswud::South),
            3u16 => Ok(FacingNeswud::West),
            4u16 => Ok(FacingNeswud::Up),
            5u16 => Ok(FacingNeswud::Down),
            x => Err(anyhow::anyhow!("invalid value {} for FacingNeswud", x)),
        }
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum FacingNswe {
    North,
    South,
    West,
    East,
}
impl TryFrom<u16> for FacingNswe {
    type Error = anyhow::Error;
    fn try_from(value: u16) -> anyhow::Result<Self> {
        match value {
            0u16 => Ok(FacingNswe::North),
            1u16 => Ok(FacingNswe::South),
            2u16 => Ok(FacingNswe::West),
            3u16 => Ok(FacingNswe::East),
            x => Err(anyhow::anyhow!("invalid value {} for FacingNswe", x)),
        }
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum HalfTb {
    Top,
    Bottom,
}
impl TryFrom<u16> for HalfTb {
    type Error = anyhow::Error;
    fn try_from(value: u16) -> anyhow::Result<Self> {
        match value {
            0u16 => Ok(HalfTb::Top),
            1u16 => Ok(HalfTb::Bottom),
            x => Err(anyhow::anyhow!("invalid value {} for HalfTb", x)),
        }
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum HalfUl {
    Upper,
    Lower,
}
impl TryFrom<u16> for HalfUl {
    type Error = anyhow::Error;
    fn try_from(value: u16) -> anyhow::Result<Self> {
        match value {
            0u16 => Ok(HalfUl::Upper),
            1u16 => Ok(HalfUl::Lower),
            x => Err(anyhow::anyhow!("invalid value {} for HalfUl", x)),
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
pub enum KindNs {
    Normal,
    Sticky,
}
impl TryFrom<u16> for KindNs {
    type Error = anyhow::Error;
    fn try_from(value: u16) -> anyhow::Result<Self> {
        match value {
            0u16 => Ok(KindNs::Normal),
            1u16 => Ok(KindNs::Sticky),
            x => Err(anyhow::anyhow!("invalid value {} for KindNs", x)),
        }
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum KindSlr {
    Single,
    Left,
    Right,
}
impl TryFrom<u16> for KindSlr {
    type Error = anyhow::Error;
    fn try_from(value: u16) -> anyhow::Result<Self> {
        match value {
            0u16 => Ok(KindSlr::Single),
            1u16 => Ok(KindSlr::Left),
            2u16 => Ok(KindSlr::Right),
            x => Err(anyhow::anyhow!("invalid value {} for KindSlr", x)),
        }
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum KindTbd {
    Top,
    Bottom,
    Double,
}
impl TryFrom<u16> for KindTbd {
    type Error = anyhow::Error;
    fn try_from(value: u16) -> anyhow::Result<Self> {
        match value {
            0u16 => Ok(KindTbd::Top),
            1u16 => Ok(KindTbd::Bottom),
            2u16 => Ok(KindTbd::Double),
            x => Err(anyhow::anyhow!("invalid value {} for KindTbd", x)),
        }
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum ModeCs {
    Compare,
    Subtract,
}
impl TryFrom<u16> for ModeCs {
    type Error = anyhow::Error;
    fn try_from(value: u16) -> anyhow::Result<Self> {
        match value {
            0u16 => Ok(ModeCs::Compare),
            1u16 => Ok(ModeCs::Subtract),
            x => Err(anyhow::anyhow!("invalid value {} for ModeCs", x)),
        }
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum ModeSlcd {
    Save,
    Load,
    Corner,
    Data,
}
impl TryFrom<u16> for ModeSlcd {
    type Error = anyhow::Error;
    fn try_from(value: u16) -> anyhow::Result<Self> {
        match value {
            0u16 => Ok(ModeSlcd::Save),
            1u16 => Ok(ModeSlcd::Load),
            2u16 => Ok(ModeSlcd::Corner),
            3u16 => Ok(ModeSlcd::Data),
            x => Err(anyhow::anyhow!("invalid value {} for ModeSlcd", x)),
        }
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum NorthUsn {
    Up,
    Side,
    None,
}
impl TryFrom<u16> for NorthUsn {
    type Error = anyhow::Error;
    fn try_from(value: u16) -> anyhow::Result<Self> {
        match value {
            0u16 => Ok(NorthUsn::Up),
            1u16 => Ok(NorthUsn::Side),
            2u16 => Ok(NorthUsn::None),
            x => Err(anyhow::anyhow!("invalid value {} for NorthUsn", x)),
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
pub enum ShapeNeaaaa {
    NorthSouth,
    EastWest,
    AscendingEast,
    AscendingWest,
    AscendingNorth,
    AscendingSouth,
}
impl TryFrom<u16> for ShapeNeaaaa {
    type Error = anyhow::Error;
    fn try_from(value: u16) -> anyhow::Result<Self> {
        match value {
            0u16 => Ok(ShapeNeaaaa::NorthSouth),
            1u16 => Ok(ShapeNeaaaa::EastWest),
            2u16 => Ok(ShapeNeaaaa::AscendingEast),
            3u16 => Ok(ShapeNeaaaa::AscendingWest),
            4u16 => Ok(ShapeNeaaaa::AscendingNorth),
            5u16 => Ok(ShapeNeaaaa::AscendingSouth),
            x => Err(anyhow::anyhow!("invalid value {} for ShapeNeaaaa", x)),
        }
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum ShapeNeaaaassnn {
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
impl TryFrom<u16> for ShapeNeaaaassnn {
    type Error = anyhow::Error;
    fn try_from(value: u16) -> anyhow::Result<Self> {
        match value {
            0u16 => Ok(ShapeNeaaaassnn::NorthSouth),
            1u16 => Ok(ShapeNeaaaassnn::EastWest),
            2u16 => Ok(ShapeNeaaaassnn::AscendingEast),
            3u16 => Ok(ShapeNeaaaassnn::AscendingWest),
            4u16 => Ok(ShapeNeaaaassnn::AscendingNorth),
            5u16 => Ok(ShapeNeaaaassnn::AscendingSouth),
            6u16 => Ok(ShapeNeaaaassnn::SouthEast),
            7u16 => Ok(ShapeNeaaaassnn::SouthWest),
            8u16 => Ok(ShapeNeaaaassnn::NorthWest),
            9u16 => Ok(ShapeNeaaaassnn::NorthEast),
            x => Err(anyhow::anyhow!("invalid value {} for ShapeNeaaaassnn", x)),
        }
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum ShapeSiioo {
    Straight,
    InnerLeft,
    InnerRight,
    OuterLeft,
    OuterRight,
}
impl TryFrom<u16> for ShapeSiioo {
    type Error = anyhow::Error;
    fn try_from(value: u16) -> anyhow::Result<Self> {
        match value {
            0u16 => Ok(ShapeSiioo::Straight),
            1u16 => Ok(ShapeSiioo::InnerLeft),
            2u16 => Ok(ShapeSiioo::InnerRight),
            3u16 => Ok(ShapeSiioo::OuterLeft),
            4u16 => Ok(ShapeSiioo::OuterRight),
            x => Err(anyhow::anyhow!("invalid value {} for ShapeSiioo", x)),
        }
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum SouthUsn {
    Up,
    Side,
    None,
}
impl TryFrom<u16> for SouthUsn {
    type Error = anyhow::Error;
    fn try_from(value: u16) -> anyhow::Result<Self> {
        match value {
            0u16 => Ok(SouthUsn::Up),
            1u16 => Ok(SouthUsn::Side),
            2u16 => Ok(SouthUsn::None),
            x => Err(anyhow::anyhow!("invalid value {} for SouthUsn", x)),
        }
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum WestUsn {
    Up,
    Side,
    None,
}
impl TryFrom<u16> for WestUsn {
    type Error = anyhow::Error;
    fn try_from(value: u16) -> anyhow::Result<Self> {
        match value {
            0u16 => Ok(WestUsn::Up),
            1u16 => Ok(WestUsn::Side),
            2u16 => Ok(WestUsn::None),
            x => Err(anyhow::anyhow!("invalid value {} for WestUsn", x)),
        }
    }
}
