use num_traits::ToPrimitive;

/// Convenient wrapper over `simdnoise::GradientSettings`
/// which supports SIMD-accelerated amplitude and linear
/// interpolation.
///
/// The `simdnoise::GradientSettings::generate_scaled` function
/// cannot be used for noise amplification because it scales
/// the noise differently depending on the min and max value
/// in a chunk, creating very obvious seams between chunks.
/// For more information,  jackmott/rust-simd-noise#9.
///
/// This algorithm uses a different technique by multiplying
/// noise by a constant value provided in the `new` function.
/// This ensures that amplification remains constant across chunks.
///
/// # Notes
/// * The value initially retrieved from the noise function is
/// not within the range -1.0 to 1.0; it is undefined. Experimentation
/// is needed to obtain a suitable amplitude value for any given set of settings.
/// * The X and Z offsets are multiplied by the horizontal and vertical
/// sizes, respectively, to obtain the offset in absolute coordinates.
/// (This means there is no need to multiply the chunk coordinate by 16.)
pub struct Wrapped3DPerlinNoise {
    /// The seed used for noise generation.
    seed: u64,
    /// The frequency.
    frequency: f32,
    /// The amplitude.
    amplitude: f32,
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

impl Wrapped3DPerlinNoise {
    /// Initializes with default settings and the given seed.
    ///
    /// Default settings are intended to match the size
    /// of chunks. Horizontal and vertical size and scale
    /// are initialized to sane defaults.
    pub fn new(seed: u64) -> Self {
        Self {
            seed,
            frequency: 0.02,
            amplitude: 400.0,
            size_horizontal: 16,
            size_vertical: 256,
            offset_x: 0,
            offset_z: 0,
            scale_horizontal: 8,
            scale_vertical: 4,
        }
    }

    /// Sets the frequency.
    pub fn with_frequency(mut self, frequency: f32) -> Self {
        self.frequency = frequency;
        self
    }

    /// Sets the amplitude.
    pub fn with_amplitude(mut self, amplitude: f32) -> Self {
        self.amplitude = amplitude;
        self
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

        if is_x86_feature_detected!("avx2") {
            self.generate_avx2()
        } else {
            self.generate_fallback()
        }
    }

    fn generate_avx2(&self) -> Vec<f32> {
        // TODO: implement this. (Premature optimization is bad!)
        self.generate_fallback()
    }

    fn generate_fallback(&self) -> Vec<f32> {
        // Block of non-interpolated noise. We need to use
        // the raw generate functions because the distance between noise values
        // is not 1.0, and simdnoise doesn't yet support this.
        // (TODO: submit a PR to support this. This could be an effective optimization.)
        let uninterpolated = self.uninterpolated_noise_fallback();

        // Final buffer, which will contain a value for
        // every block. Linear interpolation is applied
        // to `uninterpolated` to find values in this buffer.
        let cap = (self.size_horizontal * self.size_horizontal * self.size_vertical) as usize;
        let mut buf = vec![0.0; cap]; // FIXME: don't zero out buffer

        // Apply interpolation.
        for x in 0..self.size_horizontal {
            for y in 0..self.size_vertical {
                for z in 0..self.size_horizontal {
                    // Find nearest two values along each axis.
                    let (nx1, nx2, ny1, ny2, nz1, nz2) = {
                        // Find next and previous index into `uninterpolated`
                        // for each axis.
                        let next_x = (x + self.scale_horizontal - 1) / self.scale_horizontal;
                        let prev_x = x / self.scale_horizontal;

                        let next_y = (y + self.scale_vertical - 1) / self.scale_vertical;
                        let prev_y = y / self.scale_vertical;

                        let next_z = (z + self.scale_horizontal - 1) / self.scale_horizontal;
                        let prev_z = z / self.scale_horizontal;

                        let x = x / self.scale_horizontal;
                        let y = y / self.scale_vertical;
                        let z = z / self.scale_horizontal;

                        // TODO: this is inefficient.
                        (
                            uninterpolated[self.uninterpolated_index(prev_x, y, z)],
                            uninterpolated[self.uninterpolated_index(next_x, y, z)],
                            uninterpolated[self.uninterpolated_index(x, prev_y, z)],
                            uninterpolated[self.uninterpolated_index(x, next_y, z)],
                            uninterpolated[self.uninterpolated_index(x, y, prev_z)],
                            uninterpolated[self.uninterpolated_index(x, y, next_z)],
                        )
                    };

                    // Interpolate between values.
                    let weight_x = (x % (self.size_horizontal / self.scale_horizontal)) as f32;
                    let weight_y = (y % (self.size_vertical / self.scale_vertical)) as f32;
                    let weight_z = (z % (self.size_horizontal / self.scale_horizontal)) as f32;

                    let val_x = ((nx1 * weight_x)
                        + (nx2 * (self.scale_horizontal as f32 - weight_x)))
                        / self.scale_horizontal as f32;
                    let val_y = ((ny1 * weight_y)
                        + (ny2 * (self.scale_vertical as f32 - weight_y)))
                        / self.scale_vertical as f32;
                    let val_z = ((nz1 * weight_z)
                        + (nz2 * (self.scale_horizontal as f32 - weight_z)))
                        / self.scale_horizontal as f32;

                    // Average of interpolation along each of three axes.
                    let val = (val_x + val_y + val_z) / 3.0;

                    // Set value in final buffer.
                    let index = index(x as usize, y as usize, z as usize);
                    buf[index] = val;
                }
            }
        }

        buf
    }

