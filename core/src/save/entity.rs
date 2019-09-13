use crate::Position;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "id")]
pub enum EntityData {
    #[serde(rename = "minecraft:item")]
    Item(ItemEntityData),
    #[serde(rename = "minecraft:arrow")]
    Arrow(ArrowEntityData),

    /// Fallback type for unknown entities
    #[serde(other)]
    Unknown,
}

/// Common entity tags.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseEntityData {
    #[serde(rename = "Pos")]
    pub position: Vec<f64>,
    #[serde(rename = "Rotation")]
    pub rotation: Vec<f32>,
    #[serde(rename = "Motion")]
    pub velocity: Vec<f64>,
}

impl BaseEntityData {
    /// Reads the position and rotation fields. If the fields are invalid, None is returned.
    pub fn read_position(self: &BaseEntityData) -> Option<Position> {
        if self.position.len() == 3 && self.rotation.len() == 2 {
            Some(Position {
                x: self.position[0],
                y: self.position[1],
                z: self.position[2],
                yaw: self.rotation[0],
                pitch: self.rotation[1],
                on_ground: true,
            })
        } else {
            None
        }
    }

    /// Reads the velocity field. If the field is invalid, None is returned.
    pub fn read_velocity(self: &BaseEntityData) -> Option<glm::DVec3> {
        if self.velocity.len() == 3 {
            Some(glm::vec3(
                self.velocity[0],
                self.velocity[1],
                self.velocity[2],
            ))
        } else {
            None
        }
    }
}

impl Default for BaseEntityData {
    fn default() -> Self {
        BaseEntityData {
            position: vec![0.0, 0.0, 0.0],
            rotation: vec![0.0, 0.0],
            velocity: vec![0.0, 0.0, 0.0],
        }
    }
}

/// Represents a single item, without slot information.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ItemData {
    #[serde(rename = "Count")]
    pub count: u8,
    #[serde(rename = "id")]
    pub item: String,
}

/// Data for an Item entity (`minecraft:item`).
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct ItemEntityData {
    // Inherit base entity data
    #[serde(flatten)]
    pub entity: BaseEntityData,

    // Item-specific tags
    #[serde(rename = "Age")]
    pub age: i16,
    #[serde(rename = "PickupDelay")]
    pub pickup_delay: u8,
    #[serde(rename = "Item")]
    pub item: ItemData,
}

/// Data for an Arrow entity (`minecraft:arrow`).
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ArrowEntityData {
    // Inherit base entity data
    #[serde(flatten)]
    pub entity: BaseEntityData,

    // Arrow-specific tags

    // TODO: Change this field to `bool` when issue with hematite_nbt is resolved.
    // See: https://github.com/PistonDevelopers/hematite_nbt/issues/43
    #[serde(rename = "crit")]
    pub critical: u8,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_position() {
        let data = BaseEntityData {
            position: vec![1.0, 2.0, 3.0],
            rotation: vec![4.0, 5.0],
            velocity: vec![6.0, 7.0, 8.0],
        };
        let pos = data.read_position().unwrap();

        assert!(pos.x - 1.0 < std::f64::EPSILON);
        assert!(pos.y - 2.0 < std::f64::EPSILON);
        assert!(pos.z - 3.0 < std::f64::EPSILON);
        assert!(pos.yaw - 4.0 < std::f32::EPSILON);
        assert!(pos.pitch - 5.0 < std::f32::EPSILON);
        assert!(pos.on_ground);
    }

    #[test]
    fn test_read_position_invalid() {
        let data = BaseEntityData {
            position: vec![1.0],
            rotation: vec![4.0, 5.0],
            velocity: vec![6.0, 7.0, 8.0],
        };
        let pos = data.read_position();
        assert!(pos.is_none());
    }

    #[test]
    fn test_read_position_invalid_rot() {
        let data = BaseEntityData {
            position: vec![1.0, 2.0, 3.0],
            rotation: vec![4.0],
            velocity: vec![6.0, 7.0, 8.0],
        };
        let pos = data.read_position();
        assert!(pos.is_none());
    }

    #[test]
    fn test_read_velocity() {
        let data = BaseEntityData {
            position: vec![1.0, 2.0, 3.0],
            rotation: vec![4.0, 5.0],
            velocity: vec![6.0, 7.0, 8.0],
        };
        let vel = data.read_velocity().unwrap();

        assert!(vel[0] - 6.0 < std::f64::EPSILON);
        assert!(vel[1] - 7.0 < std::f64::EPSILON);
        assert!(vel[2] - 8.0 < std::f64::EPSILON);
    }

    #[test]
    fn test_read_velocity_invalid() {
        let data = BaseEntityData {
            position: vec![1.0, 2.0, 3.0],
            rotation: vec![4.0, 5.0],
            velocity: vec![6.0, 7.0],
        };
        let vel = data.read_velocity();
        assert!(vel.is_none());
    }
}
