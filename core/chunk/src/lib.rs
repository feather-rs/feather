use ahash::AHashMap;
use bitflags::bitflags;
use feather_biomes::Biome;
use feather_blocks::BlockId;
use feather_util::ChunkPosition;
use smallvec::SmallVec;

/// The number of bits used for each block
/// in the global palette.
pub const GLOBAL_BITS_PER_BLOCK: u8 = 14;

/// The minimum bits per block allowed when
/// using a section palette.
/// Bits per block values lower than this
/// value will be offsetted to this value.
pub const MIN_BITS_PER_BLOCK: u8 = 4;

/// The maximum number of bits per block
/// allowed when using a section palette.
/// Values above this will use the global palette
/// instead.
pub const MAX_BITS_PER_BLOCK: u8 = 8;

/// The height in blocks of a chunk column.
pub const CHUNK_HEIGHT: usize = 256;
/// The width in blocks of a chunk column.
pub const CHUNK_WIDTH: usize = 16;

/// The height in blocks of a chunk section.
pub const SECTION_HEIGHT: usize = 16;

/// The width in blocks of a chunk section.
pub const SECTION_WIDTH: usize = CHUNK_WIDTH;

/// The volume in blocks of a chunk section.
pub const SECTION_VOLUME: usize = (SECTION_HEIGHT * SECTION_WIDTH * SECTION_WIDTH) as usize;

/// The number of chunk sections in a column.
pub const NUM_SECTIONS: usize = 16;

/// A chunk column consisting
/// of a 16x256x16 section of blocks.
/// A chunk column maintains an array
/// of up to 16 chunk sections, each corresponding
/// to a 16x16x16 section of blocks in the chunk.
#[derive(Clone)]
pub struct Chunk {
    /// The location of this chunk, in chunk
    /// coordinates.
    location: ChunkPosition,
    /// An array of the sections in this chunk.
    /// A section with Y value `y` can be found at
    /// index `y` in this array.
    /// When an entry in this array is set to `None`,
    /// the section at the entry's Y coordinate
    /// is assumed to empty, meaning that it consists
    /// of only air.
    sections: [Option<ChunkSection>; NUM_SECTIONS],
    /// The biomes in this section, indexable by
    /// ((z << 4) | x).
    biomes: [Biome; SECTION_WIDTH * SECTION_WIDTH],
    /// Whether this chunk has been modified since the most recent
    /// call to `check_modified`().
    modified: bool,

    heightmaps: Box<[HeightMap]>,
}

#[derive(Clone, Copy, Default)]
pub struct HeightMap {
    light_blocking: u16,
    motion_blocking: u16,
    motion_blocking_no_leaves: u16,
    ocean_floor: u16,
    world_surface: u16,
}

impl HeightMap {
    /// Y-coordinate + 1 of the highest opaque block.
    pub fn light_blocking(self) -> u16 {
        self.light_blocking
    }

    pub fn set_light_blocking(&mut self, mut light_blocking: u16) {
        if light_blocking > 256 {
            light_blocking = 256;
        }
        self.light_blocking = light_blocking;
    }

    /// Y-coordinate + 1 of the highest solid or fluid block.
    pub fn motion_blocking(self) -> u16 {
        self.motion_blocking
    }

    pub fn set_motion_blocking(&mut self, mut motion_blocking: u16) {
        if motion_blocking > 256 {
            motion_blocking = 256;
        }
        self.motion_blocking = motion_blocking;
    }

    /// Y-coordinate + 1 of the highest solid or fluid block that is not leaves.
    pub fn motion_blocking_no_leaves(self) -> u16 {
        self.motion_blocking_no_leaves
    }

    pub fn set_motion_blocking_no_leaves(&mut self, mut motion_blocking_no_leaves: u16) {
        if motion_blocking_no_leaves > 256 {
            motion_blocking_no_leaves = 256;
        }
        self.motion_blocking_no_leaves = motion_blocking_no_leaves;
    }

    /// Y-coordinate + 1 of the highest solid block.
    pub fn ocean_floor(self) -> u16 {
        self.ocean_floor
    }

    pub fn set_ocean_floor(&mut self, mut ocean_floor: u16) {
        if ocean_floor > 256 {
            ocean_floor = 256;
        }
        self.ocean_floor = ocean_floor;
    }

    /// Y-coordinate + 1 of the highest non-air block.
    pub fn world_surface(self) -> u16 {
        self.world_surface
    }

    pub fn set_world_surface(&mut self, mut world_surface: u16) {
        if world_surface > 256 {
            world_surface = 256;
        }
        self.world_surface = world_surface;
    }
}

bitflags! {
    struct HeightMapMask: u8 {
        const LIGHT_BLOCKING = 0b0000_0001;
        const MOTION_BLOCKING = 0b0000_0010;
        const MOTION_BLOCKING_NO_LEAVES = 0b0000_0100;
        const OCEAN_FLOOR = 0b0000_1000;
        const WORLD_SURFACE = 0b0001_0000;
    }
}

impl Default for Chunk {
    fn default() -> Self {
        // Rust apparently forces you to implement
        // `Copy` on types if you want to use the
        // `[ChunkSection::new(); 16]` syntax,
        // so I had to do this.
        let sections = [
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None,
        ];

        Self {
            location: ChunkPosition::new(0, 0),
            modified: true,
            sections,
            biomes: [Biome::Plains; SECTION_WIDTH * SECTION_WIDTH],
            heightmaps: vec![HeightMap::default(); CHUNK_WIDTH * CHUNK_WIDTH].into_boxed_slice(),
        }
    }
}

