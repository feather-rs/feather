use std::collections::HashMap;

use commands::arguments::{EntitySelector, EntitySelectorPredicate, EntitySelectorSorting};
use commands::dispatcher::TabCompletion;
use uuid::Uuid;

use common::Game;
use ecs::{Ecs, Entity, EntityRef};
use libcraft_core::{EntityKind, Position};
use libcraft_text::Text;
use quill_common::components::{CustomName, Gamemode, Name};

pub fn get_entity_names(sender: Entity, game: &Game, selector: &EntitySelector) -> Vec<String> {
    find_entities_by_selector(sender, game, selector)
        .iter()
        .map(|&e| get_entity_name(e, &game.ecs))
        .collect()
}

pub fn get_entity_name(e: Entity, ecs: &Ecs) -> String {
    ecs.get::<Name>(e)
        .map(|name| (***name).to_string())
        .or_else(|_| ecs.get::<CustomName>(e).map(|name| (***name).to_string()))
        .or_else(|_| ecs.get::<EntityKind>(e).map(|k| k.display_name().into()))
        .unwrap()
}

pub fn find_entities_by_selector(
    sender: Entity,
    game: &Game,
    selector: &EntitySelector,
) -> Vec<Entity> {
    match selector {
        EntitySelector::Selector(selector) => {
            let sender_position = game
                .ecs
                .get::<Position>(sender)
                .map(|pos| *pos)
                .unwrap_or_default();
            let mut sort = EntitySelectorSorting::Arbitrary;
            let mut entities = if selector.contains(&EntitySelectorPredicate::Sender) {
                vec![sender]
            } else {
                game.ecs
                    .query::<&EntityKind>()
                    .iter()
                    .map(|(e, _)| e)
                    .collect()
            }
            .into_iter()
            .map(|entity_id| (entity_id, game.ecs.entity(entity_id).unwrap()))
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
                        EntitySelectorPredicate::Name(name) => {
                            macro_rules! name {
                                () => {
                                    |name| (***name).to_string()
                                };
                            }
                            entity
                                .get::<Name>()
                                .map(name!())
                                .or_else(|_| entity.get::<CustomName>().map(name!()))
                                .map(|n| name.0 == (n == name.1))
                                .unwrap_or(false)
                        }
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
                        EntitySelectorPredicate::Sender => true, // already checked for this
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
        EntitySelector::Name(name) => game
            .ecs
            .query::<&Name>()
            .iter()
            .filter(|(_, n)| &***n == name)
            .map(|(e, _)| e)
            .collect(),
        EntitySelector::Uuid(uuid) => game
            .ecs
            .query::<&Uuid>()
            .iter()
            .filter(|(_, id)| *id == uuid)
            .map(|(e, _)| e)
            .collect(),
    }
}

pub fn fixed_completion<T, U: AsRef<str>>(
    completions: Vec<U>,
) -> impl Fn(&str, &mut T) -> TabCompletion<Text> + 'static {
    let completions = completions
        .into_iter()
        .map(|c| c.as_ref().to_string())
        .collect::<Vec<_>>();
    move |text, _ctx| {
        (
            0,
            text.len(),
            completions
                .iter()
                .filter(|c| c.starts_with(text))
                .map(|c| (c.to_owned(), None))
                .collect(),
        )
    }
}

pub fn fixed_completion_with_tooltip<T, U: AsRef<str>>(
    completions: HashMap<U, Option<String>>,
) -> impl Fn(&str, &mut T) -> TabCompletion<Text> + 'static {
    let completions = completions
        .into_iter()
        .map(|(c, t)| (c.as_ref().to_string(), t))
        .collect::<Vec<_>>();
    move |text, _ctx| {
        (
            0,
            text.len(),
            completions
                .iter()
                .filter(|(c, _)| c.starts_with(text))
                .map(|(c, t)| (c.to_owned(), t.as_ref().map(|t| t.clone().into())))
                .collect(),
        )
    }
}
