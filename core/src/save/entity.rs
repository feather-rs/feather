use crate::Position;

#[derive(Clone, Serialize, Deserialize, Debug)]
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
#[derive(Clone, Serialize, Deserialize, Debug)]
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
}

/// Represents a single item, without slot information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemData {
    #[serde(rename = "Count")]
    pub count: u8,
    #[serde(rename = "id")]
    pub item: String,
}

/// Data for an Item entity (`minecraft:item`).
#[derive(Clone, Serialize, Deserialize, Debug)]
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
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ArrowEntityData {
    // Inherit base entity data
    #[serde(flatten)]
    pub entity: BaseEntityData,

    // Arrow-specific tags
    #[serde(rename = "crit")]
    pub critical: u8,
}
