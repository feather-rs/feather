use std::collections::HashMap;
use std::convert::Infallible;
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use std::num::ParseFloatError;
use std::ops::{Bound, RangeBounds, RangeFrom, RangeInclusive, RangeToInclusive};
use std::str::FromStr;

use lieutenant::{ArgumentKind, Input};
use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use smallvec::SmallVec;
use thiserror::Error;

use base::position;
use base::{Gamemode, Item, Position};
use ecs::Entity;
use quill_common::components::Name;
use quill_common::entities::Player;

use crate::entity_selector_format::{from_str, SelectorError};
use crate::CommandCtx;

#[derive(Debug, Error)]
pub enum SelectorParseError {
    #[error("Invalid selector: {0}")]
    InvalidSelector(String),
}

/// Argument kind which supports entity selectors.
pub struct EntitySelector {
    /// Entities selected by the parameter.
    pub requirements: Vec<EntitySelectorPredicate>,
}

impl ArgumentKind<CommandCtx> for EntitySelector {
    type ParseError = SelectorParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        input.advance_until(" ");

        true
    }

    fn parse<'a>(ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let head = input.advance_until(" ");

        // See https://minecraft.gamepedia.com/Commands#Target_selectors
        let requirements = parse_entity_selector(ctx, head)?;

        Ok(EntitySelector { requirements })
    }
}

impl EntitySelector {
    /// Parses the returned entities for use in reporting success messages
    /// Either the name of the entity for one entity, or how many were affected for many entities.
    pub fn entities_to_string(
        &self,
        ctx: &CommandCtx,
        entities: &[Entity],
        add_player: bool,
    ) -> String {
        if entities.is_empty() {
            "no entities".to_string()
        } else if entities.len() == 1 {
            if let Ok(name) = ctx.ecs.get::<Name>(*entities.first().unwrap()) {
                if add_player {
                    if ctx.ecs.get::<Player>(*entities.first().unwrap()).is_ok() {
                        format!("player {}", *name)
                    } else {
                        format!("entity {}", *name)
                    }
                } else {
                    name.to_string()
                }
            } else {
                "Server".to_string()
            }
        } else {
            // TODO: confirm this is correct behaviour for success messages involving many players
            let mut players = true;
            for entity in entities {
                players &= ctx.ecs.get::<Player>(*entity).is_ok();
            }
            if players {
                format!("{} players", entities.len())
            } else {
                format!("{} entities", entities.len())
            }
        }
    }
}