impl Chunk {
    /// Creates a new empty chunk
    /// with the specified location.
    pub fn new(location: ChunkPosition) -> Self {
        Self {
            location,
            modified: true,
            ..Default::default()
        }
    }

    /// Creates a new empty chunk
    /// with the specified location,
    /// and filling its biomes with
    /// the provided `default_biome`.
    pub fn new_with_default_biome(location: ChunkPosition, default_biome: Biome) -> Self {
        Self {
            location,
            modified: true,
            biomes: [default_biome; SECTION_WIDTH * SECTION_HEIGHT],
            ..Default::default()
        }
    }

    /// Gets the block at the specified
    /// position in this chunk. The position
    /// is in the chunk's local coordinate
    /// space.
    ///
    /// The specified coordinates must be inside
    /// this chunk, so the function will panic
    /// if `x >= 16 || y >= 256 || z >= 16`.
    pub fn block_at(&self, x: usize, y: usize, z: usize) -> BlockId {
        Self::check_coords(x, y, z);
        let chunk_section = &self.sections[(y / 16) as usize];
        match chunk_section {
            Some(section) => section.block_at(x, y % 16, z),
            None => BlockId::air(),
        }
    }

    /// Sets the block at the specified
    /// position in this chunk. The position
    /// is in the chunk's local coordinate
    /// space.
    ///
    /// The specified coordinates must be inside
    /// this chunk, so the function will panic
    /// if `x >= 16 || y >= 256 || z >= 16`.
    pub fn set_block_at(&mut self, x: usize, y: usize, z: usize, block: BlockId) {
        Self::check_coords(x, y, z);
        self.modified = true;

        let chunk_section = &mut self.sections[y / 16];

        let section;
        if let Some(sec) = chunk_section {
            section = sec;
        } else {
            // The section is empty - create it
            if block == BlockId::air() {
                return; // Nothing to do - section already empty
            }

            let new_section = ChunkSection::default();
            self.set_section_at(y / 16, Some(new_section));
            section = self.section_mut(y / 16).unwrap();
        }

        let old_block = section.block_at(x, y % 16, z);
        section.set_block_at(x, y % 16, z, block);

        self.update_heightmap(x, y, z, old_block, block);
    }

    pub fn heightmap(&self, x: usize, z: usize) -> &HeightMap {
        Self::check_coords(x, 0, z);
        &self.heightmaps[x + z * CHUNK_WIDTH]
    }

    pub fn heightmap_mut(&mut self, x: usize, z: usize) -> &mut HeightMap {
        Self::check_coords(x, 0, z);
        &mut self.heightmaps[x + z * CHUNK_WIDTH]
    }

    pub fn heightmaps(&self) -> &[HeightMap] {
        &self.heightmaps
    }

    fn update_heightmap(
        &mut self,
        x: usize,
        y: usize,
        z: usize,
        old_block: BlockId,
        new_block: BlockId,
    ) -> HeightMapMask {
        let mut mask: HeightMapMask = HeightMapMask::empty();
        let y = y as u16;

        struct HeightMapCheckContext(
            &'static dyn Fn(BlockId) -> bool,
            HeightMapMask,
            &'static dyn Fn(&HeightMap) -> u16,
            &'static dyn Fn(&mut HeightMap, u16),
        );

        let checks: [HeightMapCheckContext; 5] = [
            // Light blocking
            HeightMapCheckContext(
                &|block| block.is_opaque(),
                HeightMapMask::LIGHT_BLOCKING,
                &|map| map.light_blocking(),
                &|map, value| map.set_light_blocking(value),
            ),
            // Motion blocking
            HeightMapCheckContext(
                &|block| block.is_solid() || block.is_fluid(),
                HeightMapMask::MOTION_BLOCKING,
                &|map| map.motion_blocking(),
                &|map, value| map.set_motion_blocking(value),
            ),
            // Motion blocking no leaves
            HeightMapCheckContext(
                &|block| (block.is_solid() || block.is_fluid()) && !block.is_leaves(),
                HeightMapMask::MOTION_BLOCKING_NO_LEAVES,
                &|map| map.motion_blocking_no_leaves(),
                &|map, value| map.set_motion_blocking_no_leaves(value),
            ),
            // Ocean floor
            HeightMapCheckContext(
                &|block| block.is_solid(),
                HeightMapMask::OCEAN_FLOOR,
                &|map| map.ocean_floor(),
                &|map, value| map.set_ocean_floor(value),
            ),
            // World surface
            HeightMapCheckContext(
                &|block| !block.is_air(),
                HeightMapMask::WORLD_SURFACE,
                &|map| map.world_surface(),
                &|map, value| map.set_world_surface(value),
            ),
        ];

        for HeightMapCheckContext(valid_block, check_mask, map_getter, map_setter) in checks.iter()
        {
            // Check heightmap type
            if valid_block(old_block) && map_getter(self.heightmap_mut(x, z)) == y {
                // This was the highest block
                self.heightmap_mut(x, z).set_light_blocking(0);

                for i in (0..y).rev() {
                    let block = self.block_at(x, i as usize, z);

                    if valid_block(block) {
                        map_setter(self.heightmap_mut(x, z), i + 1);
                        break;
                    }
                }
                mask |= *check_mask;
            }
            if valid_block(new_block) && map_getter(self.heightmap_mut(x, z)) < y {
                // This is the new highest block
                map_setter(self.heightmap_mut(x, z), y);
                mask |= *check_mask;
            }
        }

        mask
    }