    fn uninterpolated_noise_fallback(&self) -> Vec<f32> {
        // Length and height of non-interpolated noise.
        // 1 is added to the sizes because we need to closest
        // noise value in the next chunk in order to interpolate
        // between.
        let length = self.size_horizontal / self.scale_horizontal + 1;
        let height = self.size_vertical / self.scale_vertical + 1;

        let cap = (length * length * height) as usize;
        let mut buf = vec![0.0; cap]; // FIXME: don't zero out buffer

        for x in 0..length {
            for y in 0..height {
                for z in 0..length {
                    // Offset values from chunk origin.
                    let offset_x = self.scale_horizontal * x;
                    let offset_y = self.scale_vertical * y;
                    let offset_z = self.scale_horizontal * z;

                    // Absolute offset values, from the world origin.
                    // self.offset_x is the offset in chunk coordinates,
                    // so multiply by the size of a chunk to obtain the
                    // absolute coordinates.
                    let abs_x = self.offset_x * self.size_horizontal as i32 + offset_x as i32;
                    let abs_y = offset_y;
                    let abs_z = self.offset_z * self.size_horizontal as i32 + offset_z as i32;

                    // Unmodified noise value.
                    // No idea why scalar noise is unsafe,
                    // since no SIMD is involved.
                    let mut value = unsafe {
                        simdnoise::scalar::simplex_3d(
                            abs_x as f32,
                            abs_y as f32,
                            abs_z as f32,
                            self.seed as i32,
                        )
                    };

                    // Apply amplitude to value.
                    value *= self.amplitude;

                    // Index into `buf`.
                    let index = self.uninterpolated_index(x as usize, y as usize, z as usize);

                    buf[index] = value;

                    println!("{} {} {} value {}", abs_x, abs_y, abs_z, value);
                }
            }
        }

        buf
    }

    fn uninterpolated_index<N: ToPrimitive>(&self, x: N, y: N, z: N) -> usize {
        let length = (self.size_horizontal / self.scale_horizontal + 1) as usize;

        let x = x.to_usize().unwrap();
        let y = y.to_usize().unwrap();
        let z = z.to_usize().unwrap();

        (y * (length * length) + (z * length) + x)
    }
}

pub fn index(x: usize, y: usize, z: usize) -> usize {
    ((y << 8) | z << 4) | x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        let noise = Wrapped3DPerlinNoise::new(0)
            .with_amplitude(400.0)
            .with_offset(10, 16);

        let chunk = noise.generate();

        assert_eq!(chunk.len(), 16 * 256 * 16);
    }
}
