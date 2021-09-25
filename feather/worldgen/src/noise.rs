use num_traits::ToPrimitive;

/// Struct for applying linear interpolation to a 3D
/// density array.
pub struct NoiseLerper<'a> {
    /// The density values.
    densities: &'a [f32],
    /// The size of the chunk to generate along X and Z axes.
    size_horizontal: u32,
    /// The size of the chunk to generate along the Y axis.
    size_vertical: u32,
    /// The offset along the X axis to generate.
    offset_x: i32,
    /// The offset along the Z axis to generate.
    offset_z: i32,
    /// The scale along the X and Z axes. Must be a divisor of size_horizontal.
    scale_horizontal: u32,
    /// The scale along the Y axis. Must be a divisor of size_vertical.
    scale_vertical: u32,
}

impl<'a> NoiseLerper<'a> {
    /// Initializes with default settings and the given
    /// density values.
    ///
    /// Default settings are intended to match the size
    /// of chunks. Horizontal and vertical size and scale
    /// are initialized to sane defaults.
    pub fn new(densities: &'a [f32]) -> Self {
        Self {
            densities,
            size_horizontal: 16,
            size_vertical: 256,
            offset_x: 0,
            offset_z: 0,
            scale_horizontal: 4,
            scale_vertical: 8,
        }
    }

    /// Sets the size of the chunk to be generated.
    pub fn with_size(mut self, xz: u32, y: u32) -> Self {
        self.size_horizontal = xz;
        self.size_vertical = y;
        self
    }

    /// Sets the X and Z offsets.
    ///
    /// # Notes
    /// * The X and Z offsets are multiplied by the horizontal and vertical
    /// sizes, respectively, to obtain the offset in absolute coordinates.
    /// (This means there is no need to multiply the chunk coordinate by 16.)
    pub fn with_offset(mut self, x: i32, z: i32) -> Self {
        self.offset_x = x;
        self.offset_z = z;
        self
    }

    /// Sets the scale of the noise. Linear interpolation
    /// is used between values based on this scale.
    pub fn with_scale(mut self, horizontal: u32, vertical: u32) -> Self {
        self.scale_horizontal = horizontal;
        self.size_vertical = vertical;
        self
    }

    /// Generates a linear-interpolated block of noise.
    /// The returned vector will have length `size_horizontal^2 * size_vertical`,
    /// indexable by `((y << 12) | z << 4) | x`.
    pub fn generate(&self) -> Vec<f32> {
        // If AVX2 is available, use it. Otherwise,
        // default to a scalar impl.
        // TODO: support SSE41, other SIMD instruction sets

        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        {
            if is_x86_feature_detected!("avx2") {
                return self.generate_avx2();
            }
        }

        self.generate_fallback()
    }

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn generate_avx2(&self) -> Vec<f32> {
        // TODO: implement this. (Premature optimization is bad!)
        self.generate_fallback()
    }

    fn generate_fallback(&self) -> Vec<f32> {
        // Loop through values offsetted by the scale.
        // Then, loop through all coordinates inside
        // that subchunk and apply linear interpolation.

        // This is based on Glowstone's OverworldGenerator.generateRawTerrain
        // with a few modifications and superior variable names.

        // Number of subchunks in a chunk along each axis.
        let subchunk_horizontal = self.size_horizontal / self.scale_horizontal;
        let subchunk_vertical = self.size_vertical / self.scale_vertical;

        // Density noise, with one value every `scale` blocks along each axis.
        // Indexing into this vector is done using `self.uninterpolated_index(x, y, z)`.
        let densities = self.densities;

        // Buffer to emit final noise into.
        // TODO: consider using Vec::set_len to avoid zeroing it out
        let mut buf =
            vec![0.0; (self.size_horizontal * self.size_horizontal * self.size_vertical) as usize];

        let scale_vertical = self.scale_vertical as f32;
        let scale_horizontal = self.scale_horizontal as f32;

        // Coordinates of the subchunk. The subchunk
        // is the chunk within the chunk in which we
        // only find the noise value for the corners
        // and then apply interpolation in between.

        // Here, we loop through the subchunks and interpolate
        // noise for each block within it.
        for subx in 0..subchunk_horizontal {
            for suby in 0..subchunk_vertical {
                for subz in 0..subchunk_horizontal {
                    // Two grids of noise values:
                    // one for the four bottom corners
                    // of the subchunk, and one for the
                    // offsets along the Y axis to apply
                    // to those base corners each block increment.

                    // These are mutated so that they are at the
                    // current Y position.
                    let mut base1 = densities[self.uninterpolated_index(subx, suby, subz)];
                    let mut base2 = densities[self.uninterpolated_index(subx + 1, suby, subz)];
                    let mut base3 = densities[self.uninterpolated_index(subx, suby, subz + 1)];
                    let mut base4 = densities[self.uninterpolated_index(subx + 1, suby, subz + 1)];

                    // Offsets for each block along the Y axis from each corner above.
                    let offset1 = (densities[self.uninterpolated_index(subx, suby + 1, subz)]
                        - base1)
                        / scale_vertical;
                    let offset2 = (densities[self.uninterpolated_index(subx + 1, suby + 1, subz)]
                        - base2)
                        / scale_vertical;
                    let offset3 = (densities[self.uninterpolated_index(subx, suby + 1, subz + 1)]
                        - base3)
                        / scale_vertical;
                    let offset4 = (densities
                        [self.uninterpolated_index(subx + 1, suby + 1, subz + 1)]
                        - base4)
                        / scale_vertical;

                    // Iterate through the blocks in this subchunk
                    // and apply interpolation before setting the
                    // noise value in the final buffer.
                    for blocky in 0..self.scale_vertical {
                        let mut z_base = base1;
                        let mut z_corner = base3;
                        for blockx in 0..self.scale_horizontal {
                            let mut density = z_base;
                            for blockz in 0..self.scale_horizontal {
                                // Set interpolated value in buffer.
                                buf[index(
                                    blockx + (self.scale_horizontal * subx),
                                    blocky + (self.scale_vertical * suby),
                                    blockz + (self.scale_horizontal * subz),
                                )] = density;

                                // Apply Z interpolation.
                                density += (z_corner - z_base) / scale_horizontal;
                            }
                            // Interpolation along X.
                            z_base += (base2 - base1) / scale_horizontal;
                            // Along Z again.
                            z_corner += (base4 - base3) / scale_horizontal;
                        }

                        // Interpolation along Y.
                        base1 += offset1;
                        base2 += offset2;
                        base3 += offset3;
                        base4 += offset4;
                    }
                }
            }
        }

        buf
    }

    fn uninterpolated_index<N: ToPrimitive>(&self, x: N, y: N, z: N) -> usize {
        let length = (self.size_horizontal / self.scale_horizontal + 1) as usize;
        let height = (self.size_vertical / self.scale_vertical + 1) as usize;

        let x = x.to_usize().unwrap();
        let y = y.to_usize().unwrap();
        let z = z.to_usize().unwrap();

        y * length + x + height * length * z
    }
}

pub fn index<N: ToPrimitive>(x: N, y: N, z: N) -> usize {
    let x = x.to_usize().unwrap();
    let y = y.to_usize().unwrap();
    let z = z.to_usize().unwrap();

    ((y << 8) | z << 4) | x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        let densities = [0.0; 5 * 33 * 5];
        let noise = NoiseLerper::new(&densities).with_offset(10, 16);

        let chunk = noise.generate();

        assert_eq!(chunk.len(), 16 * 256 * 16);

        for x in chunk {
            approx::assert_relative_eq!(x, 0.0);
        }
    }
}