    /// Recalculate the heightmap for the chunk
    pub fn recalculate_heightmap(&mut self) {
        // This function can be optimized, instead of
        // fetching heightmap every time, and sections
        for x in 0..CHUNK_WIDTH {
            for z in 0..CHUNK_WIDTH {
                let mut mask: HeightMapMask = HeightMapMask::empty();
                for y in (0..CHUNK_HEIGHT).rev() {
                    if mask.is_all() {
                        break;
                    }
                    let block = self.block_at(x, y, z);
                    mask |= self.update_heightmap(x, y, z, BlockId::air(), block);
                }
            }
        }
    }

    pub fn sky_light_at(&self, x: usize, y: usize, z: usize) -> u8 {
        Self::check_coords(x, y, z);
        let chunk_section = self.section_for_y(y);
        match chunk_section {
            Some(chunk_section) => chunk_section.sky_light_at(x, y % 16, z),
            None => 0,
        }
    }

    pub fn block_light_at(&self, x: usize, y: usize, z: usize) -> u8 {
        Self::check_coords(x, y, z);
        let chunk_section = self.section_for_y(y);
        match chunk_section {
            Some(chunk_section) => chunk_section.block_light_at(x, y % 16, z),
            None => 0,
        }
    }

    pub fn set_sky_light_at(&mut self, x: usize, y: usize, z: usize, value: u8) {
        Self::check_coords(x, y, z);
        let chunk_section = self.section_for_y_mut(y);
        chunk_section.set_sky_light_at(x, y % 16, z, value);
    }

    pub fn set_block_light_at(&mut self, x: usize, y: usize, z: usize, value: u8) {
        Self::check_coords(x, y, z);
        let chunk_section = self.section_for_y_mut(y);
        chunk_section.set_block_light_at(x, y % 16, z, value);
    }

    fn section_for_y(&self, y: usize) -> &Option<ChunkSection> {
        &self.sections[y / 16]
    }

    fn section_for_y_mut(&mut self, y: usize) -> &mut ChunkSection {
        self.sections[y / 16].get_or_insert_with(ChunkSection::default)
    }

    fn check_coords(x: usize, y: usize, z: usize) {
        assert!(x < CHUNK_WIDTH);
        assert!(y < CHUNK_HEIGHT);
        assert!(z < CHUNK_WIDTH);
    }

    /// Returns a slice of the 16
    /// chunk sections in the chunk.
    pub fn sections(&self) -> Vec<Option<&ChunkSection>> {
        self.sections.iter().map(|sec| sec.as_ref()).collect()
    }

    /// Returns a mutable slice of the 16 sections
    /// in this chunk.
    pub fn sections_mut(&mut self) -> Vec<Option<&mut ChunkSection>> {
        self.modified = true;
        self.sections.iter_mut().map(|sec| sec.as_mut()).collect()
    }

    /// Returns the position in chunk coordinates
    /// of this chunk.
    pub fn position(&self) -> ChunkPosition {
        self.location
    }

    /// Sets the position of this chunk.
    pub fn set_position(&mut self, pos: ChunkPosition) {
        self.location = pos
    }

    /// Returns a reference to the chunk section at the given
    /// Y offset. The Y offset must be between 0 and 15, inclusive;
    /// each Y offset value corresponds to 16 blocks vertically.
    ///
    /// If this function returns `None`, the section is assumed
    /// to be empty, meaning it consists only of air.
    pub fn section(&self, index: usize) -> Option<&ChunkSection> {
        assert!(index < NUM_SECTIONS);
        self.sections[index].as_ref()
    }

    /// Returns a mutable reference to the chunk section at the given
    /// Y offset. The Y offset must be between 0 and 15, inclusive;
    /// each Y offset value corresponds to 16 blocks vertically.
    ///
    /// If this function returns `None`, the section is assumed
    /// to be empty, meaning it consists only of air.
    pub fn section_mut(&mut self, index: usize) -> Option<&mut ChunkSection> {
        assert!(index < NUM_SECTIONS);
        self.modified = true;
        self.sections[index].as_mut()
    }

    /// Sets the section at the given section index.
    pub fn set_section_at(&mut self, index: usize, section: Option<ChunkSection>) {
        assert!(index < NUM_SECTIONS);
        self.sections[index] = section;
        self.modified = true;
    }

    /// Optimizes each section in this chunk.
    ///
    /// Returns the number of sections which were actually
    /// optimized - sections which have not been
    /// modified since the last time they were optimized
    /// are not optimized.
    pub fn optimize(&mut self) -> u32 {
        let modified = self.modified;
        let mut count = 0;
        let mut to_remove = vec![];
        for (i, s) in self.sections.iter_mut().enumerate() {
            if let Some(section) = s {
                if section.optimize() {
                    // Section was optimized - increment count
                    count += 1;
                }

                if section.empty() {
                    to_remove.push(i);
                }
            }
        }

        for i in to_remove {
            self.set_section_at(i, None);
        }

        self.modified = modified;

        count
    }

