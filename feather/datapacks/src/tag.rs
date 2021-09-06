use std::{
    borrow::Borrow, cell::RefCell, collections::VecDeque, fmt::Display, fs::File, io::Read,
    path::Path, str::FromStr,
};

use crate::NamespacedId;
use ahash::{AHashMap, AHashSet};
use blocks::BlockId;
use generated::{BlockKind, EntityKind, Item};
use protocol::{
    packets::server::{AllTags, Tag},
    VarInt,
};
use serde::Deserialize;
use smartstring::{Compact, SmartString};
use thiserror::Error;
use walkdir::WalkDir;

pub use generated::vanilla_tags::*;
/// The tag registry builder's purpose is to serve as a stepping stone to construct the full tag registry.
/// Once all datapacks are loaded, the builder resolves all tag "symlinks".
/// An example of this behaviour is the tag `#minecraft:fences`, which includes `minecraft:nether_brick_fence` and `#minecraft:wooden_fences`.
#[derive(Debug, Default)]
pub struct TagRegistryBuilder {
    block_map: AHashMap<NamespacedId, AHashSet<SmartString<Compact>>>,
    entity_map: AHashMap<NamespacedId, AHashSet<SmartString<Compact>>>,
    fluid_map: AHashMap<NamespacedId, AHashSet<SmartString<Compact>>>,
    item_map: AHashMap<NamespacedId, AHashSet<SmartString<Compact>>>,
}

