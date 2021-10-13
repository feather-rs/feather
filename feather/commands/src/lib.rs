//! Implements the Feather command dispatching framework
//! and vanilla commands not defined by plugins.

use std::ops::{Deref, DerefMut};

use commands::arguments::{EntitySelector, EntitySelectorPredicate, EntitySelectorSorting};
pub use commands::dispatcher::CommandDispatcher;
use commands::dispatcher::CommandOutput;
use log::{debug, info};
use uuid::Uuid;

use common::Game;
use ecs::{Entity, EntityRef};
use libcraft_core::{EntityKind, Position};
use libcraft_text::{Text, TextComponentBuilder};
use quill_common::components::{ChatBox, CustomName, Gamemode, Name};

mod impls;

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
    /// The game state. We don't want CommandDispatcher to have a lifetime, so
    /// raw pointers are here to elide the lifetime.
    /// Since CommandCtx is not Send, it should be sound
    game: *mut Game,
}

impl Deref for CommandCtx {
    type Target = Game;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.game }
    }
}

impl DerefMut for CommandCtx {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.game }
    }
}

impl CommandCtx {
    pub fn new(game: &mut Game, sender: Entity) -> CommandCtx {
        CommandCtx {
            game: game as *mut Game,
            sender,
        }
    }
    pub fn send_message(&self, message: impl Into<Text>) {
        self.ecs
            .get_mut::<ChatBox>(self.sender)
            .unwrap()
            .send_system(message)
    }
    /// Find entities by selector and report an error if no entities/players were found
    pub fn find_non_empty_entities_by_selector(
        &self,
        selector: &EntitySelector,
        players_only: bool,
    ) -> Option<Vec<Entity>> {
        let entities = self.find_entities_by_selector(selector);
        if entities.is_empty() {
            self.send_message(match (selector, players_only) {
                (EntitySelector::Name(_), true) => Text::translate("argument.player.unknown").red(),
                (_, true) => Text::translate("argument.entity.notfound.player").red(),
                (_, false) => Text::translate("argument.entity.notfound.entity").red(),
            });
            None
        } else {
            Some(entities)
        }
    }
    pub fn find_entities_by_selector(&self, selector: &EntitySelector) -> Vec<Entity> {
        match selector {
            EntitySelector::Selector(selector) => {
                let sender_position = self
                    .ecs
                    .get::<Position>(self.sender)
                    .map(|pos| *pos)
                    .unwrap_or_default();
                let mut sort = EntitySelectorSorting::Arbitrary;
                let mut entities = if selector.contains(&EntitySelectorPredicate::Sender) {
                    vec![self.sender]
                } else {
                    self.ecs
                        .query::<&EntityKind>()
                        .iter()
                        .map(|(e, _)| e)
                        .collect()
                }
                .into_iter()
                .map(|entity_id| (entity_id, self.ecs.entity(entity_id).unwrap()))
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
                .ecs
                .query::<&Name>()
                .iter()
                .filter(|(_, n)| &***n == name)
                .map(|(e, _)| e)
                .collect(),
            EntitySelector::Uuid(uuid) => self
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
    log: bool,
) -> Option<CommandOutput> {
    let ctx = CommandCtx::new(game, sender);

    if dispatcher.find_command(command).is_none() {
        if let Ok(mut chat) = ctx.ecs.get_mut::<ChatBox>(sender) {
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
        debug!("Unknown command: /{}", command);
        None
    } else {
        if log {
            info!(
                "{} issued server command: /{}",
                ctx.ecs
                    .get::<Name>(sender)
                    .map(|name| (***name).to_string())
                    .unwrap_or("Console".to_owned()),
                command
            );
        }
        let result = dispatcher.execute_command(command, ctx);
        match &result {
            Some(Ok(n)) => debug!("Command result: {:?}", n),
            Some(Err(e)) => debug!("Command error: {}", e),
            None => debug!("Command not found"),
        }
        result
    }
}

pub fn tab_complete(
    dispatcher: &CommandDispatcher<CommandCtx>,
    game: &mut Game,
    sender: Entity,
    prompt: &str,
) -> Vec<(String, Option<String>)> {
    dispatcher
        .tab_complete(prompt, CommandCtx::new(game, sender))
        .unwrap_or_default()
}
