use crate::CommandCtx;
use feather_core::position;
use feather_core::util::{Gamemode, Position};
use feather_server_types::{Game, Name, NetworkId, Player};
use fecs::{component, Entity, IntoQuery, Read, World};
use lieutenant::{ArgumentKind, Input};
use rand::Rng;
use smallvec::SmallVec;
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