    /// Returns the biomes of this chunk.
    pub fn biomes(&self) -> &[Biome] {
        &self.biomes
    }

    /// Returns a mutable reference to the biomes of this chunk.
    pub fn biomes_mut(&mut self) -> &mut [Biome] {
        self.modified = true;
        &mut self.biomes
    }

    /// Gets the biome for the specified column.
    ///
    /// # Panics
    /// Panics if `x < 16` or `z < 16`.
    pub fn biome_at(&self, x: usize, z: usize) -> Biome {
        let index = Self::biome_index(x, z);
        self.biomes[index]
    }

    /// Sets the biome for the specified column.
    ///
    /// # Panics
    /// Panics if `x < 16` or `z < 16`.
    pub fn set_biome_at(&mut self, x: usize, z: usize, biome: Biome) {
        let index = Self::biome_index(x, z);
        self.modified = true;
        self.biomes[index] = biome;
    }

    /// Checks whether this chunk has been modified since the last
    /// call to this function.
    pub fn check_modified(&mut self) -> bool {
        let res = self.modified;
        self.modified = false;
        res
    }

    fn biome_index(x: usize, z: usize) -> usize {
        assert!(x < 16);
        assert!(z < 16);

        (z << 4) | x
    }
}

/// A chunk section consisting of a 16x16x16
/// cube of blocks.
#[derive(Clone, Debug)]
pub struct ChunkSection {
    /// The block state data for this chunk section.
    data: BitArray,
    /// This section's palette. `None` if using the global palette.
    /// The palette should always remain sorted so that a binary
    /// search can be performed on it.
    palette: Option<Vec<BlockId>>,
    /// The number of solid blocks in this chunk, i.e. those
    /// that are not air. This value is used to figure out when
    /// the section becomes empty.
    solid_block_count: u16,

    block_light: BitArray,
    sky_light: BitArray,

    /// A section is considered dirty when it has been
    /// modified since the last time it was optimized.
    dirty: bool,
}

