use arrayvec::ArrayVec;
use feather_items::Item;
use feather_util::{vec3, Position, Vec3d};
use nbt::Value;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum EntityDataKind {
    Item,
    Arrow,
    Cow,
    Pig,
    Chicken,
    Sheep,
    Horse,
    Llama,
    Mooshroom,
    Rabbit,
    Squid,
    Donkey,
    Unknown,
}

impl<'a> From<&'a EntityData> for EntityDataKind {
    fn from(data: &'a EntityData) -> Self {
        match data {
            EntityData::Arrow(_) => EntityDataKind::Arrow,
            EntityData::Item(_) => EntityDataKind::Item,
            EntityData::Cow(_) => EntityDataKind::Cow,
            EntityData::Pig(_) => EntityDataKind::Pig,
            EntityData::Chicken(_) => EntityDataKind::Chicken,
            EntityData::Sheep(_) => EntityDataKind::Sheep,
            EntityData::Horse(_) => EntityDataKind::Horse,
            EntityData::Llama(_) => EntityDataKind::Llama,
            EntityData::Mooshroom(_) => EntityDataKind::Mooshroom,
            EntityData::Rabbit(_) => EntityDataKind::Rabbit,
            EntityData::Squid(_) => EntityDataKind::Squid,
            EntityData::Donkey(_) => EntityDataKind::Donkey,
            EntityData::Unknown => EntityDataKind::Unknown,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "id")]
pub enum EntityData {
    #[serde(rename = "minecraft:item")]
    Item(ItemEntityData),
    #[serde(rename = "minecraft:arrow")]
    Arrow(ArrowEntityData),
    #[serde(rename = "minecraft:cow")]
    Cow(AnimalData),
    #[serde(rename = "minecraft:pig")]
    Pig(AnimalData),
    #[serde(rename = "minecraft:chicken")]
    Chicken(AnimalData),
    #[serde(rename = "minecraft:sheep")]
    Sheep(AnimalData),
    #[serde(rename = "minecraft:horse")]
    Horse(AnimalData),
    #[serde(rename = "minecraft:llama")]
    Llama(AnimalData),
    #[serde(rename = "minectaft:mooshroom")]
    Mooshroom(AnimalData),
    #[serde(rename = "minecraft:rabbit}")]
    Rabbit(AnimalData),
    #[serde(rename = "minecraft:squid")]
    Squid(AnimalData),
    #[serde(rename = "minecraft:donkey")]
    Donkey(AnimalData),

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
                    EntityData::Cow(_) => "minecraft:cow",
                    EntityData::Pig(_) => "minecraft:pig",
                    EntityData::Chicken(_) => "minecraft:chicken",
                    EntityData::Sheep(_) => "minecraft:sheep",
                    EntityData::Horse(_) => "minecraft:horse",
                    EntityData::Llama(_) => "minecraft:llama",
                    EntityData::Mooshroom(_) => "minecraft:mooshroom",
                    EntityData::Rabbit(_) => "minecraft:rabbit",
                    EntityData::Squid(_) => "minecraft:squid",
                    EntityData::Donkey(_) => "minecraft:donkey",
                    EntityData::Unknown => panic!("Cannot write unknown entities"),
                }
                .to_string(),
            ),
        );

        match self {
            EntityData::Item(data) => data.write_to_map(&mut map),
            EntityData::Arrow(data) => data.write_to_map(&mut map),
            EntityData::Cow(data) => data.write_to_map(&mut map),
            EntityData::Pig(data) => data.write_to_map(&mut map),
            EntityData::Chicken(data) => data.write_to_map(&mut map),
            EntityData::Sheep(data) => data.write_to_map(&mut map),
            EntityData::Horse(data) => data.write_to_map(&mut map),
            EntityData::Llama(data) => data.write_to_map(&mut map),
            EntityData::Mooshroom(data) => data.write_to_map(&mut map),
            EntityData::Rabbit(data) => data.write_to_map(&mut map),
            EntityData::Squid(data) => data.write_to_map(&mut map),
            EntityData::Donkey(data) => data.write_to_map(&mut map),
            EntityData::Unknown => unreachable!(),
        }

        Value::Compound(map)
    }
}

/// Common entity tags.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseEntityData {
    #[serde(rename = "Pos")]
    pub position: ArrayVec<[f64; 3]>,
    #[serde(rename = "Rotation")]
    pub rotation: ArrayVec<[f32; 2]>,
    #[serde(rename = "Motion")]
    pub velocity: ArrayVec<[f64; 3]>,
}

