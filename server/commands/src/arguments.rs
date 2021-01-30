use crate::CommandCtx;
use feather_core::position;
use feather_core::util::{Gamemode, Position};
use feather_definitions::Item;
use feather_server_types::{Game, Name, NetworkId, Player};
use fecs::{component, Entity, IntoQuery, Read, World};
use lieutenant::{ArgumentKind, Input};
use rand::Rng;
use smallvec::SmallVec;
use std::convert::Infallible;
use std::num::ParseFloatError;
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SelectorParseError {
    #[error("no player with name {0}")]
    PlayerNotFound(String),
}

/// Argument kind which supports entity selectors.
pub struct EntitySelector {
    /// Entities selected by the parameter.
    pub entities: SmallVec<[Entity; 1]>,
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
        let entities = find_selected_entities(ctx, head)?;

        Ok(EntitySelector { entities })
    }
}

impl EntitySelector {
    /// Parses the returned entities for use in reporting success messages
    /// Either the name of the entity for one entity, or how many were affected for many entities.
    pub fn entities_to_string(&self, ctx: &CommandCtx, add_player: bool) -> String {
        if self.entities.is_empty() {
            "no entities".to_string()
        } else if self.entities.len() == 1 {
            if let Some(name) = ctx.world.try_get::<Name>(*self.entities.first().unwrap()) {
                if add_player {
                    if ctx
                        .world
                        .try_get::<Player>(*self.entities.first().unwrap())
                        .is_some()
                    {
                        format!("player {}", name.0)
                    } else {
                        format!("entity {}", name.0)
                    }
                } else {
                    name.0.to_string()
                }
            } else {
                "Server".to_string()
            }
        } else {
            // TODO: confirm this is correct behaviour for success messages involving many players
            let mut players = true;
            for entity in &self.entities {
                players &= ctx.world.try_get::<Player>(*entity).is_some();
            }
            if players {
                format!("{} players", self.entities.len())
            } else {
                format!("{} entities", self.entities.len())
            }
        }
    }
}

fn find_selected_entities(
    ctx: &CommandCtx,
    input: &str,
) -> Result<SmallVec<[Entity; 1]>, SelectorParseError> {
    use smallvec::smallvec;
    Ok(match input {
        "@p" => {
            // Nearest player
            let pos = ctx
                .world
                .try_get::<Position>(ctx.sender)
                .map(|r| *r)
                .unwrap_or(position!(0.0, 0.0, 0.0));

            nearest_player_to(&ctx.world, pos).into_iter().collect()
        }
        "@r" => {
            // Random player
            random_player(&ctx.game, &ctx.world).into_iter().collect()
        }
        "@a" => {
            // Every player
            <Read<Player>>::query()
                .iter_entities(ctx.world.inner())
                .map(|(e, _)| e)
                .collect()
        }
        "@e" => {
            // Every entity
            <Read<NetworkId>>::query()
                .iter_entities(ctx.world.inner())
                .map(|(e, _)| e)
                .collect()
        }
        "@s" => {
            // Command sender, if it was a player
            if ctx.world.has::<Player>(ctx.sender) {
                smallvec![ctx.sender]
            } else {
                SmallVec::new()
            }
        }
        player_name => smallvec![find_player_by_name(&ctx.world, player_name)
            .ok_or_else(|| SelectorParseError::PlayerNotFound(player_name.to_owned()))?],
    })
}

// TODO: eliminate linear searches.
// These search functions are incredibly naive.
fn find_player_by_name(world: &World, name: &str) -> Option<Entity> {
    <Read<Name>>::query()
        .iter_entities(world.inner())
        .find(|(_, n)| n.0 == name)
        .map(|(entity, _name)| entity)
}

fn nearest_player_to(world: &World, pos: Position) -> Option<Entity> {
    <Read<Position>>::query()
        .filter(component::<Player>())
        .iter_entities(world.inner())
        .min_by_key(|(_, p)| pos.distance_squared_to(**p).floor() as u64)
        .map(|(entity, _)| entity)
}

fn random_player(game: &Game, world: &World) -> Option<Entity> {
    let query = <Read<Player>>::query();

    let count = query.iter(world.inner()).count();

    let index = game.rng().gen_range(0, count);

    query
        .iter_entities(world.inner())
        .nth(index)
        .map(|(e, _)| e)
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
        let item = Item::from_identifier(text);
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