impl ChunkSection {
    /// Creates a new, empty `ChunkSection`.
    pub fn new(
        mut data: BitArray,
        mut palette: Option<Vec<BlockId>>,
        block_light: BitArray,
        sky_light: BitArray,
    ) -> Self {
        // Correct palette if not using the global palette
        if let Some(palette) = palette.as_mut() {
            Self::correct_data_and_palette(&mut data, palette);
        }

        // Count solid blocks
        let mut solid_block_count = 0;
        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    if data.get(block_index(x, y, z)) != 0 {
                        solid_block_count += 1;
                    }
                }
            }
        }

        Self {
            data,
            palette,
            solid_block_count,
            dirty: false,
            block_light,
            sky_light,
        }
    }

    /// Corrects a given raw palette and data array.
    ///
    /// Since chunk data stored by external sources
    /// (e.g. Vanilla) might not require a sorted palette
    /// like Feather does, we need to sort the palette and
    /// correct data in the array when reading from external
    /// sources.
    ///
    /// The correction is done in-place.
    fn correct_data_and_palette(data: &mut BitArray, palette: &mut Vec<BlockId>) {
        let original_palette = palette.clone(); // Palette without sorting guarantees

        palette.sort_unstable();

        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    // Replace index into palette of each block with
                    // new index into the sorted palette.
                    let block_index = block_index(x, y, z);
                    let old_index = data.get(block_index);
                    let new_index = palette
                        .binary_search(&original_palette[old_index as usize])
                        .unwrap();
                    data.set(block_index, new_index as u64);
                }
            }
        }
    }

    /// Returns whether this chunk section is empty.
    pub fn empty(&self) -> bool {
        self.solid_block_count == 0
    }

    /// Retrieves the block at the given position in this chunk section.
    /// The position is local to this section.
    pub fn block_at(&self, x: usize, y: usize, z: usize) -> BlockId {
        let index = block_index(x, y, z);
        let block_id = self.data.get(index);

        match &self.palette {
            Some(palette) => palette[block_id as usize],
            None => BlockId::from_vanilla_id(block_id as u16),
        }
    }

    /// Sets the block at the given position in this chunk section.
    /// The position is local to this section.
    pub fn set_block_at(&mut self, x: usize, y: usize, z: usize, block: BlockId) {
        self.dirty = true;

        let index = block_index(x, y, z);

        // The value that will be put into the data array.
        let mut paletted_index;
        if let Some(palette) = self.palette.as_mut() {
            // Retrieve the block index from the palette.

            // If necessary, add the block to the palette.
            match palette.binary_search(&block) {
                Ok(index) => paletted_index = index,
                Err(insertion_index) => {
                    palette.insert(insertion_index, block);
                    paletted_index = insertion_index;

                    // Resize if necessary
                    if needed_bits((palette.len() - 1) as u64) > self.data.bits_per_value {
                        let new_bits_per_value = self.data.bits_per_value + 1;
                        if new_bits_per_value <= MAX_BITS_PER_BLOCK {
                            self.data = self.data.resize_to(self.data.bits_per_value + 1).unwrap();
                            paletted_index = insertion_index;
                        } else {
                            // Switch to the global palette
                            let mut new_data = BitArray::new(GLOBAL_BITS_PER_BLOCK, SECTION_VOLUME);
                            for _x in 0..16 {
                                for _y in 0..16 {
                                    for _z in 0..16 {
                                        let block = self.block_at(_x, _y, _z);
                                        new_data.set(
                                            block_index(_x, _y, _z),
                                            block.vanilla_id() as u64,
                                        );
                                    }
                                }
                            }

                            self.palette = None;
                            paletted_index = block.vanilla_id() as usize;
                            self.data = new_data;
                        }
                    }

                    // Correct data, since palette entries after
                    // the one which was inserted will be offsetted
                    // by one.
                    for x in 0..16 {
                        for y in 0..16 {
                            for z in 0..16 {
                                let index = block_index(x, y, z);

                                let entry = self.data.get(index);
                                if entry >= insertion_index as u64 {
                                    self.data.set(index, entry + 1);
                                }
                            }
                        }
                    }
                }
            }
        } else {
            // Use the global palette.
            paletted_index = block.vanilla_id() as usize;
        }

        let old_block = self.block_at(x, y, z);
        if block.is_air() && !old_block.is_air() {
            self.solid_block_count -= 1;
        } else if !block.is_air() && old_block.is_air() {
            self.solid_block_count += 1;
        }

        self.data.set(index, paletted_index as u64);
        debug_assert_eq!(self.block_at(x, y, z), block);
    }

    /// Optimizes this chunk section, reducing the bits
    /// per block value as much as possible and removing unused
    /// entries from the palette.
    ///
    /// This function only optimizes the chunk if it is dirt,
    /// i.e. if it has been modified since the last time
    /// it was optimized. The returned value is `true` when
    /// the chunk was optimized and `false` when it wasn't.
    pub fn optimize(&mut self) -> bool {
        // Only optimize the chunk if it has been modified.
        if !self.dirty {
            return false;
        }

        self.dirty = false;

        // Replace palette with new one.
        let mut new_palette = vec![];
        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    let block = self.block_at(x, y, z);
                    match new_palette.binary_search(&block) {
                        Ok(_) => (),
                        Err(insert_index) => {
                            new_palette.insert(insert_index, block);
                        }
                    }
                }
            }
        }

        // Recalculate all block IDs to match with the new palette.
        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    let block = self.block_at(x, y, z);
                    self.data.set(
                        block_index(x, y, z),
                        new_palette.binary_search(&block).unwrap() as u64,
                    );
                }
            }
        }

        self.palette = Some(new_palette);

        // Recalculate bits per block value.
        let mut new_bits_per_block = needed_bits(self.palette.as_ref().unwrap().len() as u64);
        if new_bits_per_block > MAX_BITS_PER_BLOCK {
            self.palette = None;
        } else {
            if new_bits_per_block < MIN_BITS_PER_BLOCK {
                new_bits_per_block = MIN_BITS_PER_BLOCK;
            }
            self.data = self.data.resize_to(new_bits_per_block).unwrap();
        }

        true // Chunk was optimized
    }

    /// If the global palette is in use, convert it to a section palette.
    /// This is used for chunk saving.
    pub fn convert_palette_to_section(&mut self) {
        if self.palette.is_some() {
            // Nothing to do: section palette already in use.
            return;
        }

        type TuplePos = (usize, usize, usize);
        let mut blocks: AHashMap<BlockId, SmallVec<[TuplePos; 4]>> = AHashMap::with_capacity(512);
        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    blocks
                        .entry(self.block_at(x, y, z))
                        .or_default()
                        .push((x, y, z));
                }
            }
        }

        // Create a palette based on the blocks in the chunk.
        // We also have to modify the data array based on the new palette.
        let mut palette = Vec::with_capacity(512);
        for (block, positions) in blocks.into_iter() {
            palette.push(block);

            for (x, y, z) in positions {
                let index = block_index(x, y, z);
                self.data.set(index, (palette.len() - 1) as u64);
            }
        }

        self.palette = Some(palette);
    }

    /// Returns the internal data array for this section.
    pub fn data(&self) -> &BitArray {
        &self.data
    }

    /// Returns the palette for this section.
    pub fn palette(&self) -> Option<&[BlockId]> {
        self.palette.as_deref()
    }

    /// Returns the number of bits used to store each block.
    pub fn bits_per_block(&self) -> u8 {
        self.data.bits_per_value
    }

    pub fn sky_light(&self) -> &BitArray {
        &self.sky_light
    }

    pub fn block_light(&self) -> &BitArray {
        &self.block_light
    }

    pub fn sky_light_mut(&mut self) -> &mut BitArray {
        &mut self.sky_light
    }

    pub fn block_light_mut(&mut self) -> &mut BitArray {
        &mut self.block_light
    }

    pub fn sky_light_at(&self, x: usize, y: usize, z: usize) -> u8 {
        let index = block_index(x, y, z);
        self.sky_light.get(index) as u8
    }

    pub fn block_light_at(&self, x: usize, y: usize, z: usize) -> u8 {
        let index = block_index(x, y, z);
        self.block_light.get(index) as u8
    }

    pub fn set_sky_light_at(&mut self, x: usize, y: usize, z: usize, value: u8) {
        assert!(value < 16, "light level cannot exceed 15");
        let index = block_index(x, y, z);
        self.sky_light.set(index, u64::from(value));
    }

    pub fn set_block_light_at(&mut self, x: usize, y: usize, z: usize, value: u8) {
        assert!(value < 16, "light level cannot exceed 15");
        let index = block_index(x, y, z);
        self.block_light.set(index, u64::from(value));
    }
}