impl BaseEntityData {
    fn write_to_map(self, map: &mut HashMap<String, Value>) {
        map.insert(
            String::from("Pos"),
            Value::List(self.position.iter().copied().map(Value::Double).collect()),
        );
        map.insert(
            String::from("Rotation"),
            Value::List(self.rotation.iter().copied().map(Value::Float).collect()),
        );
        map.insert(
            String::from("Motion"),
            Value::List(self.velocity.iter().copied().map(Value::Double).collect()),
        );
    }
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum EntityLoadError {
    #[error("missing position/rotation/velocity data")]
    MissingData,
}

impl BaseEntityData {
    /// Creates a `BaseEntityData` from its parameters.
    pub fn new(pos: Position, velocity: Vec3d) -> Self {
        Self {
            position: [pos.x, pos.y, pos.z].into(),
            rotation: [pos.yaw, pos.pitch].into(),
            velocity: [velocity.x, velocity.y, velocity.z].into(),
        }
    }

    /// Reads the position and rotation fields. If the fields are invalid, an error is returned.
    pub fn read_position(self: &BaseEntityData) -> Result<Position, EntityLoadError> {
        if self.position.len() == 3 && self.rotation.len() == 2 {
            Ok(Position {
                x: self.position[0],
                y: self.position[1],
                z: self.position[2],
                yaw: self.rotation[0],
                pitch: self.rotation[1],
                on_ground: true,
            })
        } else {
            Err(EntityLoadError::MissingData)
        }
    }

    /// Reads the velocity field. If the field is invalid, an error is returned.
    pub fn read_velocity(self: &BaseEntityData) -> Result<Vec3d, EntityLoadError> {
        if self.velocity.len() == 3 {
            Ok(vec3(self.velocity[0], self.velocity[1], self.velocity[2]))
        } else {
            Err(EntityLoadError::MissingData)
        }
    }
}

impl Default for BaseEntityData {
    fn default() -> Self {
        BaseEntityData {
            position: [0.0, 0.0, 0.0].into(),
            rotation: [0.0, 0.0].into(),
            velocity: [0.0, 0.0, 0.0].into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimalData {
    #[serde(flatten)]
    pub base: BaseEntityData,
    #[serde(rename = "Health")]
    pub health: f32,
}

impl AnimalData {
    fn write_to_map(self, map: &mut HashMap<String, Value>) {
        self.base.write_to_map(map);
        map.insert(String::from("Health"), Value::Float(self.health));
    }
}

impl AnimalData {
    /// Creates an `AnimalData` from its parameters.
    pub fn new(base: BaseEntityData, health: f32) -> Self {
        Self { base, health }
    }
}

impl Default for AnimalData {
    fn default() -> Self {
        AnimalData {
            base: Default::default(),
            health: 20.0,
        }
    }
}

/// Represents a single item, without slot information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemData {
    #[serde(rename = "Count")]
    pub count: i8,
    #[serde(rename = "id")]
    pub item: String,
}

impl ItemData {
    fn write_to_map(self, map: &mut HashMap<String, Value>) {
        map.insert(String::from("Count"), Value::Byte(self.count));
        map.insert(String::from("id"), Value::String(self.item));
    }
}

impl Default for ItemData {
    fn default() -> Self {
        Self {
            count: 0,
            item: Item::Air.identifier().to_string(),
        }
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
    pub pickup_delay: i16,
    #[serde(rename = "Item")]
    pub item: ItemData,
    #[serde(rename = "Health")]
    pub health: i16,
}

impl ItemEntityData {
    fn write_to_map(self, map: &mut HashMap<String, Value>) {
        self.entity.write_to_map(map);

        let mut item = HashMap::new();
        self.item.write_to_map(&mut item);
        map.insert(String::from("Item"), Value::Compound(item));

        map.insert(String::from("Age"), Value::Short(self.age));
        map.insert(String::from("PickupDelay"), Value::Short(self.pickup_delay));
        map.insert(String::from("Health"), Value::Short(self.health));
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
    pub critical: i8,
}

impl ArrowEntityData {
    fn write_to_map(self, map: &mut HashMap<String, Value>) {
        self.entity.write_to_map(map);

        map.insert(String::from("crit"), Value::Byte(self.critical));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use feather_util::position;

    #[test]
    fn test_read_position() {
        let data = BaseEntityData {
            position: [1.0, 2.0, 3.0].into(),
            rotation: [4.0, 5.0].into(),
            velocity: [6.0, 7.0, 8.0].into(),
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
    fn test_read_velocity() {
        let data = BaseEntityData {
            position: [1.0, 2.0, 3.0].into(),
            rotation: [4.0, 5.0].into(),
            velocity: [6.0, 7.0, 8.0].into(),
        };
        let vel = data.read_velocity().unwrap();

        assert!(vel[0] - 6.0 < std::f64::EPSILON);
        assert!(vel[1] - 7.0 < std::f64::EPSILON);
        assert!(vel[2] - 8.0 < std::f64::EPSILON);
    }

    #[test]
    fn test_new() {
        let pos = position!(1.0, 10.0, 3.0, 115.0, -3.0);
        let vel = vec3(0.0, 1.0, 2.0);

        let data = BaseEntityData::new(pos, vel);
        assert_eq!(data.read_position(), Ok(pos));
        assert_eq!(data.read_velocity(), Ok(vel));
    }
}