/// A struct for data in @e\[data\]
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum EntitySelectorPredicate {
    /// Filter target selection based on the entity's identifier.
    /// The given entity type must be a valid entity ID or entity type tag
    /// used to identify different types of entities internally.
    /// The namespace can be left out if the ID is within the minecraft namespace.
    /// This is always "player" for @a
    Type(String), // TODO EntityType
    /// Filter target selection based on the entity's advancements.
    /// This naturally filters out all non-player targets.
    /// All advancements are in a single object), with a list of individual
    /// advancement IDs between the braces afterward.
    /// The values are true or false.
    /// For advancements with one criterion, testing for that
    /// criterion always gives the same results as testing for the advancement
    Advancements(HashMap<String, AdvancementPredicate>), // TODO HashMap<AdvancementId, AdvancementPredicate>
    /// Specifies the range of distance.
    /// Float ranges are supported to select a specific region.
    /// Only unsigned values are allowed.
    Distance(WrappedRange<f64>),
    /// Signed difference from `x`
    Dx(f64),
    /// Signed difference from `y`
    Dy(f64),
    /// Signed difference from `z`
    Dz(f64),
    /// Filter target selection by game mode.
    /// This naturally filters out all non-player targets.
    /// Arguments testing for equality cannot be duplicated,
    /// while arguments testing for inequality can.
    Gamemode(BoolPredicate<Gamemode>),
    /// Filter target selection based on the entity's experience levels.
    /// This naturally filters out all non-player targets.
    Level(WrappedRange<u32>),
    /// Limit the number of selectable targets for a target selector.
    /// When using the variables @p and @r, this argument defaults to one.
    /// Applying the limiting argument to them may increase the number of
    /// nearest or random targets selected. When applying this argument to @a or @e,
    /// this argument returns only a limited number of targets
    Limit(u32),
    /// Filter target selection by name. Values are strings, so spaces are allowed only
    /// if quotes are applied. This cannot be a JSON text compound.
    /// Arguments testing for equality cannot be duplicated,
    /// while arguments testing for inequality can.
    Name(BoolPredicate<String>),
    /// Filter target selection based on the entity's NBT data
    //Nbt(BoolPredicate<NbtPredicate>),
    /// Filter target selection by predicates.
    /// The given values must be a valid predicate represented by a resource location
    Predicate(BoolPredicate<String>), // TODO PredicateId
    /// Filter target selection based on their scores in the specified objectives.
    /// All tested objectives are in a single object,
    /// with a list of individual score arguments between braces afterward
    Scores(HashMap<String, WrappedRange<i32>>),
    /// Specify selection priority
    Sort(EntitySelectorSorting),
    /// Filter target selection based on the entity's scoreboard tags
    Tag(BoolPredicate<String>),
    /// Filter target selection based on teams.
    /// Arguments testing for equality cannot be duplicated,
    /// while arguments testing for inequality can.
    Team(BoolPredicate<String>),
    /// Define a position in the world the selector starts at,
    /// for use with the `distance` argument, the volume (`dx`, `dy`, `dz`)
    /// arguments, or the `limit` argument.
    /// Using these arguments alone will not restrict the entities found,
    /// and will only affect the sorting of targets
    X(f64),
    /// Define a position in the world the selector starts at,
    /// for use with the `distance` argument, the volume (`dx`, `dy`, `dz`)
    /// arguments, or the `limit` argument.
    /// Using these arguments alone will not restrict the entities found,
    /// and will only affect the sorting of targets
    Y(f64),
    /// Define a position in the world the selector starts at,
    /// for use with the `distance` argument, the volume (`dx`, `dy`, `dz`)
    /// arguments, or the `limit` argument.
    /// Using these arguments alone will not restrict the entities found,
    /// and will only affect the sorting of targets
    Z(f64),
    /// Filter target selection based on the entity's rotation along the pitch axis,
    /// measured in degrees. Values range from -90 (straight up)
    /// to 0 (at the horizon) to +90 (straight down)
    XRotation(WrappedRange<f32>),
    /// Filter target selection based on the entity's rotation along the yaw axis,
    /// measured clockwise in degrees from due south (or the positive Z direction).
    /// Values vary from -180 (facing due north) to -90 (facing due east)
    /// to 0 (facing due south) to +90 (facing due west) to +180 (facing due north again)
    ZRotation(WrappedRange<f32>),
}

#[derive(Serialize, Deserialize)]
pub struct NbtPredicate; // TODO

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum EntitySelectorSorting {
    /// Sort by increasing distance. (Default for @p)
    Nearest,
    /// Sort by decreasing distance.
    Furthest,
    /// Sort randomly. (Default for @r)
    Random,
    /// Sort by time created. (Default for @e, @a)
    Arbitrary,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum AdvancementPredicate {
    /// Include only players with the specified advancements and values.
    Value(bool),
    /// Include only players with the specified advancement's criteria.
    Criteria(HashMap<String, bool>),
}

#[derive(Debug, PartialEq)]
pub enum WrappedRange<T> {
    RangeFrom(RangeFrom<T>),
    RangeTo(RangeToInclusive<T>),
    Range(RangeInclusive<T>),
}

impl<T> WrappedRange<T> {
    fn start_bound(&self) -> Bound<&T> {
        match self {
            WrappedRange::RangeFrom(range) => range.start_bound(),
            WrappedRange::RangeTo(range) => range.start_bound(),
            WrappedRange::Range(range) => range.start_bound(),
        }
    }
    fn end_bound(&self) -> Bound<&T> {
        match self {
            WrappedRange::RangeFrom(range) => range.end_bound(),
            WrappedRange::RangeTo(range) => range.end_bound(),
            WrappedRange::Range(range) => range.end_bound(),
        }
    }

    fn bound_value(bound: Bound<&T>) -> Option<&T> {
        match bound {
            Bound::Included(x) => Some(x),
            Bound::Excluded(x) => Some(x),
            Bound::Unbounded => None,
        }
    }

    fn contains<U>(&self, item: &U) -> bool
    where
        T: PartialOrd<U>,
        U: ?Sized + PartialOrd<T>,
    {
        (match self.start_bound() {
            Bound::Included(start) => start <= item,
            Bound::Excluded(start) => start < item,
            Bound::Unbounded => true,
        }) && (match self.end_bound() {
            Bound::Included(end) => item <= end,
            Bound::Excluded(end) => item < end,
            Bound::Unbounded => true,
        })
    }
}

impl<T> From<RangeInclusive<T>> for WrappedRange<T> {
    fn from(range: RangeInclusive<T>) -> Self {
        WrappedRange::Range(range)
    }
}

impl<T> From<RangeToInclusive<T>> for WrappedRange<T> {
    fn from(range: RangeToInclusive<T>) -> Self {
        WrappedRange::RangeTo(range)
    }
}

impl<T> From<RangeFrom<T>> for WrappedRange<T> {
    fn from(range: RangeFrom<T>) -> Self {
        WrappedRange::RangeFrom(range)
    }
}

impl<T> Serialize for WrappedRange<T>
where
    T: ToString,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!(
            "{}..{}",
            Self::bound_value(self.start_bound())
                .map(ToString::to_string)
                .unwrap_or_default(),
            Self::bound_value(self.end_bound())
                .map(ToString::to_string)
                .unwrap_or_default()
        ))
    }
}

