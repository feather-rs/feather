use crate::util::IntoUsize;

/// Wrapper over `Vec`.
#[derive(Debug, Clone)]
pub struct Grid {
    pub(crate) vec: Vec<u16>,
    pub size_x: usize,
    pub size_z: usize,
}

impl Grid {
    /// Creates a new `Grid` with the provided dimensions.
    /// Values are initialized with 0.
    pub fn new(size_x: impl IntoUsize, size_z: impl IntoUsize) -> Self {
        let size_x = size_x.into_usize();
        let size_z = size_z.into_usize();
        Self {
            vec: vec![0; size_x * size_z],
            size_x,
            size_z,
        }
    }

    /// Retrieves the value at (x, z).
    pub fn at(&self, x: impl IntoUsize, z: impl IntoUsize) -> u16 {
        let x = x.into_usize();
        let z = z.into_usize();
        self.vec[x + (z * self.size_x)]
    }

    /// Sets the value at (x, z).
    pub fn set_at(&mut self, x: impl IntoUsize, z: impl IntoUsize, val: u16) {
        let x = x.into_usize();
        let z = z.into_usize();
        self.vec[x + (z * self.size_x)] = val;
    }
}
