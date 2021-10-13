//! The implementations of various commands.

use anyhow::bail;
use commands::arguments::*;
use commands::create_command::CreateCommand;
use commands::dispatcher::{CommandDispatcher, CommandOutput};
use smallvec::SmallVec;
use uuid::Uuid;

use common::banlist::{BanList, BanReason};
use common::{Game, Window};
use ecs::{Ecs, Entity};
use libcraft_core::EntityKind;
use libcraft_items::{InventorySlot, ItemStack};
use libcraft_text::{Text, TextComponentBuilder};
use quill_common::components::{ChatBox, CustomName, Gamemode, Name, PreviousGamemode, RealIp};
use quill_common::entities::Player;
use quill_common::events::{DisconnectEvent, GamemodeUpdateEvent, InventoryUpdateEvent};

use crate::CommandCtx;

pub fn register_all(dispatcher: &mut CommandDispatcher<CommandCtx>) {
    // /me
    dispatcher
        .create_command("me")
        .unwrap()
        .argument("text", StringArgument::GREEDY_PHRASE, "none")
        .executes(|context: CommandCtx, action| {
            let command_output = Text::translate_with(
                "chat.type.emote",
                vec![get_name(context.sender, &context.ecs), action],
            );
            context
                .ecs
                .query::<&mut ChatBox>()
                .iter()
                .for_each(|(_, chat_box)| chat_box.send_chat(command_output.clone()));
            Ok(1)
        });

    // /gamemode
    dispatcher
        .create_command("gamemode")
        .unwrap()
        .with(|command| gamemode_command(command, "survival", Gamemode::Survival))
        .with(|command| gamemode_command(command, "creative", Gamemode::Creative))
        .with(|command| gamemode_command(command, "adventure", Gamemode::Adventure))
        .with(|command| gamemode_command(command, "spectator", Gamemode::Spectator));
    fn gamemode_command(command: CreateCommand<CommandCtx, ()>, s: &str, gamemode: Gamemode) {
        command
            .subcommand(s)
            .executes(move |mut context: CommandCtx| {
                update_gamemode(context.sender, &mut context.ecs, gamemode)?;
                context.send_message(Text::translate_with(
                    "commands.gamemode.success.self",
                    vec![Text::translate(format!(
                        "gameMode.{}",
                        gamemode.to_string()
                    ))],
                ));
                Ok(1)
            })
            .argument("target", EntityArgument::PLAYERS, "entity")
            .executes(move |mut context: CommandCtx, selector| {
                let targets = context.find_entities_by_selector(&selector);
                let mut len = 0;
                for target in targets {
                    let name = get_name(target, &context.ecs);
                    if update_gamemode(target, &mut context.ecs, gamemode).is_ok() {
                        len += 1;
                        context.send_message(if target == context.sender {
                            Text::translate_with(
                                "commands.gamemode.success.self",
                                vec![Text::translate(format!(
                                    "gameMode.{}",
                                    gamemode.to_string()
                                ))],
                            )
                        } else {
                            Text::translate_with(
                                "commands.gamemode.success.other",
                                vec![
                                    name.into(),
                                    Text::translate(format!("gameMode.{}", gamemode.to_string())),
                                ],
                            )
                        });
                    }
                }
                Ok(len)
            });
    }

    fn update_gamemode(entity: Entity, ecs: &mut Ecs, gamemode: Gamemode) -> anyhow::Result<()> {
        let mut new_mut = ecs.get_mut::<Gamemode>(entity)?;
        let mut old_mut = ecs.get_mut::<PreviousGamemode>(entity)?;
        if *new_mut == gamemode {
            bail!("Already this gamemode")
        }

        *old_mut = PreviousGamemode(Some(*new_mut));
        *new_mut = gamemode;

        let (old, new) = (*old_mut, *new_mut);
        drop(new_mut);
        drop(old_mut);

        ecs.insert_entity_event(entity, GamemodeUpdateEvent { old, new })?;

        Ok(())
    }

    // /clear
    dispatcher
        .create_command("clear")
        .unwrap()
        .executes(|mut context: CommandCtx| {
            if context.ecs.get::<Player>(context.sender).is_ok() {
                // Go through the player's inventory and set all the slots to no items.
                // Also, keep track of how many items we delete.
                let mut count = 0;
                let sender = context.sender;
                clear_items(&mut context, sender, None, None, &mut count)?;
                // If count is zero, the player's inventory was empty and the command fails
                // "No items were found on player {0}."
                if count == 0 {
                    context.send_message(
                        Text::translate_with(
                            "clear.failed.single",
                            vec![get_name(context.sender, &context.ecs)],
                        )
                        .red(),
                    );
                    bail!("No items were found");
                }
                // If the count is not zero, we return the count of items we deleted. Command succeeds.
                // "Removed {1} items from player {0}"
                context.send_message(Text::translate_with(
                    "commands.clear.success.single",
                    vec![count.to_string(), get_name(context.sender, &context.ecs)],
                ));
                Ok(count)
            } else {
                // TODO add this check to the dispatcher
                context.send_message(Text::translate("permissions.requires.player").red());
                bail!("Requires a player")
            }
        })
        .argument("target", EntityArgument::PLAYERS, "entity")
        .executes(|mut context: CommandCtx, selector| {
            if let Some(entities) = context.find_non_empty_entities_by_selector(&selector, true) {
                let mut count = 0;
                for entity in &entities {
                    clear_items(&mut context, *entity, None, None, &mut count)?;
                }

                send_clear_message(&context, count, entities)
            } else {
                bail!("No entities were found")
            }
        })
        .argument("item", ItemPredicateArgument, "item_predicate")
        .executes(|mut context: CommandCtx, selector, item| {
            if let Some(entities) = context.find_non_empty_entities_by_selector(&selector, true) {
                let mut count = 0;
                for entity in &entities {
                    clear_items(&mut context, *entity, Some(&item), None, &mut count)?;
                }

                send_clear_message(&context, count, entities)
            } else {
                bail!("No entities were found")
            }
        })
        .argument("maxCount", IntegerArgument::new(0..=i32::MAX), "none")
        .executes(|mut context: CommandCtx, selector, item, max_count| {
            if let Some(entities) = context.find_non_empty_entities_by_selector(&selector, true) {
                let mut count = 0;
                for entity in &entities {
                    clear_items(
                        &mut context,
                        *entity,
                        Some(&item),
                        Some(max_count),
                        &mut count,
                    )?;
                }

                if max_count == 0 {
                    if entities.len() == 1 {
                        context.send_message(Text::translate_with(
                            "commands.clear.test.single",
                            vec![
                                count.to_string(),
                                get_name(*entities.first().unwrap(), &context.ecs),
                            ],
                        ));
                        Ok(count)
                    } else {
                        context.send_message(Text::translate_with(
                            "commands.clear.test.multiple",
                            vec![count.to_string(), entities.len().to_string()],
                        ));
                        Ok(count)
                    }
                } else {
                    send_clear_message(&context, count, entities)
                }
            } else {
                bail!("No entities were found")
            }
        });

    fn send_clear_message(
        context: &CommandCtx,
        count: i32,
        entities: Vec<Entity>,
    ) -> CommandOutput {
        match (count, entities.len()) {
            (0, 1) => {
                context.send_message(
                    Text::translate_with(
                        "clear.failed.single",
                        vec![get_name(*entities.first().unwrap(), &context.ecs)],
                    )
                    .red(),
                );
                bail!("No items were found")
            }
            (0, entities) => {
                context.send_message(
                    Text::translate_with("clear.failed.multiple", vec![entities.to_string()]).red(),
                );
                bail!("No items were found")
            }
            (count, 1) => {
                context.send_message(Text::translate_with(
                    "commands.clear.success.single",
                    vec![
                        count.to_string(),
                        get_name(*entities.first().unwrap(), &context.ecs),
                    ],
                ));
                Ok(count)
            }
            (count, entities) => {
                context.send_message(Text::translate_with(
                    "commands.clear.success.multiple",
                    vec![count.to_string(), entities.to_string()],
                ));
                Ok(count)
            }
        }
    }

    /// Go through a player's inventory and set all the slots that match "item" to empty, up to maxcount items removed.
    /// Also, keep track of how many items we delete total in the variable count.
    /// Will panic if entity does not have an inventory
    fn clear_items(
        context: &mut CommandCtx,
        player: Entity,
        item: Option<&ItemPredicate>,
        max_count: Option<i32>,
        count: &mut i32,
    ) -> anyhow::Result<()> {
        let inventory = context.ecs.get_mut::<Window>(player).unwrap();
        let mut changed_items: SmallVec<[usize; 2]> = SmallVec::new();
        // TODO don't clone items, they may have big NBT tags
        for (index, slot) in inventory.inner().to_vec().into_iter().enumerate() {
            if let InventorySlot::Filled(mut stack) = slot {
                if let Some(predicate) = item.as_ref() {
                    if !item_matches(context, &stack, predicate) {
                        continue;
                    }
                }
                let max_count = max_count.unwrap_or(i32::MAX);
                if max_count == 0 {
                    *count += stack.count() as i32;
                } else if (stack.count() as i32) <= max_count - *count {
                    *count += stack.count() as i32;
                    inventory.set_item(index, InventorySlot::Empty)?;
                    changed_items.push(index);
                } else {
                    stack.set_count(stack.count() - (max_count - *count) as u32)?;
                    inventory.set_item(index, InventorySlot::Filled(stack))?;
                    *count = max_count;
                    changed_items.push(index);
                    break;
                }
            }
        }
        drop(inventory);
        if !changed_items.is_empty() {
            context
                .ecs
                .insert_entity_event(player, InventoryUpdateEvent(changed_items.into_vec()))?;
        }
        Ok(())
    }

    fn item_matches(_game: &Game, item: &ItemStack, predicate: &ItemPredicate) -> bool {
        #[allow(clippy::needless_bool)]
        if !match &predicate.predicate_type {
            ItemPredicateType::Tag(_s) =>
            /*game.tag_registry.check_item_tag(
                item.item,
                &NamespacedId::from_str(s.to_string().as_str()).unwrap(),
            )*/
            {
                unimplemented!()
            }
            ItemPredicateType::Item(s) => item.item().name() == s.value(),
        } {
            false
        } else {
            // TODO compare nbt tags
            true
        }
    }

    // /ban
    dispatcher
        .create_command("ban")
        .unwrap()
        .argument("targets", EntityArgument::PLAYERS, "entity")
        .executes(|mut context: CommandCtx, selector| {
            if let Some(targets) = context.find_non_empty_entities_by_selector(selector, true) {
                Ok(ban_players(&mut context, targets, BanReason::default()) as i32)
            } else {
                bail!("No entities were found")
            }
        })
        .argument("reason", MessageArgument, "none")
        .executes(
            |mut context: CommandCtx, selector: &EntitySelector, reason: Message| {
                if let Some(targets) = context.find_non_empty_entities_by_selector(selector, true) {
                    let reason =
                        BanReason::new(reason.to_string(|s| get_entity_names(&context, s)));

                    Ok(ban_players(&mut context, targets, reason) as i32)
                } else {
                    bail!("No entities were found")
                }
            },
        );

    fn ban_players(context: &mut CommandCtx, players: Vec<Entity>, reason: BanReason) -> usize {
        let source = get_name(context.sender, &context.ecs);
        if players.is_empty() {
            0
        } else {
            let mut count = 0;
            for target in players {
                // TODO ban offline players
                let uuid = *context.ecs.get::<Uuid>(target).unwrap();
                let name = get_name(target, &context.ecs);
                if context.resources.get_mut::<BanList>().unwrap().ban(
                    uuid,
                    name.clone(),
                    Some(source.clone()),
                    reason.clone(),
                    None,
                ) {
                    count += 1;
                    context.send_message(Text::translate_with(
                        "commands.ban.success",
                        vec![name, reason.to_string()],
                    ));
                    context
                        .ecs
                        .insert_entity_event(
                            target,
                            DisconnectEvent::new(Text::translate_with(
                                "multiplayer.disconnect.banned.reason",
                                vec![reason.to_string()],
                            )),
                        )
                        .unwrap();
                } else {
                    context.send_message(Text::translate("commands.ban.failed").red());
                }
            }
            count
        }
    }

    // /ban-ip
    dispatcher
        .create_command("ban-ip")
        .unwrap()
        .argument("target", StringArgument::SINGLE_WORD, "player_names")
        .executes(|mut context, target| {
            ban_ip(&mut context, target, BanReason::default()).map(|n| n as i32)
        })
        .argument("reason", MessageArgument, "none")
        .executes(|mut context, target, reason: Message| {
            let reason = BanReason::new(reason.to_string(|s| get_entity_names(&context, s)));
            ban_ip(&mut context, target, reason).map(|n| n as i32)
        });

    fn ban_ip(context: &mut CommandCtx, ip: String, reason: BanReason) -> anyhow::Result<usize> {
        let sender = context.sender;
        let source = get_name(sender, &context.ecs);
        let ip = if let Ok(ip) = ip.parse() {
            ip
        } else if let Some(target) = context
            .ecs
            .query::<(&Player, &Name)>()
            .iter()
            .find(|(_, (_, name))| ****name == ip)
            .map(|(e, _)| e)
        {
            context.ecs.get::<RealIp>(target).unwrap().0
        } else {
            context.send_message(Text::translate("commands.banip.invalid").red());
            bail!("Invalid IP")
        };

        if context.resources.get_mut::<BanList>().unwrap().ban_ip(
            ip,
            Some(source),
            reason.clone(),
            None,
        ) {
            context.send_message(Text::translate_with(
                "commands.banip.success",
                vec![ip.to_string(), reason.to_string()],
            ));
            let targets = context
                .ecs
                .query::<(&Player, &RealIp)>()
                .iter()
                .filter(|(_, (_, &addr))| *addr == ip)
                .map(|(entity, _)| entity)
                .collect::<Vec<_>>();
            for target in targets {
                context
                    .ecs
                    .insert_entity_event(
                        target,
                        DisconnectEvent::new(Text::translate_with(
                            "multiplayer.disconnect.banned_ip.reason",
                            vec![reason.to_string()],
                        )),
                    )
                    .unwrap();
            }
            Ok(1)
        } else {
            context.send_message(Text::translate("commands.banip.failed").red());
            bail!("This IP is not banned")
        }
    }

    fn get_entity_names(
        context: &CommandCtx,
        selector: Vec<EntitySelectorPredicate>,
    ) -> Vec<String> {
        context
            .find_entities_by_selector(&EntitySelector::Selector(selector))
            .iter()
            .map(|&e| get_name(e, &context.ecs))
            .collect()
    }

    // /pardon
    dispatcher
        .create_command("pardon")
        .unwrap()
        .argument("targets", EntityArgument::PLAYERS, "banned_players")
        .executes(|context: CommandCtx, selector| {
            match selector {
                EntitySelector::Selector(s) => {
                    if context.find_non_empty_entities_by_selector(&EntitySelector::Selector(s), true).is_some() {
                        // If targets are online, then they're not banned
                        context.send_message(Text::translate("commands.pardon.failed"));
                    }
                    bail!("Tried to /pardon a selector of online players, but since they're online, they're not banned")
                }
                EntitySelector::Name(name) => {
                    let mut banlist = context.resources.get_mut::<BanList>().unwrap();
                    if banlist.pardon_name(&name) {
                        context.send_message(Text::translate_with(
                            "commands.pardon.success",
                            vec![name],
                        ));
                        Ok(1)
                    } else {
                        context.send_message(Text::translate("commands.pardon.failed").red());
                        bail!("This player is not banned")
                    }
                }
                EntitySelector::Uuid(uuid) => {
                    let mut banlist = context.resources.get_mut::<BanList>().unwrap();
                    if banlist.pardon_id(&uuid) {
                        context.send_message(Text::translate_with(
                            "commands.pardon.success",
                            vec![uuid.to_string()],
                        ));
                        Ok(1)
                    } else {
                        context.send_message(Text::translate("commands.pardon.failed").red());
                        bail!("This player is not banned")
                    }
                }
            }
        });

    // /pardon-ip
    dispatcher
        .create_command("pardon-ip")
        .unwrap()
        .argument("targets", StringArgument::SINGLE_WORD, "banned_ips")
        .executes(|context: CommandCtx, ip: String| {
            if let Ok(ip) = ip.parse() {
                let mut banlist = context.resources.get_mut::<BanList>().unwrap();
                if banlist.pardon_ip(&ip) {
                    context.send_message(Text::translate_with(
                        "commands.pardonip.success",
                        vec![ip.to_string()],
                    ));
                    Ok(1)
                } else {
                    context.send_message(Text::translate("commands.pardonip.failed").red());
                    bail!("This IP is not banned")
                }
            } else {
                context.send_message(Text::translate("commands.pardonip.invalid").red());
                bail!("Invalid IP")
            }
        });

    // /say
    dispatcher
        .create_command("say")
        .unwrap()
        .argument("message", MessageArgument, "none")
        .executes(|context: CommandCtx, message: Message| {
            let command_output = Text::translate_with(
                "chat.type.announcement",
                vec![
                    get_name(context.sender, &context.ecs),
                    message.to_string(|s| get_entity_names(&context, s)),
                ],
            );
            context
                .ecs
                .query::<&mut ChatBox>()
                .iter()
                .for_each(|(_, chat_box)| chat_box.send_chat(command_output.clone()));
            Ok(1)
        });
}

