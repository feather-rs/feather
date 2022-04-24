use crate::data::{RawBlockProperties, RawBlockState, RawBlockStateProperties, ValidProperties};
use crate::{BlockData, BlockKind, SimplifiedBlockKind};

use ahash::AHashMap;
use bytemuck::{Pod, Zeroable};
use once_cell::sync::Lazy;
use serde::{de, Deserialize, Serialize};
use smartstring::{LazyCompact, SmartString};

use std::collections::BTreeMap;
use std::fmt::Debug;
use std::io::Cursor;

/// A block state.
///
/// A block state is composed of:
/// * A _kind_, represented by the [`BlockKind`](crate::BlockKind)
/// enum. Each block kind corresponds to a Minecraft block, like "red wool"
/// or "chest."
/// * _Data_, or properties, represented by structs implementing the [`BlockData`](crate::BlockData)
/// trait. For example, a chest has a "type" property in its block data
/// that determines whether the chest is single or double.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Zeroable, Pod, Default)]
#[repr(transparent)]
pub struct BlockState {
    id: u16,
}

impl BlockState {
    /// Gets the default block state for the given block kind.
    pub fn new(kind: BlockKind) -> Self {
        REGISTRY.default_state(kind)
    }

    /// Gets the kind of this block.
    pub fn kind(self) -> BlockKind {
        self.raw().kind
    }

    /// Gets the `SimplifiedKind` of this block.
    pub fn simplified_kind(self) -> SimplifiedBlockKind {
        self.kind().simplified_kind()
    }

    /// Gets this block as a struct implementing the [`BlockData`](crate::BlockData)
    /// interface.
    ///
    /// If this block is not an instance of `T`, then returns `None`.
    ///
    /// # Warning
    /// The returned `BlockData` is not linked with this `BlockState` instance.
    /// You need to call [`BlockState::set_data`] to apply any changes made to the block data.
    pub fn data_as<T: BlockData>(self) -> Option<T> {
        T::from_raw(&self.raw().properties, self.get_valid_properties())
    }

    /// Applies the given `BlockData` to this block state.
    ///
    /// All property values in `data` override existing properties
    /// in `self`.
    pub fn set_data<T: BlockData>(&mut self, data: T) {
        let mut raw = self.raw().properties.clone();
        data.apply(&mut raw);
        if let Some(new_block) = Self::from_raw(&raw, self.kind()) {
            *self = new_block;
        }
    }

    /// Returns a new block state with the given property values applied.
    pub fn with_data<T: BlockData>(self, data: T) -> Self {
        let mut copy = self;
        copy.set_data(data);
        copy
    }

    /// Returns whether this is the default block state for
    /// the block kind.
    pub fn is_default(self) -> bool {
        self.raw().default
    }

    /// Gets the ID of this block state.
    ///
    /// Block state IDs are not stable between Minecraft versions.
    pub fn id(self) -> u16 {
        self.id
    }

    /// Creates a block state from an ID.
    /// Returns `None` if the ID is invalid.
    ///
    /// Block state IDs are not stable between Minecraft versions.
    pub fn from_id(id: u16) -> Option<Self> {
        let _state = REGISTRY.raw_state(id)?;
        Some(Self { id })
    }

    /// Determines whether this block state is valid.
    pub fn is_valid(self) -> bool {
        REGISTRY.raw_state(self.id).is_some()
    }

    /// Gets the stable namespaced ID of the block kind.
    ///
    /// Combined with `property_values`, this method can be used
    /// for the persistent serialization of block states.
    pub fn namespaced_id(&self) -> &str {
        self.kind().namespaced_id()
    }

    /// Returns an iterator over (key, value) pairs representing
    /// the properties of this block.
    ///
    /// This method can be used to serialize block states.
    pub fn property_values(&self) -> impl Iterator<Item = (&str, &str)> + '_ {
        self.raw()
            .untyped_properties
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
    }

    /// Creates a `BlockState` from its namespaced ID and property values.
    ///
    /// This method can be used to deserialize block states.
    pub fn from_namespaced_id_and_property_values<'a>(
        namespaced_id: &str,
        property_values: impl IntoIterator<Item = (&'a str, &'a str)>,
    ) -> Option<Self> {
        REGISTRY
            .id_for_untyped_repr(namespaced_id, property_values)
            .map(|id| Self { id })
    }

    pub fn get_valid_properties(&self) -> &'static ValidProperties {
        REGISTRY.valid_properties.get(&self.raw().kind).unwrap()
    }

    /// Gets the raw block state for this block state.
    pub(crate) fn raw(&self) -> &RawBlockState {
        REGISTRY.raw_state(self.id).expect("bad block")
    }

    /// Creates a block state from its raw properties.
    pub(crate) fn from_raw(raw: &RawBlockStateProperties, kind: BlockKind) -> Option<Self> {
        let id = REGISTRY.id_for_state(raw, kind)?;
        Some(Self { id })
    }
}

#[derive(Serialize, Deserialize)]
struct SerializedBlockState<'a> {
    kind: &'a str,
    properties: BTreeMap<&'a str, &'a str>,
}

