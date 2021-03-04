use crate::BlockKind;
use serde::Deserialize;
use std::convert::TryFrom;
use std::str::FromStr;
#[derive(Debug, Deserialize)]
pub struct BlockTable {
    age_0_1: Vec<(u16, u16)>,
    age_0_15: Vec<(u16, u16)>,
    age_0_2: Vec<(u16, u16)>,
    age_0_25: Vec<(u16, u16)>,
    age_0_3: Vec<(u16, u16)>,
    age_0_5: Vec<(u16, u16)>,
    age_0_7: Vec<(u16, u16)>,
    attached: Vec<(u16, u16)>,
    attachment: Vec<(u16, u16)>,
    axis_xyz: Vec<(u16, u16)>,
    axis_xz: Vec<(u16, u16)>,
    bites: Vec<(u16, u16)>,
    bottom: Vec<(u16, u16)>,
    cauldron_level: Vec<(u16, u16)>,
    charges: Vec<(u16, u16)>,
    chest_kind: Vec<(u16, u16)>,
    comparator_mode: Vec<(u16, u16)>,
    conditional: Vec<(u16, u16)>,
    delay: Vec<(u16, u16)>,
    disarmed: Vec<(u16, u16)>,
    distance_0_7: Vec<(u16, u16)>,
    distance_1_7: Vec<(u16, u16)>,
    down: Vec<(u16, u16)>,
    drag: Vec<(u16, u16)>,
    east_connected: Vec<(u16, u16)>,
    east_nlt: Vec<(u16, u16)>,
    east_wire: Vec<(u16, u16)>,
    eggs: Vec<(u16, u16)>,
    enabled: Vec<(u16, u16)>,
    extended: Vec<(u16, u16)>,
    eye: Vec<(u16, u16)>,
    face: Vec<(u16, u16)>,
    facing_cardinal: Vec<(u16, u16)>,
    facing_cardinal_and_down: Vec<(u16, u16)>,
    facing_cubic: Vec<(u16, u16)>,
    half_top_bottom: Vec<(u16, u16)>,
    half_upper_lower: Vec<(u16, u16)>,
    hanging: Vec<(u16, u16)>,
    has_book: Vec<(u16, u16)>,
    has_bottle_0: Vec<(u16, u16)>,
    has_bottle_1: Vec<(u16, u16)>,
    has_bottle_2: Vec<(u16, u16)>,
    has_record: Vec<(u16, u16)>,
    hatch: Vec<(u16, u16)>,
    hinge: Vec<(u16, u16)>,
    honey_level: Vec<(u16, u16)>,
    in_wall: Vec<(u16, u16)>,
    instrument: Vec<(u16, u16)>,
    inverted: Vec<(u16, u16)>,
    layers: Vec<(u16, u16)>,
    leaves: Vec<(u16, u16)>,
    level_0_8: Vec<(u16, u16)>,
    lit: Vec<(u16, u16)>,
    locked: Vec<(u16, u16)>,
    moisture: Vec<(u16, u16)>,
    north_connected: Vec<(u16, u16)>,
    north_nlt: Vec<(u16, u16)>,
    north_wire: Vec<(u16, u16)>,
    note: Vec<(u16, u16)>,
    occupied: Vec<(u16, u16)>,
    open: Vec<(u16, u16)>,
    orientation: Vec<(u16, u16)>,
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
    signal_fire: Vec<(u16, u16)>,
    slab_kind: Vec<(u16, u16)>,
    snowy: Vec<(u16, u16)>,
    south_connected: Vec<(u16, u16)>,
    south_nlt: Vec<(u16, u16)>,
    south_wire: Vec<(u16, u16)>,
    stage: Vec<(u16, u16)>,
    stairs_shape: Vec<(u16, u16)>,
    structure_block_mode: Vec<(u16, u16)>,
    triggered: Vec<(u16, u16)>,
    unstable: Vec<(u16, u16)>,
    up: Vec<(u16, u16)>,
    water_level: Vec<(u16, u16)>,
    waterlogged: Vec<(u16, u16)>,
    west_connected: Vec<(u16, u16)>,
    west_nlt: Vec<(u16, u16)>,
    west_wire: Vec<(u16, u16)>,
}
impl BlockTable {
    #[doc = "Retrieves the `age_0_1` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn age_0_1(&self, kind: BlockKind, state: u16) -> Option<i32> {
        let (offset_coefficient, stride) = self.age_0_1[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some({ x as i32 + 0i32 })
    }
    #[doc = "Updates the state value for the given block kind such that its `age_0_1` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_age_0_1(&self, kind: BlockKind, state: u16, value: i32) -> Option<u16> {
        let (offset_coefficient, stride) = self.age_0_1[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 - 0u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
    }
    #[doc = "Retrieves the `age_0_15` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn age_0_15(&self, kind: BlockKind, state: u16) -> Option<i32> {
        let (offset_coefficient, stride) = self.age_0_15[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some({ x as i32 + 0i32 })
    }
    #[doc = "Updates the state value for the given block kind such that its `age_0_15` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_age_0_15(&self, kind: BlockKind, state: u16, value: i32) -> Option<u16> {
        let (offset_coefficient, stride) = self.age_0_15[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 - 0u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
    }
    #[doc = "Retrieves the `age_0_2` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn age_0_2(&self, kind: BlockKind, state: u16) -> Option<i32> {
        let (offset_coefficient, stride) = self.age_0_2[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some({ x as i32 + 0i32 })
    }
    #[doc = "Updates the state value for the given block kind such that its `age_0_2` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_age_0_2(&self, kind: BlockKind, state: u16, value: i32) -> Option<u16> {
        let (offset_coefficient, stride) = self.age_0_2[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 - 0u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
    }
    #[doc = "Retrieves the `age_0_25` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn age_0_25(&self, kind: BlockKind, state: u16) -> Option<i32> {
        let (offset_coefficient, stride) = self.age_0_25[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some({ x as i32 + 0i32 })
    }
    #[doc = "Updates the state value for the given block kind such that its `age_0_25` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_age_0_25(&self, kind: BlockKind, state: u16, value: i32) -> Option<u16> {
        let (offset_coefficient, stride) = self.age_0_25[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 - 0u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
    }
    #[doc = "Retrieves the `age_0_3` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn age_0_3(&self, kind: BlockKind, state: u16) -> Option<i32> {
        let (offset_coefficient, stride) = self.age_0_3[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some({ x as i32 + 0i32 })
    }
    #[doc = "Updates the state value for the given block kind such that its `age_0_3` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_age_0_3(&self, kind: BlockKind, state: u16, value: i32) -> Option<u16> {
        let (offset_coefficient, stride) = self.age_0_3[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 - 0u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
    }
    #[doc = "Retrieves the `age_0_5` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn age_0_5(&self, kind: BlockKind, state: u16) -> Option<i32> {
        let (offset_coefficient, stride) = self.age_0_5[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some({ x as i32 + 0i32 })
    }
    #[doc = "Updates the state value for the given block kind such that its `age_0_5` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_age_0_5(&self, kind: BlockKind, state: u16, value: i32) -> Option<u16> {
        let (offset_coefficient, stride) = self.age_0_5[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 - 0u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
    }
    #[doc = "Retrieves the `age_0_7` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn age_0_7(&self, kind: BlockKind, state: u16) -> Option<i32> {
        let (offset_coefficient, stride) = self.age_0_7[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some({ x as i32 + 0i32 })
    }
    #[doc = "Updates the state value for the given block kind such that its `age_0_7` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_age_0_7(&self, kind: BlockKind, state: u16, value: i32) -> Option<u16> {
        let (offset_coefficient, stride) = self.age_0_7[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 - 0u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
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
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
    }
    #[doc = "Retrieves the `attachment` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn attachment(&self, kind: BlockKind, state: u16) -> Option<Attachment> {
        let (offset_coefficient, stride) = self.attachment[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(Attachment::try_from(x).expect("invalid block state"))
    }
    #[doc = "Updates the state value for the given block kind such that its `attachment` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_attachment(&self, kind: BlockKind, state: u16, value: Attachment) -> Option<u16> {
        let (offset_coefficient, stride) = self.attachment[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
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
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
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
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
    }
    #[doc = "Retrieves the `bites` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn bites(&self, kind: BlockKind, state: u16) -> Option<i32> {
        let (offset_coefficient, stride) = self.bites[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some({ x as i32 + 0i32 })
    }
    #[doc = "Updates the state value for the given block kind such that its `bites` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_bites(&self, kind: BlockKind, state: u16, value: i32) -> Option<u16> {
        let (offset_coefficient, stride) = self.bites[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 - 0u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
    }
    #[doc = "Retrieves the `bottom` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn bottom(&self, kind: BlockKind, state: u16) -> Option<bool> {
        let (offset_coefficient, stride) = self.bottom[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(if x == 0 { false } else { true })
    }
    #[doc = "Updates the state value for the given block kind such that its `bottom` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_bottom(&self, kind: BlockKind, state: u16, value: bool) -> Option<u16> {
        let (offset_coefficient, stride) = self.bottom[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
    }
    #[doc = "Retrieves the `cauldron_level` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn cauldron_level(&self, kind: BlockKind, state: u16) -> Option<i32> {
        let (offset_coefficient, stride) = self.cauldron_level[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some({ x as i32 + 0i32 })
    }
    #[doc = "Updates the state value for the given block kind such that its `cauldron_level` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_cauldron_level(&self, kind: BlockKind, state: u16, value: i32) -> Option<u16> {
        let (offset_coefficient, stride) = self.cauldron_level[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 - 0u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
    }
    #[doc = "Retrieves the `charges` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn charges(&self, kind: BlockKind, state: u16) -> Option<i32> {
        let (offset_coefficient, stride) = self.charges[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some({ x as i32 + 0i32 })
    }
    #[doc = "Updates the state value for the given block kind such that its `charges` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_charges(&self, kind: BlockKind, state: u16, value: i32) -> Option<u16> {
        let (offset_coefficient, stride) = self.charges[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 - 0u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
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
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
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
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
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
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
    }
    #[doc = "Retrieves the `delay` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn delay(&self, kind: BlockKind, state: u16) -> Option<i32> {
        let (offset_coefficient, stride) = self.delay[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some({ x as i32 + 1i32 })
    }
    #[doc = "Updates the state value for the given block kind such that its `delay` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_delay(&self, kind: BlockKind, state: u16, value: i32) -> Option<u16> {
        let (offset_coefficient, stride) = self.delay[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 - 1u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
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
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
    }
    #[doc = "Retrieves the `distance_0_7` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn distance_0_7(&self, kind: BlockKind, state: u16) -> Option<i32> {
        let (offset_coefficient, stride) = self.distance_0_7[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some({ x as i32 + 0i32 })
    }
    #[doc = "Updates the state value for the given block kind such that its `distance_0_7` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_distance_0_7(&self, kind: BlockKind, state: u16, value: i32) -> Option<u16> {
        let (offset_coefficient, stride) = self.distance_0_7[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 - 0u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
    }
    #[doc = "Retrieves the `distance_1_7` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn distance_1_7(&self, kind: BlockKind, state: u16) -> Option<i32> {
        let (offset_coefficient, stride) = self.distance_1_7[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some({ x as i32 + 1i32 })
    }
    #[doc = "Updates the state value for the given block kind such that its `distance_1_7` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_distance_1_7(&self, kind: BlockKind, state: u16, value: i32) -> Option<u16> {
        let (offset_coefficient, stride) = self.distance_1_7[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 - 1u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
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
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
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
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
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
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
    }
    #[doc = "Retrieves the `east_nlt` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn east_nlt(&self, kind: BlockKind, state: u16) -> Option<EastNlt> {
        let (offset_coefficient, stride) = self.east_nlt[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(EastNlt::try_from(x).expect("invalid block state"))
    }
    #[doc = "Updates the state value for the given block kind such that its `east_nlt` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_east_nlt(&self, kind: BlockKind, state: u16, value: EastNlt) -> Option<u16> {
        let (offset_coefficient, stride) = self.east_nlt[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
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
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
    }
    #[doc = "Retrieves the `eggs` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn eggs(&self, kind: BlockKind, state: u16) -> Option<i32> {
        let (offset_coefficient, stride) = self.eggs[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some({ x as i32 + 1i32 })
    }
    #[doc = "Updates the state value for the given block kind such that its `eggs` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_eggs(&self, kind: BlockKind, state: u16, value: i32) -> Option<u16> {
        let (offset_coefficient, stride) = self.eggs[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 - 1u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
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
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
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
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
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
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
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
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
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
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
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
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
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
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
    }
    #[doc = "Retrieves the `half_top_bottom` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn half_top_bottom(&self, kind: BlockKind, state: u16) -> Option<HalfTopBottom> {
        let (offset_coefficient, stride) = self.half_top_bottom[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(HalfTopBottom::try_from(x).expect("invalid block state"))
    }
    #[doc = "Updates the state value for the given block kind such that its `half_top_bottom` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_half_top_bottom(
        &self,
        kind: BlockKind,
        state: u16,
        value: HalfTopBottom,
    ) -> Option<u16> {
        let (offset_coefficient, stride) = self.half_top_bottom[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
    }
    #[doc = "Retrieves the `half_upper_lower` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn half_upper_lower(&self, kind: BlockKind, state: u16) -> Option<HalfUpperLower> {
        let (offset_coefficient, stride) = self.half_upper_lower[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(HalfUpperLower::try_from(x).expect("invalid block state"))
    }
    #[doc = "Updates the state value for the given block kind such that its `half_upper_lower` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_half_upper_lower(
        &self,
        kind: BlockKind,
        state: u16,
        value: HalfUpperLower,
    ) -> Option<u16> {
        let (offset_coefficient, stride) = self.half_upper_lower[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
    }
    #[doc = "Retrieves the `hanging` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn hanging(&self, kind: BlockKind, state: u16) -> Option<bool> {
        let (offset_coefficient, stride) = self.hanging[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(if x == 0 { false } else { true })
    }
    #[doc = "Updates the state value for the given block kind such that its `hanging` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_hanging(&self, kind: BlockKind, state: u16, value: bool) -> Option<u16> {
        let (offset_coefficient, stride) = self.hanging[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
    }
    #[doc = "Retrieves the `has_book` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn has_book(&self, kind: BlockKind, state: u16) -> Option<bool> {
        let (offset_coefficient, stride) = self.has_book[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(if x == 0 { false } else { true })
    }
    #[doc = "Updates the state value for the given block kind such that its `has_book` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_has_book(&self, kind: BlockKind, state: u16, value: bool) -> Option<u16> {
        let (offset_coefficient, stride) = self.has_book[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
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
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
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
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
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
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
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
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
    }
    #[doc = "Retrieves the `hatch` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn hatch(&self, kind: BlockKind, state: u16) -> Option<i32> {
        let (offset_coefficient, stride) = self.hatch[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some({ x as i32 + 0i32 })
    }
    #[doc = "Updates the state value for the given block kind such that its `hatch` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_hatch(&self, kind: BlockKind, state: u16, value: i32) -> Option<u16> {
        let (offset_coefficient, stride) = self.hatch[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 - 0u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
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
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
    }
    #[doc = "Retrieves the `honey_level` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn honey_level(&self, kind: BlockKind, state: u16) -> Option<i32> {
        let (offset_coefficient, stride) = self.honey_level[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some({ x as i32 + 0i32 })
    }
    #[doc = "Updates the state value for the given block kind such that its `honey_level` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_honey_level(&self, kind: BlockKind, state: u16, value: i32) -> Option<u16> {
        let (offset_coefficient, stride) = self.honey_level[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 - 0u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
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
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
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
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
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
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
    }
    #[doc = "Retrieves the `layers` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn layers(&self, kind: BlockKind, state: u16) -> Option<i32> {
        let (offset_coefficient, stride) = self.layers[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some({ x as i32 + 1i32 })
    }
    #[doc = "Updates the state value for the given block kind such that its `layers` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_layers(&self, kind: BlockKind, state: u16, value: i32) -> Option<u16> {
        let (offset_coefficient, stride) = self.layers[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 - 1u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
    }
    #[doc = "Retrieves the `leaves` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn leaves(&self, kind: BlockKind, state: u16) -> Option<Leaves> {
        let (offset_coefficient, stride) = self.leaves[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(Leaves::try_from(x).expect("invalid block state"))
    }
    #[doc = "Updates the state value for the given block kind such that its `leaves` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_leaves(&self, kind: BlockKind, state: u16, value: Leaves) -> Option<u16> {
        let (offset_coefficient, stride) = self.leaves[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
    }
    #[doc = "Retrieves the `level_0_8` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn level_0_8(&self, kind: BlockKind, state: u16) -> Option<i32> {
        let (offset_coefficient, stride) = self.level_0_8[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some({ x as i32 + 0i32 })
    }
    #[doc = "Updates the state value for the given block kind such that its `level_0_8` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_level_0_8(&self, kind: BlockKind, state: u16, value: i32) -> Option<u16> {
        let (offset_coefficient, stride) = self.level_0_8[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 - 0u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
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
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
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
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
    }
    #[doc = "Retrieves the `moisture` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn moisture(&self, kind: BlockKind, state: u16) -> Option<i32> {
        let (offset_coefficient, stride) = self.moisture[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some({ x as i32 + 0i32 })
    }
    #[doc = "Updates the state value for the given block kind such that its `moisture` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_moisture(&self, kind: BlockKind, state: u16, value: i32) -> Option<u16> {
        let (offset_coefficient, stride) = self.moisture[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 - 0u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
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
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
    }
    #[doc = "Retrieves the `north_nlt` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn north_nlt(&self, kind: BlockKind, state: u16) -> Option<NorthNlt> {
        let (offset_coefficient, stride) = self.north_nlt[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(NorthNlt::try_from(x).expect("invalid block state"))
    }
    #[doc = "Updates the state value for the given block kind such that its `north_nlt` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_north_nlt(&self, kind: BlockKind, state: u16, value: NorthNlt) -> Option<u16> {
        let (offset_coefficient, stride) = self.north_nlt[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
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
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
    }
    #[doc = "Retrieves the `note` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn note(&self, kind: BlockKind, state: u16) -> Option<i32> {
        let (offset_coefficient, stride) = self.note[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some({ x as i32 + 0i32 })
    }
    #[doc = "Updates the state value for the given block kind such that its `note` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_note(&self, kind: BlockKind, state: u16, value: i32) -> Option<u16> {
        let (offset_coefficient, stride) = self.note[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 - 0u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
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
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
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
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
    }
    #[doc = "Retrieves the `orientation` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn orientation(&self, kind: BlockKind, state: u16) -> Option<Orientation> {
        let (offset_coefficient, stride) = self.orientation[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(Orientation::try_from(x).expect("invalid block state"))
    }
    #[doc = "Updates the state value for the given block kind such that its `orientation` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_orientation(&self, kind: BlockKind, state: u16, value: Orientation) -> Option<u16> {
        let (offset_coefficient, stride) = self.orientation[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
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
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
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
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
    }
    #[doc = "Retrieves the `pickles` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn pickles(&self, kind: BlockKind, state: u16) -> Option<i32> {
        let (offset_coefficient, stride) = self.pickles[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some({ x as i32 + 1i32 })
    }
    #[doc = "Updates the state value for the given block kind such that its `pickles` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_pickles(&self, kind: BlockKind, state: u16, value: i32) -> Option<u16> {
        let (offset_coefficient, stride) = self.pickles[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 - 1u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
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
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
    }
    #[doc = "Retrieves the `power` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn power(&self, kind: BlockKind, state: u16) -> Option<i32> {
        let (offset_coefficient, stride) = self.power[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some({ x as i32 + 0i32 })
    }
    #[doc = "Updates the state value for the given block kind such that its `power` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_power(&self, kind: BlockKind, state: u16, value: i32) -> Option<u16> {
        let (offset_coefficient, stride) = self.power[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 - 0u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
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
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
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
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
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
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
    }
    #[doc = "Retrieves the `rotation` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn rotation(&self, kind: BlockKind, state: u16) -> Option<i32> {
        let (offset_coefficient, stride) = self.rotation[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some({ x as i32 + 0i32 })
    }
    #[doc = "Updates the state value for the given block kind such that its `rotation` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_rotation(&self, kind: BlockKind, state: u16, value: i32) -> Option<u16> {
        let (offset_coefficient, stride) = self.rotation[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 - 0u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
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
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
    }
    #[doc = "Retrieves the `signal_fire` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn signal_fire(&self, kind: BlockKind, state: u16) -> Option<bool> {
        let (offset_coefficient, stride) = self.signal_fire[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(if x == 0 { false } else { true })
    }
    #[doc = "Updates the state value for the given block kind such that its `signal_fire` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_signal_fire(&self, kind: BlockKind, state: u16, value: bool) -> Option<u16> {
        let (offset_coefficient, stride) = self.signal_fire[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
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
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
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
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
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
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
    }
    #[doc = "Retrieves the `south_nlt` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn south_nlt(&self, kind: BlockKind, state: u16) -> Option<SouthNlt> {
        let (offset_coefficient, stride) = self.south_nlt[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(SouthNlt::try_from(x).expect("invalid block state"))
    }
    #[doc = "Updates the state value for the given block kind such that its `south_nlt` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_south_nlt(&self, kind: BlockKind, state: u16, value: SouthNlt) -> Option<u16> {
        let (offset_coefficient, stride) = self.south_nlt[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
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
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
    }
    #[doc = "Retrieves the `stage` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn stage(&self, kind: BlockKind, state: u16) -> Option<i32> {
        let (offset_coefficient, stride) = self.stage[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some({ x as i32 + 0i32 })
    }
    #[doc = "Updates the state value for the given block kind such that its `stage` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_stage(&self, kind: BlockKind, state: u16, value: i32) -> Option<u16> {
        let (offset_coefficient, stride) = self.stage[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 - 0u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
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
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
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
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
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
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
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
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
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
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
    }
    #[doc = "Retrieves the `water_level` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn water_level(&self, kind: BlockKind, state: u16) -> Option<i32> {
        let (offset_coefficient, stride) = self.water_level[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some({ x as i32 + 0i32 })
    }
    #[doc = "Updates the state value for the given block kind such that its `water_level` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_water_level(&self, kind: BlockKind, state: u16, value: i32) -> Option<u16> {
        let (offset_coefficient, stride) = self.water_level[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 - 0u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
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
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
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
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
    }
    #[doc = "Retrieves the `west_nlt` value for the given block kind with the given state value.\n        Returns the value of the property, or `None` if it does not exist."]
    pub fn west_nlt(&self, kind: BlockKind, state: u16) -> Option<WestNlt> {
        let (offset_coefficient, stride) = self.west_nlt[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(state, offset_coefficient, stride);
        Some(WestNlt::try_from(x).expect("invalid block state"))
    }
    #[doc = "Updates the state value for the given block kind such that its `west_nlt` value is updated. Returns the new state,\n        or `None` if the block does not have this property."]
    pub fn set_west_nlt(&self, kind: BlockKind, state: u16, value: WestNlt) -> Option<u16> {
        let (offset_coefficient, stride) = self.west_nlt[kind as u16 as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
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
        let old = crate::n_dimensional_index(state, offset_coefficient, stride) as i32;
        let new = ({ value as u16 } as i32 - old) * stride as i32 + state as i32;
        Some(new as u16)
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum Attachment {
    Floor,
    Ceiling,
    SingleWall,
    DoubleWall,
}
impl TryFrom<u16> for Attachment {
    type Error = anyhow::Error;
    fn try_from(value: u16) -> anyhow::Result<Self> {
        match value {
            0u16 => Ok(Attachment::Floor),
            1u16 => Ok(Attachment::Ceiling),
            2u16 => Ok(Attachment::SingleWall),
            3u16 => Ok(Attachment::DoubleWall),
            x => Err(anyhow::anyhow!("invalid value {} for Attachment", x)),
        }
    }
}
impl FromStr for Attachment {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> anyhow::Result<Self> {
        match s {
            "floor" => Ok(Attachment::Floor),
            "ceiling" => Ok(Attachment::Ceiling),
            "single_wall" => Ok(Attachment::SingleWall),
            "double_wall" => Ok(Attachment::DoubleWall),
            _ => Err(anyhow::anyhow!(
                "invalid value for {}",
                stringify!(Attachment)
            )),
        }
    }
}
impl Attachment {
    pub fn as_str(self) -> &'static str {
        match self {
            Attachment::Floor => "floor",
            Attachment::Ceiling => "ceiling",
            Attachment::SingleWall => "single_wall",
            Attachment::DoubleWall => "double_wall",
        }
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
impl FromStr for AxisXyz {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> anyhow::Result<Self> {
        match s {
            "x" => Ok(AxisXyz::X),
            "y" => Ok(AxisXyz::Y),
            "z" => Ok(AxisXyz::Z),
            _ => Err(anyhow::anyhow!("invalid value for {}", stringify!(AxisXyz))),
        }
    }
}
impl AxisXyz {
    pub fn as_str(self) -> &'static str {
        match self {
            AxisXyz::X => "x",
            AxisXyz::Y => "y",
            AxisXyz::Z => "z",
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
impl FromStr for AxisXz {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> anyhow::Result<Self> {
        match s {
            "x" => Ok(AxisXz::X),
            "z" => Ok(AxisXz::Z),
            _ => Err(anyhow::anyhow!("invalid value for {}", stringify!(AxisXz))),
        }
    }
}
impl AxisXz {
    pub fn as_str(self) -> &'static str {
        match self {
            AxisXz::X => "x",
            AxisXz::Z => "z",
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
impl FromStr for ChestKind {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> anyhow::Result<Self> {
        match s {
            "single" => Ok(ChestKind::Single),
            "left" => Ok(ChestKind::Left),
            "right" => Ok(ChestKind::Right),
            _ => Err(anyhow::anyhow!(
                "invalid value for {}",
                stringify!(ChestKind)
            )),
        }
    }
}
impl ChestKind {
    pub fn as_str(self) -> &'static str {
        match self {
            ChestKind::Single => "single",
            ChestKind::Left => "left",
            ChestKind::Right => "right",
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
impl FromStr for ComparatorMode {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> anyhow::Result<Self> {
        match s {
            "compare" => Ok(ComparatorMode::Compare),
            "subtract" => Ok(ComparatorMode::Subtract),
            _ => Err(anyhow::anyhow!(
                "invalid value for {}",
                stringify!(ComparatorMode)
            )),
        }
    }
}
impl ComparatorMode {
    pub fn as_str(self) -> &'static str {
        match self {
            ComparatorMode::Compare => "compare",
            ComparatorMode::Subtract => "subtract",
        }
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum EastNlt {
    None,
    Low,
    Tall,
}
impl TryFrom<u16> for EastNlt {
    type Error = anyhow::Error;
    fn try_from(value: u16) -> anyhow::Result<Self> {
        match value {
            0u16 => Ok(EastNlt::None),
            1u16 => Ok(EastNlt::Low),
            2u16 => Ok(EastNlt::Tall),
            x => Err(anyhow::anyhow!("invalid value {} for EastNlt", x)),
        }
    }
}
impl FromStr for EastNlt {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> anyhow::Result<Self> {
        match s {
            "none" => Ok(EastNlt::None),
            "low" => Ok(EastNlt::Low),
            "tall" => Ok(EastNlt::Tall),
            _ => Err(anyhow::anyhow!("invalid value for {}", stringify!(EastNlt))),
        }
    }
}
impl EastNlt {
    pub fn as_str(self) -> &'static str {
        match self {
            EastNlt::None => "none",
            EastNlt::Low => "low",
            EastNlt::Tall => "tall",
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
impl FromStr for EastWire {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> anyhow::Result<Self> {
        match s {
            "up" => Ok(EastWire::Up),
            "side" => Ok(EastWire::Side),
            "none" => Ok(EastWire::None),
            _ => Err(anyhow::anyhow!(
                "invalid value for {}",
                stringify!(EastWire)
            )),
        }
    }
}
impl EastWire {
    pub fn as_str(self) -> &'static str {
        match self {
            EastWire::Up => "up",
            EastWire::Side => "side",
            EastWire::None => "none",
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
impl FromStr for Face {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> anyhow::Result<Self> {
        match s {
            "floor" => Ok(Face::Floor),
            "wall" => Ok(Face::Wall),
            "ceiling" => Ok(Face::Ceiling),
            _ => Err(anyhow::anyhow!("invalid value for {}", stringify!(Face))),
        }
    }
}
impl Face {
    pub fn as_str(self) -> &'static str {
        match self {
            Face::Floor => "floor",
            Face::Wall => "wall",
            Face::Ceiling => "ceiling",
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
impl FromStr for FacingCardinal {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> anyhow::Result<Self> {
        match s {
            "north" => Ok(FacingCardinal::North),
            "south" => Ok(FacingCardinal::South),
            "west" => Ok(FacingCardinal::West),
            "east" => Ok(FacingCardinal::East),
            _ => Err(anyhow::anyhow!(
                "invalid value for {}",
                stringify!(FacingCardinal)
            )),
        }
    }
}
impl FacingCardinal {
    pub fn as_str(self) -> &'static str {
        match self {
            FacingCardinal::North => "north",
            FacingCardinal::South => "south",
            FacingCardinal::West => "west",
            FacingCardinal::East => "east",
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
impl FromStr for FacingCardinalAndDown {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> anyhow::Result<Self> {
        match s {
            "down" => Ok(FacingCardinalAndDown::Down),
            "north" => Ok(FacingCardinalAndDown::North),
            "south" => Ok(FacingCardinalAndDown::South),
            "west" => Ok(FacingCardinalAndDown::West),
            "east" => Ok(FacingCardinalAndDown::East),
            _ => Err(anyhow::anyhow!(
                "invalid value for {}",
                stringify!(FacingCardinalAndDown)
            )),
        }
    }
}
impl FacingCardinalAndDown {
    pub fn as_str(self) -> &'static str {
        match self {
            FacingCardinalAndDown::Down => "down",
            FacingCardinalAndDown::North => "north",
            FacingCardinalAndDown::South => "south",
            FacingCardinalAndDown::West => "west",
            FacingCardinalAndDown::East => "east",
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
impl FromStr for FacingCubic {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> anyhow::Result<Self> {
        match s {
            "north" => Ok(FacingCubic::North),
            "east" => Ok(FacingCubic::East),
            "south" => Ok(FacingCubic::South),
            "west" => Ok(FacingCubic::West),
            "up" => Ok(FacingCubic::Up),
            "down" => Ok(FacingCubic::Down),
            _ => Err(anyhow::anyhow!(
                "invalid value for {}",
                stringify!(FacingCubic)
            )),
        }
    }
}
impl FacingCubic {
    pub fn as_str(self) -> &'static str {
        match self {
            FacingCubic::North => "north",
            FacingCubic::East => "east",
            FacingCubic::South => "south",
            FacingCubic::West => "west",
            FacingCubic::Up => "up",
            FacingCubic::Down => "down",
        }
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum HalfTopBottom {
    Top,
    Bottom,
}
impl TryFrom<u16> for HalfTopBottom {
    type Error = anyhow::Error;
    fn try_from(value: u16) -> anyhow::Result<Self> {
        match value {
            0u16 => Ok(HalfTopBottom::Top),
            1u16 => Ok(HalfTopBottom::Bottom),
            x => Err(anyhow::anyhow!("invalid value {} for HalfTopBottom", x)),
        }
    }
}
impl FromStr for HalfTopBottom {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> anyhow::Result<Self> {
        match s {
            "top" => Ok(HalfTopBottom::Top),
            "bottom" => Ok(HalfTopBottom::Bottom),
            _ => Err(anyhow::anyhow!(
                "invalid value for {}",
                stringify!(HalfTopBottom)
            )),
        }
    }
}
impl HalfTopBottom {
    pub fn as_str(self) -> &'static str {
        match self {
            HalfTopBottom::Top => "top",
            HalfTopBottom::Bottom => "bottom",
        }
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum HalfUpperLower {
    Upper,
    Lower,
}
impl TryFrom<u16> for HalfUpperLower {
    type Error = anyhow::Error;
    fn try_from(value: u16) -> anyhow::Result<Self> {
        match value {
            0u16 => Ok(HalfUpperLower::Upper),
            1u16 => Ok(HalfUpperLower::Lower),
            x => Err(anyhow::anyhow!("invalid value {} for HalfUpperLower", x)),
        }
    }
}
impl FromStr for HalfUpperLower {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> anyhow::Result<Self> {
        match s {
            "upper" => Ok(HalfUpperLower::Upper),
            "lower" => Ok(HalfUpperLower::Lower),
            _ => Err(anyhow::anyhow!(
                "invalid value for {}",
                stringify!(HalfUpperLower)
            )),
        }
    }
}
impl HalfUpperLower {
    pub fn as_str(self) -> &'static str {
        match self {
            HalfUpperLower::Upper => "upper",
            HalfUpperLower::Lower => "lower",
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
impl FromStr for Hinge {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> anyhow::Result<Self> {
        match s {
            "left" => Ok(Hinge::Left),
            "right" => Ok(Hinge::Right),
            _ => Err(anyhow::anyhow!("invalid value for {}", stringify!(Hinge))),
        }
    }
}
impl Hinge {
    pub fn as_str(self) -> &'static str {
        match self {
            Hinge::Left => "left",
            Hinge::Right => "right",
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
    IronXylophone,
    CowBell,
    Didgeridoo,
    Bit,
    Banjo,
    Pling,
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
            10u16 => Ok(Instrument::IronXylophone),
            11u16 => Ok(Instrument::CowBell),
            12u16 => Ok(Instrument::Didgeridoo),
            13u16 => Ok(Instrument::Bit),
            14u16 => Ok(Instrument::Banjo),
            15u16 => Ok(Instrument::Pling),
            x => Err(anyhow::anyhow!("invalid value {} for Instrument", x)),
        }
    }
}
impl FromStr for Instrument {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> anyhow::Result<Self> {
        match s {
            "harp" => Ok(Instrument::Harp),
            "basedrum" => Ok(Instrument::Basedrum),
            "snare" => Ok(Instrument::Snare),
            "hat" => Ok(Instrument::Hat),
            "bass" => Ok(Instrument::Bass),
            "flute" => Ok(Instrument::Flute),
            "bell" => Ok(Instrument::Bell),
            "guitar" => Ok(Instrument::Guitar),
            "chime" => Ok(Instrument::Chime),
            "xylophone" => Ok(Instrument::Xylophone),
            "iron_xylophone" => Ok(Instrument::IronXylophone),
            "cow_bell" => Ok(Instrument::CowBell),
            "didgeridoo" => Ok(Instrument::Didgeridoo),
            "bit" => Ok(Instrument::Bit),
            "banjo" => Ok(Instrument::Banjo),
            "pling" => Ok(Instrument::Pling),
            _ => Err(anyhow::anyhow!(
                "invalid value for {}",
                stringify!(Instrument)
            )),
        }
    }
}
impl Instrument {
    pub fn as_str(self) -> &'static str {
        match self {
            Instrument::Harp => "harp",
            Instrument::Basedrum => "basedrum",
            Instrument::Snare => "snare",
            Instrument::Hat => "hat",
            Instrument::Bass => "bass",
            Instrument::Flute => "flute",
            Instrument::Bell => "bell",
            Instrument::Guitar => "guitar",
            Instrument::Chime => "chime",
            Instrument::Xylophone => "xylophone",
            Instrument::IronXylophone => "iron_xylophone",
            Instrument::CowBell => "cow_bell",
            Instrument::Didgeridoo => "didgeridoo",
            Instrument::Bit => "bit",
            Instrument::Banjo => "banjo",
            Instrument::Pling => "pling",
        }
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum Leaves {
    None,
    Small,
    Large,
}
impl TryFrom<u16> for Leaves {
    type Error = anyhow::Error;
    fn try_from(value: u16) -> anyhow::Result<Self> {
        match value {
            0u16 => Ok(Leaves::None),
            1u16 => Ok(Leaves::Small),
            2u16 => Ok(Leaves::Large),
            x => Err(anyhow::anyhow!("invalid value {} for Leaves", x)),
        }
    }
}
impl FromStr for Leaves {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> anyhow::Result<Self> {
        match s {
            "none" => Ok(Leaves::None),
            "small" => Ok(Leaves::Small),
            "large" => Ok(Leaves::Large),
            _ => Err(anyhow::anyhow!("invalid value for {}", stringify!(Leaves))),
        }
    }
}
impl Leaves {
    pub fn as_str(self) -> &'static str {
        match self {
            Leaves::None => "none",
            Leaves::Small => "small",
            Leaves::Large => "large",
        }
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum NorthNlt {
    None,
    Low,
    Tall,
}
impl TryFrom<u16> for NorthNlt {
    type Error = anyhow::Error;
    fn try_from(value: u16) -> anyhow::Result<Self> {
        match value {
            0u16 => Ok(NorthNlt::None),
            1u16 => Ok(NorthNlt::Low),
            2u16 => Ok(NorthNlt::Tall),
            x => Err(anyhow::anyhow!("invalid value {} for NorthNlt", x)),
        }
    }
}
impl FromStr for NorthNlt {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> anyhow::Result<Self> {
        match s {
            "none" => Ok(NorthNlt::None),
            "low" => Ok(NorthNlt::Low),
            "tall" => Ok(NorthNlt::Tall),
            _ => Err(anyhow::anyhow!(
                "invalid value for {}",
                stringify!(NorthNlt)
            )),
        }
    }
}
impl NorthNlt {
    pub fn as_str(self) -> &'static str {
        match self {
            NorthNlt::None => "none",
            NorthNlt::Low => "low",
            NorthNlt::Tall => "tall",
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
impl FromStr for NorthWire {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> anyhow::Result<Self> {
        match s {
            "up" => Ok(NorthWire::Up),
            "side" => Ok(NorthWire::Side),
            "none" => Ok(NorthWire::None),
            _ => Err(anyhow::anyhow!(
                "invalid value for {}",
                stringify!(NorthWire)
            )),
        }
    }
}
impl NorthWire {
    pub fn as_str(self) -> &'static str {
        match self {
            NorthWire::Up => "up",
            NorthWire::Side => "side",
            NorthWire::None => "none",
        }
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum Orientation {
    DownEast,
    DownNorth,
    DownSouth,
    DownWest,
    UpEast,
    UpNorth,
    UpSouth,
    UpWest,
    WestUp,
    EastUp,
    NorthUp,
    SouthUp,
}
impl TryFrom<u16> for Orientation {
    type Error = anyhow::Error;
    fn try_from(value: u16) -> anyhow::Result<Self> {
        match value {
            0u16 => Ok(Orientation::DownEast),
            1u16 => Ok(Orientation::DownNorth),
            2u16 => Ok(Orientation::DownSouth),
            3u16 => Ok(Orientation::DownWest),
            4u16 => Ok(Orientation::UpEast),
            5u16 => Ok(Orientation::UpNorth),
            6u16 => Ok(Orientation::UpSouth),
            7u16 => Ok(Orientation::UpWest),
            8u16 => Ok(Orientation::WestUp),
            9u16 => Ok(Orientation::EastUp),
            10u16 => Ok(Orientation::NorthUp),
            11u16 => Ok(Orientation::SouthUp),
            x => Err(anyhow::anyhow!("invalid value {} for Orientation", x)),
        }
    }
}
impl FromStr for Orientation {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> anyhow::Result<Self> {
        match s {
            "down_east" => Ok(Orientation::DownEast),
            "down_north" => Ok(Orientation::DownNorth),
            "down_south" => Ok(Orientation::DownSouth),
            "down_west" => Ok(Orientation::DownWest),
            "up_east" => Ok(Orientation::UpEast),
            "up_north" => Ok(Orientation::UpNorth),
            "up_south" => Ok(Orientation::UpSouth),
            "up_west" => Ok(Orientation::UpWest),
            "west_up" => Ok(Orientation::WestUp),
            "east_up" => Ok(Orientation::EastUp),
            "north_up" => Ok(Orientation::NorthUp),
            "south_up" => Ok(Orientation::SouthUp),
            _ => Err(anyhow::anyhow!(
                "invalid value for {}",
                stringify!(Orientation)
            )),
        }
    }
}
impl Orientation {
    pub fn as_str(self) -> &'static str {
        match self {
            Orientation::DownEast => "down_east",
            Orientation::DownNorth => "down_north",
            Orientation::DownSouth => "down_south",
            Orientation::DownWest => "down_west",
            Orientation::UpEast => "up_east",
            Orientation::UpNorth => "up_north",
            Orientation::UpSouth => "up_south",
            Orientation::UpWest => "up_west",
            Orientation::WestUp => "west_up",
            Orientation::EastUp => "east_up",
            Orientation::NorthUp => "north_up",
            Orientation::SouthUp => "south_up",
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
impl FromStr for Part {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> anyhow::Result<Self> {
        match s {
            "head" => Ok(Part::Head),
            "foot" => Ok(Part::Foot),
            _ => Err(anyhow::anyhow!("invalid value for {}", stringify!(Part))),
        }
    }
}
impl Part {
    pub fn as_str(self) -> &'static str {
        match self {
            Part::Head => "head",
            Part::Foot => "foot",
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
impl FromStr for PistonKind {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> anyhow::Result<Self> {
        match s {
            "normal" => Ok(PistonKind::Normal),
            "sticky" => Ok(PistonKind::Sticky),
            _ => Err(anyhow::anyhow!(
                "invalid value for {}",
                stringify!(PistonKind)
            )),
        }
    }
}
impl PistonKind {
    pub fn as_str(self) -> &'static str {
        match self {
            PistonKind::Normal => "normal",
            PistonKind::Sticky => "sticky",
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
impl FromStr for PoweredRailShape {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> anyhow::Result<Self> {
        match s {
            "north_south" => Ok(PoweredRailShape::NorthSouth),
            "east_west" => Ok(PoweredRailShape::EastWest),
            "ascending_east" => Ok(PoweredRailShape::AscendingEast),
            "ascending_west" => Ok(PoweredRailShape::AscendingWest),
            "ascending_north" => Ok(PoweredRailShape::AscendingNorth),
            "ascending_south" => Ok(PoweredRailShape::AscendingSouth),
            _ => Err(anyhow::anyhow!(
                "invalid value for {}",
                stringify!(PoweredRailShape)
            )),
        }
    }
}
impl PoweredRailShape {
    pub fn as_str(self) -> &'static str {
        match self {
            PoweredRailShape::NorthSouth => "north_south",
            PoweredRailShape::EastWest => "east_west",
            PoweredRailShape::AscendingEast => "ascending_east",
            PoweredRailShape::AscendingWest => "ascending_west",
            PoweredRailShape::AscendingNorth => "ascending_north",
            PoweredRailShape::AscendingSouth => "ascending_south",
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
impl FromStr for RailShape {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> anyhow::Result<Self> {
        match s {
            "north_south" => Ok(RailShape::NorthSouth),
            "east_west" => Ok(RailShape::EastWest),
            "ascending_east" => Ok(RailShape::AscendingEast),
            "ascending_west" => Ok(RailShape::AscendingWest),
            "ascending_north" => Ok(RailShape::AscendingNorth),
            "ascending_south" => Ok(RailShape::AscendingSouth),
            "south_east" => Ok(RailShape::SouthEast),
            "south_west" => Ok(RailShape::SouthWest),
            "north_west" => Ok(RailShape::NorthWest),
            "north_east" => Ok(RailShape::NorthEast),
            _ => Err(anyhow::anyhow!(
                "invalid value for {}",
                stringify!(RailShape)
            )),
        }
    }
}
impl RailShape {
    pub fn as_str(self) -> &'static str {
        match self {
            RailShape::NorthSouth => "north_south",
            RailShape::EastWest => "east_west",
            RailShape::AscendingEast => "ascending_east",
            RailShape::AscendingWest => "ascending_west",
            RailShape::AscendingNorth => "ascending_north",
            RailShape::AscendingSouth => "ascending_south",
            RailShape::SouthEast => "south_east",
            RailShape::SouthWest => "south_west",
            RailShape::NorthWest => "north_west",
            RailShape::NorthEast => "north_east",
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
impl FromStr for SlabKind {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> anyhow::Result<Self> {
        match s {
            "top" => Ok(SlabKind::Top),
            "bottom" => Ok(SlabKind::Bottom),
            "double" => Ok(SlabKind::Double),
            _ => Err(anyhow::anyhow!(
                "invalid value for {}",
                stringify!(SlabKind)
            )),
        }
    }
}
impl SlabKind {
    pub fn as_str(self) -> &'static str {
        match self {
            SlabKind::Top => "top",
            SlabKind::Bottom => "bottom",
            SlabKind::Double => "double",
        }
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum SouthNlt {
    None,
    Low,
    Tall,
}
impl TryFrom<u16> for SouthNlt {
    type Error = anyhow::Error;
    fn try_from(value: u16) -> anyhow::Result<Self> {
        match value {
            0u16 => Ok(SouthNlt::None),
            1u16 => Ok(SouthNlt::Low),
            2u16 => Ok(SouthNlt::Tall),
            x => Err(anyhow::anyhow!("invalid value {} for SouthNlt", x)),
        }
    }
}
impl FromStr for SouthNlt {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> anyhow::Result<Self> {
        match s {
            "none" => Ok(SouthNlt::None),
            "low" => Ok(SouthNlt::Low),
            "tall" => Ok(SouthNlt::Tall),
            _ => Err(anyhow::anyhow!(
                "invalid value for {}",
                stringify!(SouthNlt)
            )),
        }
    }
}
impl SouthNlt {
    pub fn as_str(self) -> &'static str {
        match self {
            SouthNlt::None => "none",
            SouthNlt::Low => "low",
            SouthNlt::Tall => "tall",
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
impl FromStr for SouthWire {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> anyhow::Result<Self> {
        match s {
            "up" => Ok(SouthWire::Up),
            "side" => Ok(SouthWire::Side),
            "none" => Ok(SouthWire::None),
            _ => Err(anyhow::anyhow!(
                "invalid value for {}",
                stringify!(SouthWire)
            )),
        }
    }
}
impl SouthWire {
    pub fn as_str(self) -> &'static str {
        match self {
            SouthWire::Up => "up",
            SouthWire::Side => "side",
            SouthWire::None => "none",
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
impl FromStr for StairsShape {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> anyhow::Result<Self> {
        match s {
            "straight" => Ok(StairsShape::Straight),
            "inner_left" => Ok(StairsShape::InnerLeft),
            "inner_right" => Ok(StairsShape::InnerRight),
            "outer_left" => Ok(StairsShape::OuterLeft),
            "outer_right" => Ok(StairsShape::OuterRight),
            _ => Err(anyhow::anyhow!(
                "invalid value for {}",
                stringify!(StairsShape)
            )),
        }
    }
}
impl StairsShape {
    pub fn as_str(self) -> &'static str {
        match self {
            StairsShape::Straight => "straight",
            StairsShape::InnerLeft => "inner_left",
            StairsShape::InnerRight => "inner_right",
            StairsShape::OuterLeft => "outer_left",
            StairsShape::OuterRight => "outer_right",
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
impl FromStr for StructureBlockMode {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> anyhow::Result<Self> {
        match s {
            "save" => Ok(StructureBlockMode::Save),
            "load" => Ok(StructureBlockMode::Load),
            "corner" => Ok(StructureBlockMode::Corner),
            "data" => Ok(StructureBlockMode::Data),
            _ => Err(anyhow::anyhow!(
                "invalid value for {}",
                stringify!(StructureBlockMode)
            )),
        }
    }
}
impl StructureBlockMode {
    pub fn as_str(self) -> &'static str {
        match self {
            StructureBlockMode::Save => "save",
            StructureBlockMode::Load => "load",
            StructureBlockMode::Corner => "corner",
            StructureBlockMode::Data => "data",
        }
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum WestNlt {
    None,
    Low,
    Tall,
}
impl TryFrom<u16> for WestNlt {
    type Error = anyhow::Error;
    fn try_from(value: u16) -> anyhow::Result<Self> {
        match value {
            0u16 => Ok(WestNlt::None),
            1u16 => Ok(WestNlt::Low),
            2u16 => Ok(WestNlt::Tall),
            x => Err(anyhow::anyhow!("invalid value {} for WestNlt", x)),
        }
    }
}
impl FromStr for WestNlt {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> anyhow::Result<Self> {
        match s {
            "none" => Ok(WestNlt::None),
            "low" => Ok(WestNlt::Low),
            "tall" => Ok(WestNlt::Tall),
            _ => Err(anyhow::anyhow!("invalid value for {}", stringify!(WestNlt))),
        }
    }
}
impl WestNlt {
    pub fn as_str(self) -> &'static str {
        match self {
            WestNlt::None => "none",
            WestNlt::Low => "low",
            WestNlt::Tall => "tall",
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
impl FromStr for WestWire {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> anyhow::Result<Self> {
        match s {
            "up" => Ok(WestWire::Up),
            "side" => Ok(WestWire::Side),
            "none" => Ok(WestWire::None),
            _ => Err(anyhow::anyhow!(
                "invalid value for {}",
                stringify!(WestWire)
            )),
        }
    }
}
impl WestWire {
    pub fn as_str(self) -> &'static str {
        match self {
            WestWire::Up => "up",
            WestWire::Side => "side",
            WestWire::None => "none",
        }
    }
}
