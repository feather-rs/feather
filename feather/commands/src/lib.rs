//! Implements the Feather command dispatching framework
//! and vanilla commands not defined by plugins.

use std::ops::{Deref, DerefMut};

use commands::arguments::{EntitySelector, EntitySelectorPredicate, EntitySelectorSorting};
pub use commands::dispatcher::CommandDispatcher;
use uuid::Uuid;

use common::Game;
use ecs::{Entity, EntityRef};
use libcraft_core::{EntityKind, Position};
use libcraft_text::{Text, TextComponentBuilder};
use quill_common::components::{ChatBox, CustomName, Gamemode, Name};

mod impls;

/// Dumb workaround for a certain lifetime issue.
///
/// `CommandCtx` stores references to `Game`, and it
/// is used as the `C` parameter for `CommandDispatcher`,
/// This combination of lifetimes and storage in structs
/// prevents a lifetime-based `CommandCtx` from being stored
/// in `CommandState` without adding a lifetime parameter to `CommandState`.
///
/// Since `CommandCtx` is never actually _stored_ in `CommandState` (it's
/// only passed into a function), we can (hopefully) soundly erase
/// the lifetime parameters. FIXME: if someone has a better solution,
/// a PR is welcome :)
pub struct LifetimelessMut<T>(*mut T);

impl<T> Deref for LifetimelessMut<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &mut *self.0 }
    }
}

impl<T> DerefMut for LifetimelessMut<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.0 }
    }
}

/// Context passed into a command. This value can be used
/// for access to game and entity data, such as components.
pub struct CommandCtx {
    /// The entity which triggered the command.
    ///
    /// _Not necessarily a player_. If the command was executed
    /// from the server console, then this will be the "server entity"
    /// associated with the console. You may check if an entity is a player
    /// by checking if it has the `Player` component. Similarly, you
    /// may check if an entity is the server console through the `Console` component.
    ///
    /// Note that players and the console are not the only possible command senders,
    /// and command implementations should account for this.
    pub sender: Entity,
    /// The game state.
    pub game: LifetimelessMut<Game>,
}