impl Serialize for BlockState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        SerializedBlockState {
            kind: self.namespaced_id(),
            properties: self.property_values().collect(),
        }
        .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for BlockState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let serialized = SerializedBlockState::deserialize(deserializer)?;
        BlockState::from_namespaced_id_and_property_values(serialized.kind, serialized.properties)
            .ok_or_else(|| de::Error::custom("invalid block state"))
    }
}

impl Debug for BlockState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_struct("BlockState");
        s.field("kind", &self.kind());
        for (property, value) in self.property_values() {
            s.field(property, &value);
        }
        s.finish()
    }
}

static REGISTRY: Lazy<BlockRegistry> = Lazy::new(BlockRegistry::new);

type SmartStr = SmartString<LazyCompact>;
type PropertyValues = Vec<(SmartStr, SmartStr)>;

struct BlockRegistry {
    states: Vec<RawBlockState>,
    id_mapping: AHashMap<(BlockKind, RawBlockStateProperties), u16>,
    valid_properties: AHashMap<BlockKind, ValidProperties>,
    default_states: AHashMap<BlockKind, BlockState>,
    default_property_values: AHashMap<BlockKind, PropertyValues>,
    by_untyped_repr: AHashMap<(SmartStr, PropertyValues), u16>,
}

fn set_defualt_property_values(values: &mut PropertyValues, default: &PropertyValues) {
    for (key, value) in default {
        if !values.iter().any(|(k, _)| k == key) {
            values.push((key.clone(), value.clone()));
        }
    }
    values.sort_unstable();
}

impl BlockRegistry {
    fn new() -> Self {
        static STATE_DATA: &[u8] = include_bytes!("../../assets/raw_block_states.bc.gz");
        let mut state_reader = flate2::bufread::GzDecoder::new(Cursor::new(STATE_DATA));
        let states: Vec<RawBlockState> =
            bincode::decode_from_std_read(&mut state_reader, bincode::config::standard())
                .expect("malformed block state data");

        static PROPERTY_DATA: &[u8] = include_bytes!("../../assets/raw_block_properties.bc.gz");
        let mut property_reader = flate2::bufread::GzDecoder::new(Cursor::new(PROPERTY_DATA));
        let properties: Vec<RawBlockProperties> =
            bincode::decode_from_std_read(&mut property_reader, bincode::config::standard())
                .expect("malformed block properties");

        // Ensure that indexes match IDs.
        #[cfg(debug_assertions)]
        {
            for (index, state) in states.iter().enumerate() {
                assert_eq!(index, state.id as usize);
            }
        }

        let id_mapping = states
            .iter()
            .map(|state| ((state.kind, state.properties.clone()), state.id))
            .collect();

        let valid_properties = properties
            .iter()
            .map(|properties| (properties.kind, properties.valid_properties.clone()))
            .collect();

        let default_states = states
            .iter()
            .filter(|s| s.default)
            .map(|s| (s.kind, BlockState { id: s.id }))
            .collect();

        let by_untyped_repr: AHashMap<(SmartStr, PropertyValues), u16> = states
            .iter()
            .map(|s| {
                let mut props: PropertyValues = s
                    .untyped_properties
                    .iter()
                    .map(|(a, b)| (a.into(), b.into()))
                    .collect();
                props.sort_unstable();
                ((s.kind.namespaced_id().into(), props), s.id)
            })
            .collect();

        let default_property_values = states
            .iter()
            .filter(|s| s.default)
            .map(|s| {
                (
                    s.kind,
                    s.untyped_properties
                        .iter()
                        .map(|(a, b)| (a.into(), b.into()))
                        .collect(),
                )
            })
            .collect();

        Self {
            states,
            id_mapping,
            valid_properties,
            default_states,
            by_untyped_repr,
            default_property_values,
        }
    }

    fn raw_state(&self, id: u16) -> Option<&RawBlockState> {
        self.states.get(id as usize)
    }

    fn id_for_state(&self, state: &RawBlockStateProperties, kind: BlockKind) -> Option<u16> {
        self.id_mapping.get(&(kind, state.clone())).copied()
    }

    fn default_state(&self, kind: BlockKind) -> BlockState {
        self.default_states[&kind]
    }

    fn id_for_untyped_repr<'a>(
        &self,
        namespaced_id: impl Into<SmartStr>,
        property_values: impl IntoIterator<Item = (&'a str, &'a str)>,
    ) -> Option<u16> {
        let namespaced_id = namespaced_id.into();
        let kind = BlockKind::from_namespaced_id(&namespaced_id)?;
        let mut property_values: PropertyValues = property_values
            .into_iter()
            .map(|(k, v)| (k.into(), v.into()))
            .collect::<Vec<_>>();
        set_defualt_property_values(&mut property_values, &self.default_property_values[&kind]);
        self.by_untyped_repr
            .get(&(namespaced_id, property_values))
            .copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn block_registry_creates_successfully() {
        let _ = BlockRegistry::new();
    }
}
