use std::{
    borrow::Borrow, collections::VecDeque, fmt::Display, fs::File, io::Read, path::Path,
    str::FromStr,
};

use crate::NamespacedId;
use ahash::{AHashMap, AHashSet};
use feather_generated::{BlockKind, EntityKind, Item};
use serde::Deserialize;
use smartstring::{Compact, SmartString};
use thiserror::Error;
use walkdir::WalkDir;
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
    pub fn add_tags_from_dir(&mut self, dir: &Path, namespace: &str) -> Result<(), crate::Error> {
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
    pub fn from_dir(dir: &Path, namespace: &str) -> Result<Self, crate::Error> {
        let mut this = Self::new();
        this.add_tags_from_dir(dir, namespace)?;
        Ok(this)
    }
    fn fill_map(
        dir: &Path,
        map: &mut AHashMap<NamespacedId, AHashSet<SmartString<Compact>>>,
        namespace: &str,
    ) -> Result<(), crate::Error> {
        for entry in WalkDir::new(dir).into_iter() {
            let entry = entry?;
            let entry = entry.path();
            if !entry.is_file() {
                continue;
            }
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
    fn fill_set(file: &Path, set: &mut AHashSet<SmartString<Compact>>) -> Result<(), crate::Error> {
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
    ) -> Result<(), crate::Error> {
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
    ) -> Result<(), crate::Error> {
        if stack.contains(&tag) {
            return Err(LoopError(stack.iter().cloned().collect()).into());
        }
        let set = match source.get(&tag) {
            Some(s) => s,
            None => return Err(crate::Error::InvalidLink(stack.pop_back().unwrap(), tag)),
        };
        assert!(target.insert(tag.clone(), Default::default()).is_none());
        // Parse all child tags
        for child in set.iter().filter_map(|s| s.strip_prefix('#')) {
            println!("{}", child);
            let child = NamespacedId::from_str(child)?;
            if !target.contains_key(&child) {
                // Skip already parsed tags
                stack.push_back(tag.clone());
                Self::parse_rec(child.clone(), stack, source, target)?;
            }
            for element in target.get(&child).unwrap().clone() {
                // Insert freshly child entry
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
    pub fn build(self) -> Result<TagRegistry, crate::Error> {
        let mut res = TagRegistry::new();
        Self::parse(&self.block_map, &mut res.block_map)?;
        Self::parse(&self.entity_map, &mut res.entity_map)?;
        Self::parse(&self.fluid_map, &mut res.fluid_map)?;
        Self::parse(&self.item_map, &mut res.item_map)?;
        Ok(res)
    }
}
#[derive(Debug, Default)]
pub struct TagRegistry {
    pub block_map: AHashMap<NamespacedId, AHashSet<NamespacedId>>,
    pub entity_map: AHashMap<NamespacedId, AHashSet<NamespacedId>>,
    pub fluid_map: AHashMap<NamespacedId, AHashSet<NamespacedId>>,
    pub item_map: AHashMap<NamespacedId, AHashSet<NamespacedId>>,
}
impl TagRegistry {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    pub fn check_block_tag(&self, block: BlockKind, tag: impl Into<NamespacedId>) -> bool {
        self.block_map
            .get(&tag.into())
            .map(|set| set.get(&NamespacedId::from_str(block.name()).unwrap()))
            .is_some()
    }
    pub fn check_entity_tag(&self, entity: EntityKind, tag: impl Into<NamespacedId>) -> bool {
        self.entity_map
            .get(&tag.into())
            .map(|set| set.get(&NamespacedId::from_str(entity.name()).unwrap()))
            .is_some()
    }
    pub fn check_fluid_tag(&self, fluid: impl Borrow<str>, tag: impl Into<NamespacedId>) -> bool {
        self.fluid_map
            .get(&tag.into())
            .map(|set| set.get(&NamespacedId::from_str(fluid.borrow()).unwrap()))
            .is_some()
    }
    pub fn check_item_tag(&self, item: Item, tag: impl Into<NamespacedId>) -> bool {
        self.item_map
            .get(&tag.into())
            .map(|set| set.get(&NamespacedId::from_str(item.name()).unwrap()))
            .is_some()
    }
    pub fn check_for_any_tag(&self, thing: impl Borrow<str>, tag: impl Into<NamespacedId>) -> bool {
        let thing = NamespacedId::from_str(thing.borrow()).unwrap();
        let tag = tag.into();
        self.block_map.get(&tag).map(|s| s.get(&thing)).is_some()
            | self.entity_map.get(&tag).map(|s| s.get(&thing)).is_some()
            | self.fluid_map.get(&tag).map(|s| s.get(&thing)).is_some()
            | self.item_map.get(&tag).map(|s| s.get(&thing)).is_some()
    }
    fn k(
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
        Self::k(&self.block_map, f)?;
        Self::k(&self.entity_map, f)?;
        Self::k(&self.fluid_map, f)?;
        Self::k(&self.item_map, f)?;

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
