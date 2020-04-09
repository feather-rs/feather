use num_traits::FromPrimitive;
#[derive(Deserialize, Debug)]
pub struct BlocksTable {
    pub(crate) age: Vec<(u16, u16)>,
    pub(crate) attached: Vec<(u16, u16)>,
    pub(crate) axis: Vec<(u16, u16)>,
    pub(crate) bites: Vec<(u16, u16)>,
    pub(crate) conditional: Vec<(u16, u16)>,
    pub(crate) delay: Vec<(u16, u16)>,
    pub(crate) disarmed: Vec<(u16, u16)>,
    pub(crate) distance: Vec<(u16, u16)>,
    pub(crate) down: Vec<(u16, u16)>,
    pub(crate) drag: Vec<(u16, u16)>,
    pub(crate) east: Vec<(u16, u16)>,
    pub(crate) eggs: Vec<(u16, u16)>,
    pub(crate) enabled: Vec<(u16, u16)>,
    pub(crate) extended: Vec<(u16, u16)>,
    pub(crate) eye: Vec<(u16, u16)>,
    pub(crate) face: Vec<(u16, u16)>,
    pub(crate) facing: Vec<(u16, u16)>,
    pub(crate) half: Vec<(u16, u16)>,
    pub(crate) has_bottle_0: Vec<(u16, u16)>,
    pub(crate) has_bottle_1: Vec<(u16, u16)>,
    pub(crate) has_bottle_2: Vec<(u16, u16)>,
    pub(crate) has_record: Vec<(u16, u16)>,
    pub(crate) hatch: Vec<(u16, u16)>,
    pub(crate) hinge: Vec<(u16, u16)>,
    pub(crate) in_wall: Vec<(u16, u16)>,
    pub(crate) instrument: Vec<(u16, u16)>,
    pub(crate) inverted: Vec<(u16, u16)>,
    pub(crate) kind: Vec<(u16, u16)>,
    pub(crate) layers: Vec<(u16, u16)>,
    pub(crate) level: Vec<(u16, u16)>,
    pub(crate) lit: Vec<(u16, u16)>,
    pub(crate) locked: Vec<(u16, u16)>,
    pub(crate) mode: Vec<(u16, u16)>,
    pub(crate) moisture: Vec<(u16, u16)>,
    pub(crate) north: Vec<(u16, u16)>,
    pub(crate) note: Vec<(u16, u16)>,
    pub(crate) occupied: Vec<(u16, u16)>,
    pub(crate) open: Vec<(u16, u16)>,
    pub(crate) part: Vec<(u16, u16)>,
    pub(crate) persistent: Vec<(u16, u16)>,
    pub(crate) pickles: Vec<(u16, u16)>,
    pub(crate) power: Vec<(u16, u16)>,
    pub(crate) powered: Vec<(u16, u16)>,
    pub(crate) rotation: Vec<(u16, u16)>,
    pub(crate) shape: Vec<(u16, u16)>,
    pub(crate) short: Vec<(u16, u16)>,
    pub(crate) snowy: Vec<(u16, u16)>,
    pub(crate) south: Vec<(u16, u16)>,
    pub(crate) stage: Vec<(u16, u16)>,
    pub(crate) triggered: Vec<(u16, u16)>,
    pub(crate) unstable: Vec<(u16, u16)>,
    pub(crate) up: Vec<(u16, u16)>,
    pub(crate) waterlogged: Vec<(u16, u16)>,
    pub(crate) west: Vec<(u16, u16)>,
}
impl BlocksTable {
    pub fn age(&self, index: u16) -> Option<i32> {
        let (offset_coefficient, stride) = self.age[index as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(index, offset_coefficient, stride);
        i32::from_u16(x)
    }
    pub fn attached(&self, index: u16) -> Option<bool> {
        let (offset_coefficient, stride) = self.attached[index as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(index, offset_coefficient, stride);
        match x {
            1 => Some(true),
            0 => Some(false),
            _ => None,
        }
    }
    pub fn axis(&self, index: u16) -> Option<BoneBlockAxis> {
        let (offset_coefficient, stride) = self.axis[index as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(index, offset_coefficient, stride);
        BoneBlockAxis::from_u16(x)
    }
    pub fn bites(&self, index: u16) -> Option<i32> {
        let (offset_coefficient, stride) = self.bites[index as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(index, offset_coefficient, stride);
        i32::from_u16(x)
    }
    pub fn conditional(&self, index: u16) -> Option<bool> {
        let (offset_coefficient, stride) = self.conditional[index as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(index, offset_coefficient, stride);
        match x {
            1 => Some(true),
            0 => Some(false),
            _ => None,
        }
    }
    pub fn delay(&self, index: u16) -> Option<i32> {
        let (offset_coefficient, stride) = self.delay[index as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(index, offset_coefficient, stride);
        i32::from_u16(x)
    }
    pub fn disarmed(&self, index: u16) -> Option<bool> {
        let (offset_coefficient, stride) = self.disarmed[index as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(index, offset_coefficient, stride);
        match x {
            1 => Some(true),
            0 => Some(false),
            _ => None,
        }
    }
    pub fn distance(&self, index: u16) -> Option<i32> {
        let (offset_coefficient, stride) = self.distance[index as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(index, offset_coefficient, stride);
        i32::from_u16(x)
    }
    pub fn down(&self, index: u16) -> Option<bool> {
        let (offset_coefficient, stride) = self.down[index as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(index, offset_coefficient, stride);
        match x {
            1 => Some(true),
            0 => Some(false),
            _ => None,
        }
    }
    pub fn drag(&self, index: u16) -> Option<bool> {
        let (offset_coefficient, stride) = self.drag[index as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(index, offset_coefficient, stride);
        match x {
            1 => Some(true),
            0 => Some(false),
            _ => None,
        }
    }
    pub fn east(&self, index: u16) -> Option<bool> {
        let (offset_coefficient, stride) = self.east[index as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(index, offset_coefficient, stride);
        match x {
            1 => Some(true),
            0 => Some(false),
            _ => None,
        }
    }
    pub fn eggs(&self, index: u16) -> Option<i32> {
        let (offset_coefficient, stride) = self.eggs[index as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(index, offset_coefficient, stride);
        i32::from_u16(x)
    }
    pub fn enabled(&self, index: u16) -> Option<bool> {
        let (offset_coefficient, stride) = self.enabled[index as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(index, offset_coefficient, stride);
        match x {
            1 => Some(true),
            0 => Some(false),
            _ => None,
        }
    }
    pub fn extended(&self, index: u16) -> Option<bool> {
        let (offset_coefficient, stride) = self.extended[index as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(index, offset_coefficient, stride);
        match x {
            1 => Some(true),
            0 => Some(false),
            _ => None,
        }
    }
    pub fn eye(&self, index: u16) -> Option<bool> {
        let (offset_coefficient, stride) = self.eye[index as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(index, offset_coefficient, stride);
        match x {
            1 => Some(true),
            0 => Some(false),
            _ => None,
        }
    }
    pub fn face(&self, index: u16) -> Option<DarkOakButtonFace> {
        let (offset_coefficient, stride) = self.face[index as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(index, offset_coefficient, stride);
        DarkOakButtonFace::from_u16(x)
    }
    pub fn facing(&self, index: u16) -> Option<HornCoralWallFanFacing> {
        let (offset_coefficient, stride) = self.facing[index as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(index, offset_coefficient, stride);
        HornCoralWallFanFacing::from_u16(x)
    }
    pub fn half(&self, index: u16) -> Option<PurpurStairsHalf> {
        let (offset_coefficient, stride) = self.half[index as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(index, offset_coefficient, stride);
        PurpurStairsHalf::from_u16(x)
    }
    pub fn has_bottle_0(&self, index: u16) -> Option<bool> {
        let (offset_coefficient, stride) = self.has_bottle_0[index as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(index, offset_coefficient, stride);
        match x {
            1 => Some(true),
            0 => Some(false),
            _ => None,
        }
    }
    pub fn has_bottle_1(&self, index: u16) -> Option<bool> {
        let (offset_coefficient, stride) = self.has_bottle_1[index as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(index, offset_coefficient, stride);
        match x {
            1 => Some(true),
            0 => Some(false),
            _ => None,
        }
    }
    pub fn has_bottle_2(&self, index: u16) -> Option<bool> {
        let (offset_coefficient, stride) = self.has_bottle_2[index as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(index, offset_coefficient, stride);
        match x {
            1 => Some(true),
            0 => Some(false),
            _ => None,
        }
    }
    pub fn has_record(&self, index: u16) -> Option<bool> {
        let (offset_coefficient, stride) = self.has_record[index as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(index, offset_coefficient, stride);
        match x {
            1 => Some(true),
            0 => Some(false),
            _ => None,
        }
    }
    pub fn hatch(&self, index: u16) -> Option<i32> {
        let (offset_coefficient, stride) = self.hatch[index as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(index, offset_coefficient, stride);
        i32::from_u16(x)
    }
    pub fn hinge(&self, index: u16) -> Option<DarkOakDoorHinge> {
        let (offset_coefficient, stride) = self.hinge[index as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(index, offset_coefficient, stride);
        DarkOakDoorHinge::from_u16(x)
    }
    pub fn in_wall(&self, index: u16) -> Option<bool> {
        let (offset_coefficient, stride) = self.in_wall[index as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(index, offset_coefficient, stride);
        match x {
            1 => Some(true),
            0 => Some(false),
            _ => None,
        }
    }
    pub fn instrument(&self, index: u16) -> Option<NoteBlockInstrument> {
        let (offset_coefficient, stride) = self.instrument[index as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(index, offset_coefficient, stride);
        NoteBlockInstrument::from_u16(x)
    }
    pub fn inverted(&self, index: u16) -> Option<bool> {
        let (offset_coefficient, stride) = self.inverted[index as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(index, offset_coefficient, stride);
        match x {
            1 => Some(true),
            0 => Some(false),
            _ => None,
        }
    }
    pub fn kind(&self, index: u16) -> Option<PurpurSlabKind> {
        let (offset_coefficient, stride) = self.kind[index as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(index, offset_coefficient, stride);
        PurpurSlabKind::from_u16(x)
    }
    pub fn layers(&self, index: u16) -> Option<i32> {
        let (offset_coefficient, stride) = self.layers[index as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(index, offset_coefficient, stride);
        i32::from_u16(x)
    }
    pub fn level(&self, index: u16) -> Option<i32> {
        let (offset_coefficient, stride) = self.level[index as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(index, offset_coefficient, stride);
        i32::from_u16(x)
    }
    pub fn lit(&self, index: u16) -> Option<bool> {
        let (offset_coefficient, stride) = self.lit[index as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(index, offset_coefficient, stride);
        match x {
            1 => Some(true),
            0 => Some(false),
            _ => None,
        }
    }
    pub fn locked(&self, index: u16) -> Option<bool> {
        let (offset_coefficient, stride) = self.locked[index as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(index, offset_coefficient, stride);
        match x {
            1 => Some(true),
            0 => Some(false),
            _ => None,
        }
    }
    pub fn mode(&self, index: u16) -> Option<StructureBlockMode> {
        let (offset_coefficient, stride) = self.mode[index as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(index, offset_coefficient, stride);
        StructureBlockMode::from_u16(x)
    }
    pub fn moisture(&self, index: u16) -> Option<i32> {
        let (offset_coefficient, stride) = self.moisture[index as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(index, offset_coefficient, stride);
        i32::from_u16(x)
    }
    pub fn north(&self, index: u16) -> Option<bool> {
        let (offset_coefficient, stride) = self.north[index as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(index, offset_coefficient, stride);
        match x {
            1 => Some(true),
            0 => Some(false),
            _ => None,
        }
    }
    pub fn note(&self, index: u16) -> Option<i32> {
        let (offset_coefficient, stride) = self.note[index as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(index, offset_coefficient, stride);
        i32::from_u16(x)
    }
    pub fn occupied(&self, index: u16) -> Option<bool> {
        let (offset_coefficient, stride) = self.occupied[index as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(index, offset_coefficient, stride);
        match x {
            1 => Some(true),
            0 => Some(false),
            _ => None,
        }
    }
    pub fn open(&self, index: u16) -> Option<bool> {
        let (offset_coefficient, stride) = self.open[index as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(index, offset_coefficient, stride);
        match x {
            1 => Some(true),
            0 => Some(false),
            _ => None,
        }
    }
    pub fn part(&self, index: u16) -> Option<BlackBedPart> {
        let (offset_coefficient, stride) = self.part[index as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(index, offset_coefficient, stride);
        BlackBedPart::from_u16(x)
    }
    pub fn persistent(&self, index: u16) -> Option<bool> {
        let (offset_coefficient, stride) = self.persistent[index as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(index, offset_coefficient, stride);
        match x {
            1 => Some(true),
            0 => Some(false),
            _ => None,
        }
    }
    pub fn pickles(&self, index: u16) -> Option<i32> {
        let (offset_coefficient, stride) = self.pickles[index as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(index, offset_coefficient, stride);
        i32::from_u16(x)
    }
    pub fn power(&self, index: u16) -> Option<i32> {
        let (offset_coefficient, stride) = self.power[index as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(index, offset_coefficient, stride);
        i32::from_u16(x)
    }
    pub fn powered(&self, index: u16) -> Option<bool> {
        let (offset_coefficient, stride) = self.powered[index as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(index, offset_coefficient, stride);
        match x {
            1 => Some(true),
            0 => Some(false),
            _ => None,
        }
    }
    pub fn rotation(&self, index: u16) -> Option<i32> {
        let (offset_coefficient, stride) = self.rotation[index as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(index, offset_coefficient, stride);
        i32::from_u16(x)
    }
    pub fn shape(&self, index: u16) -> Option<PurpurStairsShape> {
        let (offset_coefficient, stride) = self.shape[index as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(index, offset_coefficient, stride);
        PurpurStairsShape::from_u16(x)
    }
    pub fn short(&self, index: u16) -> Option<bool> {
        let (offset_coefficient, stride) = self.short[index as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(index, offset_coefficient, stride);
        match x {
            1 => Some(true),
            0 => Some(false),
            _ => None,
        }
    }
    pub fn snowy(&self, index: u16) -> Option<bool> {
        let (offset_coefficient, stride) = self.snowy[index as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(index, offset_coefficient, stride);
        match x {
            1 => Some(true),
            0 => Some(false),
            _ => None,
        }
    }
    pub fn south(&self, index: u16) -> Option<bool> {
        let (offset_coefficient, stride) = self.south[index as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(index, offset_coefficient, stride);
        match x {
            1 => Some(true),
            0 => Some(false),
            _ => None,
        }
    }
    pub fn stage(&self, index: u16) -> Option<i32> {
        let (offset_coefficient, stride) = self.stage[index as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(index, offset_coefficient, stride);
        i32::from_u16(x)
    }
    pub fn triggered(&self, index: u16) -> Option<bool> {
        let (offset_coefficient, stride) = self.triggered[index as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(index, offset_coefficient, stride);
        match x {
            1 => Some(true),
            0 => Some(false),
            _ => None,
        }
    }
    pub fn unstable(&self, index: u16) -> Option<bool> {
        let (offset_coefficient, stride) = self.unstable[index as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(index, offset_coefficient, stride);
        match x {
            1 => Some(true),
            0 => Some(false),
            _ => None,
        }
    }
    pub fn up(&self, index: u16) -> Option<bool> {
        let (offset_coefficient, stride) = self.up[index as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(index, offset_coefficient, stride);
        match x {
            1 => Some(true),
            0 => Some(false),
            _ => None,
        }
    }
    pub fn waterlogged(&self, index: u16) -> Option<bool> {
        let (offset_coefficient, stride) = self.waterlogged[index as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(index, offset_coefficient, stride);
        match x {
            1 => Some(true),
            0 => Some(false),
            _ => None,
        }
    }
    pub fn west(&self, index: u16) -> Option<bool> {
        let (offset_coefficient, stride) = self.west[index as usize];
        if offset_coefficient == 0 {
            return None;
        }
        let x = crate::n_dimensional_index(index, offset_coefficient, stride);
        match x {
            1 => Some(true),
            0 => Some(false),
            _ => None,
        }
    }
}
