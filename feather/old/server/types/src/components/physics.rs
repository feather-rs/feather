//! Assorted components relating to physics
//! and systems to initialize them.

use glm::DVec3;
use ncollide3d::bounding_volume::AABB;

pub const DEFAULT_SLIP_MULTIPLIER: f64 = 0.6;

/// Component for entities with physics applied to them.
///
/// Physics will only be performed on entities with this component.
///
/// Typically, this component should be constructed using `PhysicsBuilder`.
#[derive(Debug)]
pub struct Physics {
    /// This entity's bounding box.
    pub bbox: AABB<f64>,
    /// The drag coefficient for this entity. Each tick,
    /// the entity's velocity will be multiplied by this amount
    /// (so higher values cause less drag).
    pub drag: f64,
    /// Gravitational acceleration for this entity. Each tick,
    /// this value will be added to the entity's Y speed.
    ///
    /// This value should generally be negative.
    pub gravity: f64,
    /// Slip multiplier for this entity. When on the ground,
    /// the X and Z velocities will be multiplied by this amount
    /// each tick.
    ///
    /// This value is `DEFAULT_SLIP_MULTIPLIER` for most entities.
    pub slip_multiplier: f64,
}

/// Builder for physics components.
pub struct PhysicsBuilder {
    comp: Physics,
}

impl Default for PhysicsBuilder {
    fn default() -> Self {
        let comp = Physics {
            bbox: bbox(0.5, 0.5, 0.5),
            drag: 0.98,
            gravity: -0.08,
            slip_multiplier: DEFAULT_SLIP_MULTIPLIER,
        };
        Self { comp }
    }
}

impl PhysicsBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a `PhysicsBuilder` with defaults set to the settings
    /// for living entities.
    pub fn for_living() -> Self {
        Self::new().drag(0.98).gravity(-0.08).slip_multiplier(0.6)
    }

    pub fn bbox(mut self, x: f64, y: f64, z: f64) -> Self {
        self.comp.bbox = bbox(x, y, z);
        self
    }

    pub fn drag(mut self, drag: f64) -> Self {
        self.comp.drag = drag;
        self
    }

    pub fn gravity(mut self, gravity: f64) -> Self {
        self.comp.gravity = gravity;
        self
    }

    pub fn slip_multiplier(mut self, slip_multiplier: f64) -> Self {
        self.comp.slip_multiplier = slip_multiplier;
        self
    }

    pub fn build(self) -> Physics {
        self.comp
    }
}

pub trait AABBExt {
    /// Returns the difference between the two
    /// corners of this bounding box.
    fn size(&self) -> DVec3;
}

impl AABBExt for AABB<f64> {
    fn size(&self) -> DVec3 {
        self.maxs() - self.mins()
    }
}

/// Returns a bounding box with the given width and height.
pub fn bbox(size_x: f64, size_y: f64, size_z: f64) -> AABB<f64> {
    AABB::new(
        glm::vec3(0.0, 0.0, 0.0).into(),
        glm::vec3(size_x, size_y, size_z).into(),
    )
}