impl Default for ChunkSection {
    fn default() -> Self {
        let air = BlockId::air();
        Self {
            data: BitArray::new(4, SECTION_VOLUME),
            palette: Some(vec![air]),
            solid_block_count: 0,
            dirty: false,
            block_light: BitArray::new(4, SECTION_VOLUME),
            sky_light: BitArray::new(4, SECTION_VOLUME),
        }
    }
}

/// Returns the index into a block state array
/// for the given block position.
fn block_index(x: usize, y: usize, z: usize) -> usize {
    assert!(x < 16);
    assert!(y < 16);
    assert!(z < 16);
    (y << 8) | (z << 4) | x
}

/// A "bit array." This struct manages
/// an internal array of `u64` to which
/// values of arbitrary bit length can be written.
#[derive(Clone, Debug)]
pub struct BitArray {
    /// The internal data array containing all values
    data: Vec<u64>,
    /// The capacity, in values, of this array
    capacity: usize,
    /// The number of bits used to represent each value
    bits_per_value: u8,
    /// The maximum value represented by an entry in this array
    value_mask: u64,
}

impl BitArray {
    /// Creates a new `BitArray` with the given
    /// bits per value and capacity. The array
    /// will be initialized with zeroes.
    pub fn new(bits_per_value: u8, capacity: usize) -> Self {
        assert!(
            bits_per_value <= 64,
            "Bits per value cannot be more than 64"
        );
        assert!(bits_per_value > 0, "Bits per value must be positive");
        let data = {
            let len = (((capacity * (bits_per_value as usize)) as f64) / 64.0).ceil() as usize;
            vec![0u64; len]
        };

        let value_mask = (1 << (bits_per_value as u64)) - 1;

        Self {
            data,
            capacity,
            bits_per_value,
            value_mask,
        }
    }

    /// Creates a new `BitArray` based on the given raw parts.
    pub fn from_raw(data: Vec<u64>, bits_per_value: u8, capacity: usize) -> Self {
        assert!(
            bits_per_value <= 64,
            "Bits per value cannot be more than 64"
        );
        assert!(bits_per_value > 0, "Bits per value must be positive");

        let value_mask = (1 << (bits_per_value as u64)) - 1;

        Self {
            data,
            capacity,
            bits_per_value,
            value_mask,
        }
    }

    /// Returns the highest possible value represented
    /// by and entry in this `BitArray`.
    pub fn highest_possible_value(&self) -> u64 {
        self.value_mask
    }

    /// Returns the value at the given location in this `BitArray`.
    pub fn get(&self, index: usize) -> u64 {
        assert!(index < self.capacity, "Index out of bounds");

        let bit_index = index * (self.bits_per_value as usize);

        let start_long_index = bit_index / 64;

        let start_long = self.data[start_long_index];

        let index_in_start_long = (bit_index % 64) as u64;

        let mut result = start_long >> index_in_start_long;

        let end_bit_offset = index_in_start_long + self.bits_per_value as u64;

        if end_bit_offset > 64 {
            // Value stretches across multiple longs
            let end_long = self.data[start_long_index + 1];
            result |= end_long << (64 - index_in_start_long);
        }

        result & self.value_mask
    }

    /// Sets the value at the given index into this `BitArray`
    pub fn set(&mut self, index: usize, val: u64) {
        assert!(index < self.capacity, "Index out of bounds");
        assert!(
            val <= self.value_mask,
            "Value does not fit into bits_per_value"
        );

        let bit_index = index * (self.bits_per_value as usize);

        let start_long_index = bit_index / 64;

        let index_in_start_long = (bit_index % 64) as u64;

        // Clear bits of this value first
        self.data[start_long_index] = (self.data[start_long_index]
            & !(self.value_mask << index_in_start_long))
            | ((val & self.value_mask) << index_in_start_long);

        let end_bit_offset = index_in_start_long + self.bits_per_value as u64;
        if end_bit_offset > 64 {
            // Value stretches across multiple longs
            self.data[start_long_index + 1] = (self.data[start_long_index + 1]
                & !((1 << (end_bit_offset - 64)) - 1))
                | val >> (64 - index_in_start_long);
        }

        debug_assert_eq!(self.get(index), val);
    }

    /// Produces a `BitArray` with the same values
    /// as this `BitArray` but with a new bits per value.
    /// If a value in this `BitArray` cannot be represented
    /// by the new bits per value, `Err` is returned.
    pub fn resize_to(&self, new_bits_per_value: u8) -> Result<BitArray, ()> {
        assert!(
            new_bits_per_value <= 64,
            "Bits per value cannot be more than 64"
        );

        let mut new_arr = BitArray::new(new_bits_per_value, self.capacity);

        for i in 0..self.capacity {
            let val = self.get(i);
            if needed_bits(val) > new_bits_per_value {
                return Err(());
            }

            new_arr.set(i, val);
            debug_assert_eq!(new_arr.get(i), val);
        }

        Ok(new_arr)
    }

