use crate::Position;
use nbt::Value;
use std::collections::HashMap;

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

impl EntityData {
    pub fn into_nbt_value(self) -> Value {
        let mut map = HashMap::new();

        map.insert(
            String::from("id"),
            Value::String(
                match self {
                    EntityData::Item(_) => "minecraft:item",
                    EntityData::Arrow(_) => "minecraft:arrow",
                    EntityData::Unknown => panic!("Cannot write unknown entities"),
                }
                .to_string(),
            ),
        );

        match self {
            EntityData::Item(data) => data.write_to_map(&mut map),
            EntityData::Arrow(data) => data.write_to_map(&mut map),
            EntityData::Unknown => panic!("Cannot write unknown entities"),
        }

        Value::Compound(map)
    }
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
    fn write_to_map(self, map: &mut HashMap<String, Value>) {
        map.insert(
            String::from("Pos"),
            Value::List(self.position.into_iter().map(Value::Double).collect()),
        );
        map.insert(
            String::from("Rotation"),
            Value::List(self.rotation.into_iter().map(Value::Float).collect()),
        );
        map.insert(
            String::from("Motion"),
            Value::List(self.velocity.into_iter().map(Value::Double).collect()),
        );
    }
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

impl ItemData {
    fn write_to_map(self, map: &mut HashMap<String, Value>) {
        map.insert(String::from("Count"), Value::Byte(self.count as i8));
        map.insert(String::from("id"), Value::String(self.item));
    }
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

impl ItemEntityData {
    fn write_to_map(self, map: &mut HashMap<String, Value>) {
        self.entity.write_to_map(map);

        let mut item = HashMap::new();
        self.item.write_to_map(&mut item);
        map.insert(String::from("Item"), Value::Compound(item));

        map.insert(String::from("Age"), Value::Short(self.age));
        map.insert(
            String::from("PickupDelay"),
            Value::Byte(self.pickup_delay as i8),
        );
    }
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

impl ArrowEntityData {
    fn write_to_map(self, map: &mut HashMap<String, Value>) {
        self.entity.write_to_map(map);

        map.insert(String::from("crit"), Value::Byte(self.critical as i8));
    }
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