impl TagRegistryBuilder {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    pub fn add_tags_from_dir(
        &mut self,
        dir: &Path,
        namespace: &str,
    ) -> Result<(), crate::TagLoadError> {
        assert!(dir.is_dir());
        let blocks = dir.join("blocks");
        let entity_types = dir.join("entity_types");
        let fluids = dir.join("fluids");
        let items = dir.join("items");
        if blocks.exists() {
            Self::fill_map(&blocks, &mut self.block_map, namespace)?;
        }
        if entity_types.exists() {
            Self::fill_map(&entity_types, &mut self.entity_map, namespace)?;
        }
        if fluids.exists() {
            Self::fill_map(&fluids, &mut self.fluid_map, namespace)?;
        }
        if items.exists() {
            Self::fill_map(&items, &mut self.item_map, namespace)?;
        }
        Ok(())
    }
    pub fn from_dir(dir: &Path, namespace: &str) -> Result<Self, crate::TagLoadError> {
        let mut this = Self::new();
        this.add_tags_from_dir(dir, namespace)?;
        Ok(this)
    }
    fn fill_map(
        dir: &Path,
        map: &mut AHashMap<NamespacedId, AHashSet<SmartString<Compact>>>,
        namespace: &str,
    ) -> Result<(), crate::TagLoadError> {
        for entry in WalkDir::new(dir).into_iter() {
            let entry = entry?;
            let entry = entry.path();
            if !entry.is_file() {
                continue;
            }
            log::trace!("{}", entry.to_string_lossy());
            let path_to_file = entry.parent().unwrap();
            let file_name = entry.file_stem().unwrap();
            let tag_name = std::borrow::Cow::Owned(
                path_to_file
                    .strip_prefix(dir)
                    .unwrap()
                    .to_string_lossy()
                    .replace("\\", "/"),
            ) + file_name.to_string_lossy();
            let namespaced = NamespacedId::from_parts(namespace, &tag_name[..])?;
            if !map.contains_key(&namespaced) {
                map.insert(namespaced.clone(), Default::default());
            }
            Self::fill_set(entry, map.get_mut(&namespaced).unwrap())?;
        }
        Ok(())
    }
    fn fill_set(
        file: &Path,
        set: &mut AHashSet<SmartString<Compact>>,
    ) -> Result<(), crate::TagLoadError> {
        assert!(file.is_file());
        let mut s = String::new();
        File::open(file).unwrap().read_to_string(&mut s).unwrap();
        let file: TagFile = serde_json::from_str(&s[..])?;
        if file.replace {
            set.clear();
        }
        for entry in file.values {
            set.insert(SmartString::from(entry));
        }
        Ok(())
    }
    fn parse(
        source: &AHashMap<NamespacedId, AHashSet<SmartString<Compact>>>,
        target: &mut AHashMap<NamespacedId, AHashSet<NamespacedId>>,
    ) -> Result<(), crate::TagLoadError> {
        let mut stack = VecDeque::new();
        for tag in source.keys().cloned() {
            if target.contains_key(&tag) {
                continue;
            }
            Self::parse_rec(tag, &mut stack, source, target)?;
        }
        Ok(())
    }
    fn parse_rec(
        tag: NamespacedId,
        stack: &mut VecDeque<NamespacedId>,
        source: &AHashMap<NamespacedId, AHashSet<SmartString<Compact>>>,
        target: &mut AHashMap<NamespacedId, AHashSet<NamespacedId>>,
    ) -> Result<(), crate::TagLoadError> {
        if stack.contains(&tag) {
            return Err(LoopError(stack.iter().cloned().collect()).into());
        }
        let set = match source.get(&tag) {
            Some(s) => s,
            None => {
                return Err(crate::TagLoadError::InvalidLink(
                    stack.pop_back().unwrap(),
                    tag,
                ))
            }
        };
        assert!(target.insert(tag.clone(), Default::default()).is_none());
        // Parse all child tags
        for child in set.iter().filter_map(|s| s.strip_prefix('#')) {
            let child = NamespacedId::from_str(child)?;
            if !target.contains_key(&child) {
                // Skip already parsed tags
                stack.push_back(tag.clone());
                Self::parse_rec(child.clone(), stack, source, target)?;
            }
            for element in target.get(&child).unwrap().clone() {
                // Insert child entry
                target.get_mut(&tag).unwrap().insert(element);
            }
        }
        let target_entry = target.get_mut(&tag).unwrap();

        for i in source
            .get(&tag)
            .unwrap()
            .iter()
            .filter(|e| !e.starts_with('#'))
        {
            // Insert all non-tag entries
            target_entry.insert(NamespacedId::from_str(i)?);
        }
        // This tag is now parsed.
        stack.pop_back();

        Ok(())
    }
    pub fn build(self) -> Result<TagRegistry, crate::TagLoadError> {
        let mut res = TagRegistry::new();
        Self::parse(&self.block_map, &mut res.block_map)?;
        Self::parse(&self.entity_map, &mut res.entity_map)?;
        Self::parse(&self.fluid_map, &mut res.fluid_map)?;
        Self::parse(&self.item_map, &mut res.item_map)?;
        Ok(res)
    }
}
/// A registry for keeping track of tags.
#[derive(Debug, Default)]
pub struct TagRegistry {
    block_map: AHashMap<NamespacedId, AHashSet<NamespacedId>>,
    entity_map: AHashMap<NamespacedId, AHashSet<NamespacedId>>,
    fluid_map: AHashMap<NamespacedId, AHashSet<NamespacedId>>,
    item_map: AHashMap<NamespacedId, AHashSet<NamespacedId>>,
    cached_packet: RefCell<Option<Box<AllTags>>>,
}
impl TagRegistry {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    pub fn check_block_tag<T>(&self, block: BlockKind, tag: &T) -> bool
    where
        T: Into<NamespacedId> + Clone,
    {
        self.block_map
            .get(&tag.clone().into())
            .map(|set| set.get(&NamespacedId::from_str(block.name()).unwrap()))
            .flatten()
            .is_some()
    }
    pub fn check_entity_tag<T>(&self, entity: EntityKind, tag: &T) -> bool
    where
        T: Into<NamespacedId> + Clone,
    {
        self.entity_map
            .get(&tag.clone().into())
            .map(|set| set.get(&NamespacedId::from_str(entity.name()).unwrap()))
            .flatten()
            .is_some()
    }
    pub fn check_fluid_tag<T>(&self, fluid: BlockKind, tag: &T) -> bool
    where
        T: Into<NamespacedId> + Clone,
    {
        self.fluid_map
            .get(&tag.clone().into())
            .map(|set| set.get(&NamespacedId::from_str(fluid.name()).unwrap()))
            .flatten()
            .is_some()
    }
    pub fn check_item_tag<T>(&self, item: Item, tag: &T) -> bool
    where
        T: Into<NamespacedId> + Clone,
    {
        self.item_map
            .get(&tag.clone().into())
            .map(|set| set.get(&NamespacedId::from_str(item.name()).unwrap()))
            .flatten()
            .is_some()
    }
    pub fn check_for_any_tag<T>(&self, thing: impl Borrow<str>, tag: &T) -> bool
    where
        T: Into<NamespacedId> + Clone,
    {
        let thing = NamespacedId::from_str(thing.borrow()).unwrap();
        let tag = tag.clone().into();
        self.block_map.get(&tag).map(|s| s.get(&thing)).is_some()
            | self.entity_map.get(&tag).map(|s| s.get(&thing)).is_some()
            | self.fluid_map.get(&tag).map(|s| s.get(&thing)).is_some()
            | self.item_map.get(&tag).map(|s| s.get(&thing)).is_some()
    }
    /// Provides an `AllTags` packet for sending to the client. This tag is cached to save some performance.
    pub fn all_tags(&self) -> AllTags {
        let mut inner = self.cached_packet.borrow_mut();
        if inner.is_some() {
            inner.as_ref().unwrap().as_ref().to_owned()
        } else {
            let tags = self.build_tags_packet();
            *inner = Some(Box::new(tags.clone()));
            tags
        }
    }
    fn build_tags_packet(&self) -> AllTags {
        let mut block_tags = vec![];
        let mut entity_tags = vec![];
        let mut fluid_tags = vec![];
        let mut item_tags = vec![];
        for (tag_name, block_names) in &self.block_map {
            block_tags.push(Tag {
                name: tag_name.to_string(),
                entries: block_names
                    .iter()
                    .map(|e| VarInt(generated::BlockKind::from_name(e.name()).unwrap().id() as i32))
                    .collect(),
            });
        }
        for (tag_name, entity_names) in &self.entity_map {
            entity_tags.push(Tag {
                name: tag_name.to_string(),
                entries: entity_names
                    .iter()
                    .map(
                        |e| VarInt(generated::EntityKind::from_name(e.name()).unwrap().id() as i32),
                    )
                    .collect(),
            });
        }
        for (tag_name, fluid_names) in &self.fluid_map {
            let mut entries = vec![];
            for entry in fluid_names {
                let block = match BlockId::from_identifier(&entry.to_string()) {
                    Some(s) => s,
                    None => BlockId::from_identifier(&entry.to_string().replace("flowing_", ""))
                        .unwrap()
                        .with_water_level(1),
                };
                entries.push(VarInt(block.vanilla_fluid_id().unwrap() as i32));
            }
            fluid_tags.push(Tag {
                name: tag_name.to_string(),
                entries,
            });
        }
        for (tag_name, item_names) in &self.item_map {
            item_tags.push(Tag {
                name: tag_name.to_string(),
                entries: item_names
                    .iter()
                    .map(|e| VarInt(generated::Item::from_name(e.name()).unwrap().id() as i32))
                    .collect(),
            });
        }
        AllTags {
            block_tags,
            item_tags,
            fluid_tags,
            entity_tags,
        }
    }
    fn display_helper(
        map: &AHashMap<NamespacedId, AHashSet<NamespacedId>>,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let mut m = map.iter().collect::<Vec<_>>();
        m.sort_by(|a, b| a.0.cmp(b.0));
        for (a, b) in m {
            writeln!(f, "{}: ", a)?;
            let mut n = b.iter().collect::<Vec<_>>();
            n.sort();
            for c in n {
                writeln!(f, "    {}", c)?;
            }
        }
        Ok(())
    }
}
impl Display for TagRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Self::display_helper(&self.block_map, f)?;
        Self::display_helper(&self.entity_map, f)?;
        Self::display_helper(&self.fluid_map, f)?;
        Self::display_helper(&self.item_map, f)?;

        Ok(())
    }
}
#[derive(Deserialize)]
struct TagFile {
    pub replace: bool,
    pub values: Vec<String>,
}
#[derive(Debug, Error)]
pub struct LoopError(Vec<NamespacedId>);
impl Display for LoopError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for entry in &self.0 {
            writeln!(f, "{}", entry)?;
        }
        Ok(())
    }
}
impl From<VanillaBlockTags> for crate::NamespacedId {
    fn from(tag: VanillaBlockTags) -> Self {
        NamespacedId::from_str(tag.name()).unwrap()
    }
}
impl From<VanillaEntityTypes> for crate::NamespacedId {
    fn from(tag: VanillaEntityTypes) -> Self {
        NamespacedId::from_str(tag.name()).unwrap()
    }
}
impl From<VanillaFluidTags> for crate::NamespacedId {
    fn from(tag: VanillaFluidTags) -> Self {
        NamespacedId::from_str(tag.name()).unwrap()
    }
}
impl From<VanillaItemTags> for crate::NamespacedId {
    fn from(tag: VanillaItemTags) -> Self {
        NamespacedId::from_str(tag.name()).unwrap()
    }
}