impl CommandCtx {
    pub fn send_message(&self, message: impl Into<Text>) {
        self.game
            .ecs
            .get_mut::<ChatBox>(self.sender)
            .unwrap()
            .send_system(message)
    }
    pub fn find_entities_by_selector(&self, selector: &EntitySelector) -> Vec<Entity> {
        match selector {
            EntitySelector::Selector(selector) => {
                let sender_position = self
                    .game
                    .ecs
                    .get::<Position>(self.sender)
                    .map(|pos| *pos)
                    .unwrap_or_default();
                let mut sort = EntitySelectorSorting::Arbitrary;
                let mut entities = if selector.contains(&EntitySelectorPredicate::Sender) {
                    vec![self.sender]
                } else {
                    self.game
                        .ecs
                        .query::<&EntityKind>()
                        .iter()
                        .map(|(e, _)| e)
                        .collect()
                }
                .into_iter()
                .map(|entity_id| (entity_id, self.game.ecs.entity(entity_id).unwrap()))
                .filter(|(_, entity)| {
                    let pos = entity.get::<Position>().unwrap();

                    let mut origin = sender_position;
                    let mut dpos = [None; 3];
                    let mut distance = None;

                    for filter in selector {
                        if !match filter {
                            EntitySelectorPredicate::Type(t) => entity
                                .get::<EntityKind>()
                                .map(|k| k.name() == t.name())
                                .unwrap_or(false),
                            EntitySelectorPredicate::Advancements(_) => todo!(),
                            EntitySelectorPredicate::Distance(d) => {
                                distance = Some(d.clone());
                                true
                            }
                            EntitySelectorPredicate::Dx(x) => {
                                dpos[0] = Some(*x);
                                true
                            }
                            EntitySelectorPredicate::Dy(y) => {
                                dpos[1] = Some(*y);
                                true
                            }
                            EntitySelectorPredicate::Dz(z) => {
                                dpos[2] = Some(*z);
                                true
                            }
                            EntitySelectorPredicate::Gamemode(mode) => entity
                                .get::<Gamemode>()
                                .map(|m| mode.0 == (m.to_string() == mode.1.to_string()))
                                .unwrap_or(false),
                            EntitySelectorPredicate::Level(_) => todo!(),
                            EntitySelectorPredicate::Limit(_) => true,
                            EntitySelectorPredicate::Name(name) => entity
                                .get::<Name>()
                                .map(|name| (***name).to_string())
                                .or_else(|_| {
                                    entity.get::<CustomName>().map(|name| (***name).to_string())
                                })
                                .map(|n| name.0 == (n == name.1))
                                .unwrap_or(false),
                            EntitySelectorPredicate::Predicate(_) => todo!(),
                            EntitySelectorPredicate::Scores(_) => todo!(),
                            EntitySelectorPredicate::Sort(s) => {
                                sort = s.clone();
                                true
                            }
                            EntitySelectorPredicate::Tag(_) => todo!(),
                            EntitySelectorPredicate::Team(_) => todo!(),
                            EntitySelectorPredicate::X(x) => {
                                origin.x = *x;
                                true
                            }
                            EntitySelectorPredicate::Y(y) => {
                                origin.y = *y;
                                true
                            }
                            EntitySelectorPredicate::Z(z) => {
                                origin.z = *z;
                                true
                            }
                            EntitySelectorPredicate::XRotation(x_rot) => x_rot.contains(&pos.pitch),
                            EntitySelectorPredicate::YRotation(y_rot) => y_rot.contains(&pos.yaw),
                            EntitySelectorPredicate::Sender => true,
                        } {
                            return false;
                        }
                    }
                    // TODO use Aabb when it's a component
                    if let Some(dx) = dpos[0] {
                        let range = if dx > 0.0 {
                            origin.x..origin.x + dx
                        } else {
                            origin.x + dx..origin.x
                        };
                        if !range.contains(&pos.x) {
                            return false;
                        }
                    }
                    if let Some(dy) = dpos[1] {
                        let range = if dy > 0.0 {
                            origin.y..origin.y + dy
                        } else {
                            origin.y + dy..origin.y
                        };
                        if !range.contains(&pos.y) {
                            return false;
                        }
                    }
                    if let Some(dz) = dpos[2] {
                        let range = if dz > 0.0 {
                            origin.z..origin.z + dz
                        } else {
                            origin.z + dz..origin.z
                        };
                        if !range.contains(&pos.z) {
                            return false;
                        }
                    }
                    if dpos.iter().all(Option::is_none)
                        && distance.is_some()
                        && !distance.unwrap().contains(&pos.distance_to(origin))
                    {
                        return false;
                    }
                    true
                })
                .collect::<Vec<_>>();
                entities.sort_by(|(entity_id, entity), (entity_id2, entity2)| {
                    let by = |entity_id: &Entity, entity: &EntityRef| match sort {
                        EntitySelectorSorting::Nearest => entity
                            .get::<Position>()
                            .unwrap()
                            .distance_to(sender_position),
                        EntitySelectorSorting::Furthest => -entity
                            .get::<Position>()
                            .unwrap()
                            .distance_to(sender_position),
                        EntitySelectorSorting::Random => rand::random(),
                        EntitySelectorSorting::Arbitrary => entity_id.id() as f64,
                    };
                    by(entity_id, entity)
                        .partial_cmp(&by(entity_id2, entity2))
                        .unwrap()
                });
                entities
                    .into_iter()
                    .map(|(entity_id, _)| entity_id)
                    .collect()
            }
            EntitySelector::Name(name) => self
                .game
                .ecs
                .query::<&Name>()
                .iter()
                .filter(|(_, n)| &***n == name)
                .map(|(e, _)| e)
                .collect(),
            EntitySelector::Uuid(uuid) => self
                .game
                .ecs
                .query::<&Uuid>()
                .iter()
                .filter(|(_, id)| *id == uuid)
                .map(|(e, _)| e)
                .collect(),
        }
    }
}

pub fn register_vanilla_commands(dispatcher: &mut CommandDispatcher<CommandCtx>) {
    impls::register_all(dispatcher)
}

pub fn dispatch_command(
    dispatcher: &CommandDispatcher<CommandCtx>,
    game: &mut Game,
    sender: Entity,
    command: &str,
) -> bool {
    let ctx = CommandCtx {
        game: LifetimelessMut(game),
        sender,
    };

    if dispatcher.find_command(command).is_none() {
        if let Ok(mut chat) = ctx.game.ecs.get_mut::<ChatBox>(sender) {
            chat.send_system(
                Text::translate("command.unknown.command")
                    .push_extra(
                        Text::Array(vec![
                            Text::of("\n/").gray(),
                            Text::of(command.to_string()).underlined(),
                            Text::translate("command.context.here").italic(),
                        ])
                        .on_click_suggest_command(format!("/{}", command)),
                    )
                    .red(),
            );
        }
        false
    } else {
        dispatcher.execute_command(command, ctx)
    }
}

pub fn tab_complete(
    dispatcher: &CommandDispatcher<CommandCtx>,
    game: &mut Game,
    sender: Entity,
    prompt: &str,
) -> Vec<(String, Option<String>)> {
    dispatcher
        .tab_complete(
            prompt,
            CommandCtx {
                sender,
                game: LifetimelessMut(game),
            },
        )
        .unwrap_or_default()
}
