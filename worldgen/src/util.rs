use num_traits::ToPrimitive;

pub fn scramble(seed: u64, with: u64) -> u64 {
    // TODO: better algorithm. I just wrote random arithmetic.
    seed.wrapping_mul(with)
        .wrapping_add(with ^ seed)
        .wrapping_mul(with.wrapping_shl(32).wrapping_add(0x86cf))
}

pub fn scramble2(seed: u64, a: i32, b: i32) -> u64 {
    scramble(seed, ((a as u64) << 32) | b as u64)
}

pub trait IntoUsize {
    fn into_usize(self) -> usize;
}

impl<T> IntoUsize for T
where
    T: ToPrimitive,
{
    fn into_usize(self) -> usize {
        self.to_usize().expect("failed to convert to `usize`")
    }
}