impl<'de, T> Deserialize<'de> for WrappedRange<T>
where
    T: Deserialize<'de> + FromStr + Debug + PartialOrd,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(RangeVisitor(PhantomData))
    }
}

pub struct RangeVisitor<T>(PhantomData<T>);

impl<'de, T> Visitor<'de> for RangeVisitor<T>
where
    T: FromStr + Debug + PartialOrd,
{
    type Value = WrappedRange<T>;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("a range")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_string(v.to_owned())
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Self::parse(v).map_err(|err| E::custom(format!("Failed to parse a range: {}", err.0)))
    }
}

impl<T> RangeVisitor<T>
where
    T: FromStr + Debug + PartialOrd,
{
    pub fn parse(s: String) -> Result<WrappedRange<T>, SelectorError> {
        let (start, end) = s.split_once("..").unwrap_or((&s, &s));
        let (start, end) = (
            if start.is_empty() {
                None
            } else {
                Some(
                    start
                        .parse()
                        .map_err(|_| SelectorError("Invalid range".to_string()))?,
                )
            },
            if end.is_empty() {
                None
            } else {
                Some(
                    end.parse()
                        .map_err(|_| SelectorError("Invalid range".to_string()))?,
                )
            },
        );
        match (start, end) {
            (None, None) => Err(SelectorError(
                "Double-open ranges (\"..\") are not allowed".to_string(),
            )),
            (Some(start), None) => Ok(WrappedRange::RangeFrom(start..)),
            (None, Some(end)) => Ok(WrappedRange::RangeTo(..=end)),
            (Some(start), Some(end)) => {
                if start > end {
                    Err(SelectorError(format!(
                        "{:?} is greater than {:?}, so {0:?}..{1:?} is invalid",
                        start, end
                    )))
                } else {
                    Ok(WrappedRange::Range(start..=end))
                }
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct BoolPredicate<T>(pub bool, pub T);

impl<T> Serialize for BoolPredicate<T>
where
    T: ToString,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if self.0 {
            serializer.serialize_str(&self.1.to_string())
        } else {
            serializer.serialize_str(&format!("!{}", self.1.to_string()))
        }
    }
}

impl<'de, T> Deserialize<'de> for BoolPredicate<T>
where
    T: Deserialize<'de> + FromStr,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(BoolPredicateVisitor(PhantomData))
    }
}

pub struct BoolPredicateVisitor<T>(PhantomData<T>);

impl<'de, T> Visitor<'de> for BoolPredicateVisitor<T>
where
    T: FromStr,
{
    type Value = BoolPredicate<T>;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("a bool predicate")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_string(v.to_owned())
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Self::parse(v).map_err(|_| E::custom("Failed to parse value"))
    }
}