    /// Returns the internal array.
    pub fn inner(&self) -> &Vec<u64> {
        &self.data
    }
}

/// Returns the number of bits
/// needed to represent the given value.
fn needed_bits(mut val: u64) -> u8 {
    let mut result = 0;
    loop {
        val >>= 1;
        result += 1;

        if val == 0 {
            break;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chunk_new() {
        let pos = ChunkPosition::new(0, 0);
        let chunk = Chunk::new(pos);

        // Confirm that chunk is empty
        for x in 0..16 {
            assert!(chunk.section(x).is_none());
            assert!(chunk.section(x).is_none());
        }

        assert_eq!(chunk.position(), pos);
    }

    #[test]
    fn chunk_new_with_default_biome() {
        let pos = ChunkPosition::new(0, 0);
        let chunk = Chunk::new_with_default_biome(pos, Biome::Mountains);

        // Confirm that chunk is empty
        for x in 0..16 {
            assert!(chunk.section(x).is_none());
            assert!(chunk.section(x).is_none());
        }

        assert_eq!(chunk.position(), pos);

        // Confirm that biomes are set
        for x in 0..16 {
            for z in 0..16 {
                assert_eq!(chunk.biome_at(x, z), Biome::Mountains);
            }
        }
    }

    #[test]
    fn set_block_simple() {
        let pos = ChunkPosition::new(0, 0);
        let mut chunk = Chunk::new(pos);

        chunk.set_block_at(0, 0, 0, BlockId::andesite());
        assert_eq!(chunk.block_at(0, 0, 0), BlockId::andesite());
        assert!(chunk.section(0).is_some());
    }

    #[test]
    fn fill_chunk() {
        let pos = ChunkPosition::new(0, 0);
        let mut chunk = Chunk::new(pos);

        let block = BlockId::stone();

        for x in 0..16 {
            for y in 0..256 {
                for z in 0..16 {
                    chunk.set_block_at(x, y, z, block);
                    assert_eq!(chunk.block_at(x, y, z), block);
                }
            }
        }

        // Check again, just to be sure
        for x in 0..16 {
            for y in 0..256 {
                for z in 0..16 {
                    assert_eq!(chunk.block_at(x, y, z), block);
                }
            }
        }
    }

    #[test]
    fn spray_chunk() {
        // This test fills each section of the chunk
        // with the blocks with IDs corresponding
        // to 0-4095 in order, testing that
        // resizing, etc. works correctly.

        let pos = ChunkPosition::new(0, 0);
        let mut chunk = Chunk::new(pos);

        for section in chunk.sections() {
            assert!(section.is_none());
        }

        for section in 0..16 {
            let mut counter = 0;
            for x in 0..16 {
                for y in 0..16 {
                    for z in 0..16 {
                        let block = BlockId::from_vanilla_id(counter);
                        chunk.set_block_at(x, (section * 16) + y, z, block);
                        assert_eq!(chunk.block_at(x, (section * 16) + y, z), block);
                        if counter != 0 {
                            assert!(
                                chunk.section(section).is_some(),
                                "Section {} failed",
                                section
                            );
                        }
                        counter += 1;
                    }
                }
            }
        }

        // Go through again to be sure
        for section in 0..16 {
            assert!(chunk.section(section).is_some());
            let mut counter = 0;
            for x in 0..16 {
                for y in 0..16 {
                    for z in 0..16 {
                        let block = BlockId::from_vanilla_id(counter);
                        assert_eq!(chunk.block_at(x, (section * 16) + y, z), block);
                        assert!(chunk.section(section).is_some());
                        counter += 1;
                    }
                }
            }
        }

        // Now, empty the chunk, call optimize(), and ensure
        // that the sections become empty.
        for x in 0..16 {
            for y in 0..256 {
                for z in 0..16 {
                    chunk.set_block_at(x, y, z, BlockId::air());
                }
            }
        }

        chunk.optimize();

        for section in chunk.sections() {
            assert!(section.is_none());
        }
    }

    #[test]
    fn section_from_data_and_palette() {
        let pos = ChunkPosition::new(0, 0);
        let mut chunk = Chunk::new(pos);

        let mut data = BitArray::new(5, 4096);
        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    data.set(block_index(x, y, z), 0);
                }
            }
        }

        let palette = vec![BlockId::stone()];
        let section = ChunkSection::new(
            data,
            Some(palette),
            BitArray::new(4, SECTION_VOLUME),
            BitArray::new(4, SECTION_VOLUME),
        );
        chunk.set_section_at(0, Some(section));

        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    assert_eq!(chunk.block_at(x, y, z), BlockId::stone());
                }
            }
        }
    }

    #[test]
    fn bit_array() {
        let mut barr = BitArray::new(5, 4096);
        assert_eq!(barr.highest_possible_value(), 31);

        for i in 0..4096 {
            barr.set(i, 8);
            assert_eq!(barr.get(i), 8);
        }

        for i in 0..4096 {
            assert_eq!(barr.get(i), 8);
        }

        let resized = barr.resize_to(8).unwrap();
        for i in 0..4096 {
            assert_eq!(resized.get(i), 8);
        }

        let resized = barr.resize_to(4).unwrap();
        for i in 0..4096 {
            assert_eq!(resized.get(i), 8);
        }
    }

    #[test]
    fn bit_array_resize_fail() {
        let mut barr = BitArray::new(5, 4096);

        for i in 0..4096 {
            barr.set(i, 31);
        }

        assert!(barr.resize_to(4).is_err());
    }

    #[test]
    fn bit_array_big_test() {
        let mut barr = BitArray::new(14, 4096);

        for i in 0..4096 {
            barr.set(i, i as u64);
            assert_eq!(barr.get(i), i as u64);
            if i != 4095 {
                assert_eq!(barr.get(i + 1), 0);
            }
            if i != 0 {
                assert_eq!(barr.get(i - 1), (i - 1) as u64);
            }
        }

        for i in 0..4096 {
            assert_eq!(barr.get(i), i as u64);
        }
    }

    #[test]
    fn bit_array_resize() {
        let mut barr = BitArray::new(12, 4096);
        assert_eq!(barr.bits_per_value, 12);

        for i in 0..4096 {
            barr.set(i, i as u64);
            assert_eq!(barr.get(i), i as u64);
        }

        let mut barr = barr.resize_to(13).unwrap();
        assert_eq!(barr.bits_per_value, 13);

        for i in 0..4096 {
            assert_eq!(barr.get(i), i as u64);
            barr.set(i, (i + 1) as u64);
            assert_eq!(barr.get(i), (i + 1) as u64);
        }

        let mut barr = barr.resize_to(14).unwrap();
        assert_eq!(barr.bits_per_value, 14);

        for i in 0..4096 {
            assert_eq!(barr.get(i), (i + 1) as u64);
            barr.set(i, i as u64);
            assert_eq!(barr.get(i), i as u64);
        }

        for i in 0..4096 {
            assert_eq!(barr.get(i), i as u64);
        }
    }

    #[test]
    fn test_needed_bits() {
        assert_eq!(needed_bits(31), 5);
        assert_eq!(needed_bits(255), 8);
        assert_eq!(needed_bits(256), 9);
        assert_eq!(needed_bits(1), 1);
    }

    #[test]
    fn test_block_index() {
        assert_eq!(block_index(0, 1, 0), 256);
        assert_eq!(block_index(1, 1, 1), 256 + 16 + 1);
    }

    #[test]
    fn test_correct_data_and_palette() {
        let mut data = BitArray::new(4, 4096);
        let mut palette = vec![
            BlockId::redstone_wire(),
            BlockId::stone(),
            BlockId::stone_brick_slab(),
            BlockId::sea_pickle(),
            BlockId::acacia_button(),
        ];
        ChunkSection::correct_data_and_palette(&mut data, &mut palette);
        assert_eq!(palette.len(), 5);

        let mut sorted = palette.clone();
        sorted.sort();
        assert_eq!(sorted, palette);
    }

    #[test]
    fn test_palette_insertion_in_middle() {
        let mut chunk = ChunkSection::default();

        chunk.set_block_at(0, 0, 0, BlockId::cobblestone());
        chunk.set_block_at(0, 1, 0, BlockId::stone());

        assert_eq!(chunk.block_at(0, 0, 0), BlockId::cobblestone());
        assert_eq!(chunk.block_at(0, 1, 0), BlockId::stone());
    }

    #[test]
    fn test_biomes() {
        let mut chunk = Chunk::default();

        for x in 0..SECTION_WIDTH {
            for z in 0..SECTION_WIDTH {
                assert_eq!(chunk.biome_at(x, z), Biome::Plains);
                chunk.set_biome_at(x, z, Biome::BirchForest);
                assert_eq!(chunk.biome_at(x, z), Biome::BirchForest);
            }
        }
    }

    #[test]
    fn test_modified() {
        let mut chunk = Chunk::default();
        assert!(chunk.check_modified());
        assert!(!chunk.check_modified());

        chunk.set_block_at(0, 0, 0, BlockId::stone());
        assert!(chunk.check_modified());
        assert!(!chunk.check_modified());
    }

    #[test]
    fn test_convert_section_to_palette() {
        let mut chunk = Chunk::default();

        let mut counter = 0;
        for x in 0..SECTION_WIDTH {
            for y in 0..SECTION_HEIGHT {
                for z in 0..SECTION_WIDTH {
                    chunk.set_block_at(x, y, z, BlockId::from_vanilla_id(counter));

                    counter += 1;
                }
            }
        }

        let section = chunk.section_mut(0).unwrap();
        section.convert_palette_to_section();

        assert_eq!(section.palette().unwrap().len(), counter as usize);

        // Ensure that data array still represents the same data
        counter = 0;
        for x in 0..SECTION_WIDTH {
            for y in 0..SECTION_HEIGHT {
                for z in 0..SECTION_WIDTH {
                    assert_eq!(chunk.block_at(x, y, z), BlockId::from_vanilla_id(counter));
                    counter += 1;
                }
            }
        }
    }

    #[test]
    fn test_light() {
        let mut chunk = Chunk::default();

        for x in 0..SECTION_WIDTH {
            for y in 0..SECTION_HEIGHT {
                for z in 0..SECTION_WIDTH {
                    chunk.set_block_light_at(x, y, z, 10);
                    chunk.set_sky_light_at(x, y, z, 8);
                    assert_eq!(chunk.block_light_at(x, y, z), 10);
                    assert_eq!(chunk.sky_light_at(x, y, z), 8);
                }
            }
        }
    }
}