// #[command(usage = "tp|teleport <destination>")]
// pub fn tp_1(context: &mut CommandCtx, destination: EntitySelector) -> anyhow::Result<Option<String>> {
//     let entities = find_selected_entities(ctx, &destination.requirements)?;
//     if let Some(first) = entities.first() {
//         if let Ok(pos) = context.ecs.get::<Position>(*first).map(|r| *r) {
//             teleport_entity_to_pos(&context.ecs, context.sender, pos);
//         }
//
//         Ok(Some(format!(
//             "Teleported {0} to {1}",
//             *context.ecs.get::<Name>(context.sender).unwrap(),
//             *context.ecs.get::<Name>(*first).unwrap()
//         )))
//     } else {
//         Err(TpError::NoMatchingEntities.into())
//     }
// }
//
// #[command(usage = "tp|teleport <location>")]
// pub fn tp_2(context: &mut CommandCtx, location: Coordinates) -> anyhow::Result<Option<String>> {
//     teleport_entity(&context.ecs, context.sender, location);
//
//     let position = context.ecs.get::<Position>(context.sender).unwrap();
//     Ok(Some(format!(
//         "Teleported {0} to {1}, {2}, {3}",
//         *context.ecs.get::<Name>(context.sender).unwrap(),
//         position.x,
//         position.y,
//         position.z
//     )))
// }
//
// #[command(usage = "tp|teleport <targets> <location>")]
// pub fn tp_3(
//     context: &mut CommandCtx,
//     targets: EntitySelector,
//     location: Coordinates,
// ) -> anyhow::Result<Option<String>> {
//     let entities = find_selected_entities(ctx, &targets.requirements)?;
//     if entities.is_empty() {
//         Err(TpError::NoMatchingEntities.into())
//     } else {
//         for entity in &entities {
//             teleport_entity(&context.ecs, *entity, location);
//         }
//
//         let position = context.ecs.get::<Position>(*entities.first().unwrap()).unwrap();
//         Ok(Some(format!(
//             "Teleported {0} to {1}, {2}, {3}",
//             targets.entities_to_string(ctx, &entities.into_vec(), false),
//             position.x,
//             position.y,
//             position.z
//         )))
//     }
// }
//
// #[command(usage = "tp|teleport <targets> <destination>")]
// pub fn tp_4(
//     context: &mut CommandCtx,
//     targets: EntitySelector,
//     destination: EntitySelector,
// ) -> anyhow::Result<Option<String>> {
//     let entities = find_selected_entities(ctx, &targets.requirements)?;
//     if entities.len() > 1 {
//         Err(TpError::TooManyEntities.into())
//     } else if let Some(Ok(location)) = entities
//         .first()
//         .map(|e| context.ecs.get::<Position>(*e).map(|r| *r))
//     {
//         if entities.is_empty() {
//             Err(TpError::NoMatchingEntities.into())
//         } else {
//             for entity in &entities {
//                 teleport_entity_to_pos(&context.ecs, *entity, location);
//             }
//             let entities = entities.into_vec();
//             Ok(Some(format!(
//                 "Teleported {0} to {1}",
//                 targets.entities_to_string(ctx, &entities, false),
//                 destination.entities_to_string(ctx, &entities, false)
//             )))
//         }
//     } else {
//         Err(TpError::NoMatchingEntities.into())
//     }
// }
//
// fn teleport_entity(ecs: &Ecs, entity: Entity, location: Coordinates) {
//     let new_pos = ecs
//         .get::<Position>(entity)
//         .map(|r| *r)
//         .map(|relative_to| location.into_position(relative_to));
//
//     if let Ok(new_pos) = new_pos {
//         teleport_entity_to_pos(ecs, entity, new_pos);
//     }
// }
//
// fn teleport_entity_to_pos(ecs: &Ecs, entity: Entity, pos: Position) {
//     if let Ok(mut old_pos) = ecs.get_mut::<Position>(entity) {
//         *old_pos = pos;
//     }
//     //let _ = game.ecs.entity(entity).add(Teleported);
// }
//
//
//     if let Some(event) = event {
//         context.ecs.insert_entity_event(entity, event).unwrap();
//     }
// }
//
// #[command(usage = "tell|msg|w <target> <message>")]
// pub fn whisper(
//     context: &mut CommandCtx,
//     target: EntitySelector,
//     message: TextArgument,
// ) -> anyhow::Result<Option<String>> {
//     let entities = find_selected_entities(ctx, &target.requirements)?;
//     let sender_name = if let Ok(sender_name) = context.ecs.get::<Name>(context.sender) {
//         (**sender_name).to_owned()
//     } else {
//         // Use a default value if the executor has no Name component
//         String::from("Server")
//     };
//
//     // The message that is returned to the whisperer
//     // You whisper to [player] (and [player]): [message]
//     let mut response_message = String::from("You whisper to");
//
//     // Tracks if there needs to be "and" before the next player added to the response message
//     let mut needs_and = false;
//
//     for entity in entities {
//         if let Ok(mut chat) = context.ecs.get_mut::<ChatBox>(context.sender) {
//             chat.send_system(
//                 Text::from(format!(
//                     "{} whispers to you: {}",
//                     sender_name,
//                     message.0.clone()
//                 ))
//                 .gray()
//                 .italic(),
//             );
//         } else {
//             // If the entity doesn't have a message receiver it is not a player and there is no need to continue
//             continue;
//         };
//
//         if let Ok(player_name) = context.ecs.get::<Name>(entity) {
//             if needs_and {
//                 response_message += format!(" and {}", *player_name).as_str();
//             } else {
//                 needs_and = true;
//
//                 response_message += format!(" {}", *player_name).as_str();
//             }
//         }
//     }
//
//     // Send the whisperer a confirmation message
//     if let Ok(mut chat) = context.ecs.get_mut::<ChatBox>(context.sender) {
//         response_message += format!(": {}", message.0).as_str();
//         let return_text = Text::from(response_message).gray().italic();
//
//         chat.send_system(return_text);
//     }
//
//     Ok(None)
// }
//
// #[command(usage = "say <message>")]
// pub fn say(context: &mut CommandCtx, message: TextArgument) -> anyhow::Result<Option<String>> {
//     let name = context.ecs.get::<Name>(context.sender);
//
//     let sender_name = if let Ok(name) = &name {
//         &***name
//     } else {
//         "Server"
//     };
//
//     let command_output = Text::from(format!("[{}] {}", sender_name, message.0));
//
//     drop(name);
//
//     context.ecs
//         .query::<&mut ChatBox>()
//         .iter()
//         .for_each(|(_, chat_box)| chat_box.send_chat(command_output.clone()));
//
//     Ok(None)
// }
//
// #[derive(Debug, Error)]
// pub enum KickError {
//     #[error(
//         "Only players may be affected by this command, but the provided selector includes entities"
//     )]
//     NoEntities,
// }
//
// #[command(usage = "kick <targets>")]
// pub fn kick_1(context: &mut CommandCtx, targets: EntitySelector) -> anyhow::Result<Option<String>> {
//     kick_players(
//         ctx,
//         &targets,
//         TextValue::translate("multiplayer.disconnect.kicked").into(),
//     )
// }
//
// #[command(usage = "kick <targets> <reason>")]
// pub fn kick_2(
//     context: &mut CommandCtx,
//     targets: EntitySelector,
//     reason: TextArgument,
// ) -> anyhow::Result<Option<String>> {
//     kick_players(ctx, &targets, reason.0.into())
// }
//
// fn kick_players(
//     context: &mut CommandCtx,
//     targets: &EntitySelector,
//     reason: Text,
// ) -> anyhow::Result<Option<String>> {
//     let entities = find_selected_entities(ctx, &targets.requirements)?;
//     for entity in &entities {
//         if context.ecs.get::<Player>(*entity).is_err() {
//             return Err(KickError::NoEntities.into());
//         }
//     }
//
//     for entity in &entities {
//         let name = (**context.ecs.get::<Name>(*entity).unwrap()).to_owned();
//         let _client_id = context.ecs.get::<ClientId>(*entity).unwrap();
//
//         // Send confirmation message
//         // TODO Server ops should also see the message
//         if let Ok(mut chat) = context.ecs.get_mut::<ChatBox>(context.sender) {
//             let kick_confirm = Text::translate_with(
//                 "commands.kick.success",
//                 vec![Text::from(name), reason.clone()],
//             );
//             chat.send_system(kick_confirm);
//         }
//     }
//     Ok(None)
// }
//
// #[command(usage = "stop")]
// pub fn stop(context: &mut CommandCtx) -> anyhow::Result<Option<String>> {
//     // Confirmation message
//     // TODO Server ops should also see the message
//     if let Ok(mut chat) = context.ecs.get_mut::<ChatBox>(context.sender) {
//         let text = Text::from(TextValue::translate("commands.stop.stopping"));
//         chat.send_system(text);
//     }
//
//     //context.game
//     //    .resources
//     //    .get::<ShutdownChannels>()
//     //    .tx
//     //    .try_send(())?;
//
//     Ok(None)
// }
//
// #[command(usage = "seed")]
// pub fn seed(context: &mut CommandCtx) -> anyhow::Result<Option<String>> {
//     if let Ok(mut chat) = context.ecs.get_mut::<ChatBox>(context.sender) {
//         let world_seed = "TODO";
//         chat.send_system(
//             Text::from("Seed: [")
//                 + Text::from(world_seed.to_string())
//                     .green()
//                     .on_click_copy_to_clipboard(world_seed.to_string())
//                     .on_hover_show_text(TextValue::translate("chat.copy.click"))
//                 + Text::from("]"),
//         );
//     }
//     Ok(None)
// }
//
// #[command(usage = "time query <info>")]
// pub fn time_query(
//     context: &mut CommandCtx,
//     info: TimeQueryInformation,
// ) -> anyhow::Result<Option<String>> {
//     let time = match info {
//         TimeQueryInformation::DayTime => context.world.time.time(),
//         TimeQueryInformation::GameTime => context.world.time.world_age(),
//         TimeQueryInformation::Day => context.world.time.days(),
//     };
//
//     if let Ok(mut chat) = context.ecs.get_mut::<ChatBox>(context.sender) {
//         let message =
//             Text::translate_with("commands.time.query", vec![Text::from(time.to_string())]);
//         chat.send_system(message);
//     }
//
//     Ok(None)
// }
//
// #[command(usage = "time add <time>")]
// pub fn time_add(context: &mut CommandCtx, time: TimeArgument) -> anyhow::Result<Option<String>> {
//     let time = context.world.time.time() + time.0;
//     set_time(ctx, time);
//     Ok(None)
// }
//
// #[command(usage = "time set <time>")]
// pub fn time_set_0(context: &mut CommandCtx, time: TimeArgument) -> anyhow::Result<Option<String>> {
//     set_time(ctx, time.0);
//     Ok(None)
// }
//
// #[command(usage = "time set <time_spec>")]
// pub fn time_set_1(context: &mut CommandCtx, time_spec: TimeSpec) -> anyhow::Result<Option<String>> {
//     set_time(
//         ctx,
//         match time_spec {
//             TimeSpec::Day => 1_000,
//             TimeSpec::Noon => 6_000,
//             TimeSpec::Night => 13_000,
//             TimeSpec::Midnight => 18_000,
//         },
//     );
//     Ok(None)
// }
//
// fn set_time(context: &mut CommandCtx, time: u64) {
//     context.ecs.insert_event(TimeUpdateEvent {
//         old: context.world.time.time(),
//         new: time,
//     });
//     context.world.time.set_time(time);
// }

fn get_name(e: Entity, ecs: &Ecs) -> String {
    ecs.get::<Name>(e)
        .map(|name| (***name).to_string())
        .or_else(|_| ecs.get::<CustomName>(e).map(|name| (***name).to_string()))
        .or_else(|_| ecs.get::<EntityKind>(e).map(|k| k.display_name().into()))
        .unwrap()
}