impl<T> BoolPredicateVisitor<T>
where
    T: FromStr,
{
    pub fn parse(s: String) -> Result<BoolPredicate<T>, <T as FromStr>::Err> {
        Ok(if let Some(stripped) = s.strip_prefix('!') {
            BoolPredicate(false, stripped.parse()?)
        } else {
            BoolPredicate(true, s.parse()?)
        })
    }
}

fn parse_entity_selector(
    ctx: &CommandCtx,
    input: &str,
) -> Result<Vec<EntitySelectorPredicate>, SelectorParseError> {
    let mut requirements = Vec::new();
    match input.split('[').next().unwrap() {
        "@p" => {
            requirements.push(EntitySelectorPredicate::Sort(
                EntitySelectorSorting::Nearest,
            ));
            requirements.push(EntitySelectorPredicate::Limit(1));
            requirements.push(EntitySelectorPredicate::Type("player".to_string()));
        }
        "@r" => {
            requirements.push(EntitySelectorPredicate::Sort(EntitySelectorSorting::Random));
            requirements.push(EntitySelectorPredicate::Limit(1));
            requirements.push(EntitySelectorPredicate::Type("player".to_string()));
        }
        "@a" => {
            requirements.push(EntitySelectorPredicate::Sort(
                EntitySelectorSorting::Arbitrary,
            ));
            requirements.push(EntitySelectorPredicate::Type("player".to_string()));
        }
        "@e" => {
            requirements.push(EntitySelectorPredicate::Sort(
                EntitySelectorSorting::Arbitrary,
            ));
        }
        "@s" => {
            requirements.push(EntitySelectorPredicate::Sort(
                EntitySelectorSorting::Arbitrary,
            ));
            requirements.push(EntitySelectorPredicate::Name(BoolPredicate(
                true,
                (**ctx.ecs.get::<Name>(ctx.sender).unwrap()).to_owned(),
            )));
            requirements.push(EntitySelectorPredicate::Type("player".to_string()));
        }
        player_name => {
            requirements.push(EntitySelectorPredicate::Sort(
                EntitySelectorSorting::Arbitrary,
            ));
            requirements.push(EntitySelectorPredicate::Name(BoolPredicate(
                true,
                player_name.to_owned(),
            )));
            requirements.push(EntitySelectorPredicate::Type("player".to_string()));
        }
    };
    if input.contains('[') {
        requirements.extend(
            from_str::<Vec<EntitySelectorPredicate>>(&input[input.find('[').unwrap()..])
                .map_err(|e| SelectorParseError::InvalidSelector(e.0))?,
        );
    }
    Ok(requirements)
}

pub fn find_selected_entities(
    ctx: &CommandCtx,
    requirements: &[EntitySelectorPredicate],
) -> Result<SmallVec<[Entity; 1]>, SelectorParseError> {
    println!("{:?}", requirements);
    Ok(SmallVec::new()) // TODO
}

#[derive(Debug, Error)]
pub enum CoordinatesParseError {
    #[error("missing coordinate")]
    MissingCoordinate,
    #[error("failed to parse float: {0}")]
    ParseFloat(#[from] ParseFloatError),
}

/// Parses a position (<x> <y> <z>, but also with support for relative
/// positions as per https://minecraft.gamepedia.com/Commands#Tilde_and_caret_notation).
#[derive(Copy, Clone, Debug)]
pub struct Coordinates {
    pub x: Coordinate,
    pub y: Coordinate,
    pub z: Coordinate,
}

impl Coordinates {
    /// Converts these coordinates into a `Position`.
    ///
    /// The input `relative_to` is the position to interpret
    /// as the origin of relative coordinates. For example,
    /// this is the position of the target entity for the `/tp`
    /// command.
    pub fn into_position(self, relative_to: Position) -> Position {
        let direction = relative_to.direction();
        position!(
            Self::coordinate_into_absolute(self.x, relative_to.x, direction.x),
            Self::coordinate_into_absolute(self.y, relative_to.y, direction.y),
            Self::coordinate_into_absolute(self.z, relative_to.z, direction.z),
            relative_to.pitch,
            relative_to.yaw,
        )
    }

    fn coordinate_into_absolute(coord: Coordinate, relative_to: f64, facing_magnitude: f64) -> f64 {
        match coord {
            Coordinate::Absolute(coord) => coord,
            Coordinate::Relative(rel) => relative_to + rel,
            Coordinate::RelativeLook(rel) => relative_to + rel * facing_magnitude,
        }
    }
}

impl From<Position> for Coordinates {
    fn from(pos: Position) -> Self {
        Coordinates {
            x: Coordinate::Absolute(pos.x),
            y: Coordinate::Absolute(pos.y),
            z: Coordinate::Absolute(pos.z),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Coordinate {
    /// Coordinates relative to some position. The origin
    /// is interpreted differently by different commands.
    ///
    /// For example, `/tp` interprets this as the coordinates
    /// relative to the initial position of the target entity.
    /// On the other hand, another command may use
    /// this as the coordinates relative to the sender's
    /// position.
    Relative(f64),
    /// Relative coordinates in the direction the player is looking.
    /// This is similar to `Relative`, but the axes are rotated
    /// to align with the entity's view direction.
    RelativeLook(f64),
    /// Absolute coordinates, in world space.
    Absolute(f64),
}

impl FromStr for Coordinate {
    type Err = CoordinatesParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(first) = s.chars().next() {
            Ok(match first {
                '~' => {
                    let offset = if s.len() > 1 {
                        f64::from_str(&s[1..])?
                    } else {
                        0.0
                    };
                    Coordinate::Relative(offset)
                }
                '^' => {
                    let offset = if s.len() > 1 {
                        f64::from_str(&s[1..])?
                    } else {
                        0.0
                    };
                    Coordinate::RelativeLook(offset)
                }
                _ => Coordinate::Absolute(f64::from_str(s)?),
            })
        } else {
            Err(CoordinatesParseError::MissingCoordinate)
        }
    }
}

impl ArgumentKind<CommandCtx> for Coordinates {
    type ParseError = CoordinatesParseError;

    fn satisfies<'a>(ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        Self::parse(ctx, input).is_ok()
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let x = input.advance_until(" ");
        let y = input.advance_until(" ");
        let z = input.advance_until(" ");

        let x = Coordinate::from_str(x)?;
        let y = Coordinate::from_str(y)?;
        let z = Coordinate::from_str(z)?;

        Ok(Coordinates { x, y, z })
    }
}

#[derive(Debug, Error)]
pub enum GamemodeParseError {
    #[error("invalid gamemode string {0}")]
    InvalidGamemode(String),
}

/// A parsed gamemode string ("survival", "creative", ...)
#[derive(Copy, Clone, Debug)]
pub struct ParsedGamemode(pub Gamemode);

impl ArgumentKind<CommandCtx> for ParsedGamemode {
    type ParseError = GamemodeParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        !input.advance_until(" ").is_empty()
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let s = input.advance_until(" ");

        let gamemode = match s {
            "survival" => Gamemode::Survival,
            "creative" => Gamemode::Creative,
            "spectator" => Gamemode::Spectator,
            "adventure" => Gamemode::Adventure,
            s => return Err(GamemodeParseError::InvalidGamemode(s.to_owned())),
        };

        Ok(ParsedGamemode(gamemode))
    }
}

#[derive(Debug, Error)]
pub enum TextParseError {}

/// A multi-word argument (parses until the end of the command)
#[derive(Clone, Debug)]
pub struct TextArgument(pub String);

impl ArgumentKind<CommandCtx> for TextArgument {
    type ParseError = Infallible;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        !input.advance_to_end().is_empty()
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_to_end();

        Ok(TextArgument(text.to_owned()))
    }
}

impl AsRef<str> for TextArgument {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Error)]
pub enum ItemParseError {
    #[error("Unknown item {0}")]
    ItemDoesNotExist(String),
}

#[derive(Clone, Debug)]
pub struct ItemArgument(pub Item);

impl ArgumentKind<CommandCtx> for ItemArgument {
    type ParseError = ItemParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        !input.advance_until(" ").is_empty()
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        let item = Item::from_name(text);
        match item {
            Some(s) => Ok(ItemArgument(s)),
            None => Err(ItemParseError::ItemDoesNotExist(text.to_owned())),
        }
    }
}

#[derive(Debug, Error)]
pub enum I32ParseError {
    #[error("Invalid integer {0}")]
    Invalid(String),
}

#[derive(Clone, Debug)]
pub struct I32Argument(pub i32);

impl ArgumentKind<CommandCtx> for I32Argument {
    type ParseError = I32ParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        !input.advance_until(" ").is_empty()
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        let number = text.parse::<i32>();
        match number {
            Ok(s) => Ok(I32Argument(s)),
            Err(_) => Err(I32ParseError::Invalid(text.to_owned())),
        }
    }
}

#[derive(Debug, Error)]
pub enum PositiveI32ParseError {
    #[error("Invalid integer {0}")]
    Invalid(String),
    #[error("Integer must not be less than 0, found {0}")]
    Negative(i32),
}

#[derive(Clone, Debug)]
pub struct PositiveI32Argument(pub i32);

impl ArgumentKind<CommandCtx> for PositiveI32Argument {
    type ParseError = PositiveI32ParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        !input.advance_until(" ").is_empty()
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        let number = text.parse::<i32>();
        match number {
            Ok(integer) => {
                if integer >= 0 {
                    Ok(PositiveI32Argument(integer))
                } else {
                    Err(PositiveI32ParseError::Negative(integer))
                }
            }
            Err(_) => Err(PositiveI32ParseError::Invalid(text.to_owned())),
        }
    }
}

#[derive(Debug, Error)]
pub enum TimeQueryInformationError {
    #[error("Unknown Argument {0}")]
    UnknownArgument(String),
}

#[derive(Clone, Debug)]
pub enum TimeQueryInformation {
    DayTime,
    GameTime,
    Day,
}

impl ArgumentKind<CommandCtx> for TimeQueryInformation {
    type ParseError = TimeQueryInformationError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input) -> bool {
        !input.advance_until(" ").is_empty()
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        match text {
            "daytime" => Ok(TimeQueryInformation::DayTime),
            "gametime" => Ok(TimeQueryInformation::GameTime),
            "day" => Ok(TimeQueryInformation::Day),
            s => Err(TimeQueryInformationError::UnknownArgument(s.to_owned())),
        }
    }
}

#[derive(Debug, Error)]
pub enum TimeArgumentError {
    #[error("Invalid integer {0}")]
    Invalid(String),
    #[error("Invalid unit, found {0}")]
    InvalidUnit(char),
}

#[derive(Clone, Debug)]
pub struct TimeArgument(pub u64);

impl ArgumentKind<CommandCtx> for TimeArgument {
    type ParseError = TimeArgumentError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        Self::parse(_ctx, input).is_ok()
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let text = input.advance_until(" ");
        let (value, unit) = {
            let it = text.chars();
            // Get and parse the number part up to the unit character
            let val = it
                .clone()
                // allow . to be able to type decimal
                .take_while(|&c| c == '.' || char::is_numeric(c))
                .collect::<String>()
                .parse::<f32>();
            // Skips the number part and gets the unit character or default to 't'
            let unit = it
                .clone()
                .find(|&c| c != '.' && !char::is_numeric(c))
                .unwrap_or('t');
            (val, unit)
        };
        match value {
            Ok(num) => match unit {
                'd' => Ok(TimeArgument((num * 24_000.0) as u64)),
                's' => Ok(TimeArgument((num * 20.0) as u64)),
                't' => Ok(TimeArgument(num as u64)),
                _ => Err(TimeArgumentError::InvalidUnit(unit)),
            },
            Err(_) => Err(TimeArgumentError::Invalid(text.to_owned())),
        }
    }
}

#[derive(Debug, Error)]
pub enum TimeSpecParseError {
    #[error("invalid time string {0}")]
    InvalidTimeSpec(String),
}

/// A parsed TimeSpec
#[derive(Copy, Clone, Debug)]
pub enum TimeSpec {
    Day,
    Night,
    Noon,
    Midnight,
}

impl ArgumentKind<CommandCtx> for TimeSpec {
    type ParseError = TimeSpecParseError;

    fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {
        !input.advance_until(" ").is_empty()
    }

    fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {
        let s = input.advance_until(" ");

        Ok(match s {
            "day" => TimeSpec::Day,
            "night" => TimeSpec::Night,
            "noon" => TimeSpec::Noon,
            "midnight" => TimeSpec::Midnight,
            s => return Err(TimeSpecParseError::InvalidTimeSpec(s.to_owned())),
        })
    }
}
