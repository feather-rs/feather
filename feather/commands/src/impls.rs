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
use libcraft_core::Position;
use libcraft_items::{InventorySlot, ItemStack};
use libcraft_text::{Text, TextComponentBuilder};
use quill_common::components::{
    ChatBox, DefaultGamemode, Gamemode, Name, PreviousGamemode, RealIp,
};
use quill_common::entities::Player;
use quill_common::events::{DisconnectEvent, GamemodeUpdateEvent, InventoryUpdateEvent};

use crate::utils::*;
use crate::CommandCtx;

pub fn register_all(dispatcher: &mut CommandDispatcher<CommandCtx, Text>) {
    // /me
    dispatcher
        .create_command("me")
        .argument("text", StringArgument::GREEDY_PHRASE, None)
        .executes(|context: CommandCtx, action: &mut String| {
            let command_output = Text::translate_with(
                "chat.type.emote",
                vec![
                    get_entity_name(context.sender, &context.ecs),
                    action.to_owned(),
                ],
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
        .with(|command| gamemode_command(command, "survival", Gamemode::Survival))
        .with(|command| gamemode_command(command, "creative", Gamemode::Creative))
        .with(|command| gamemode_command(command, "adventure", Gamemode::Adventure))
        .with(|command| gamemode_command(command, "spectator", Gamemode::Spectator));
    fn gamemode_command(command: CreateCommand<CommandCtx, Text, ()>, s: &str, gamemode: Gamemode) {
        command
            .subcommand(s)
            .executes(move |mut context: CommandCtx| {
                if context.ecs.get::<Player>(context.sender).is_ok() {
                    update_gamemode(context.sender, &mut context.ecs, gamemode)?;
                    context.send_message(Text::translate_with(
                        "commands.gamemode.success.self",
                        vec![Text::translate(format!(
                            "gameMode.{}",
                            gamemode.to_string()
                        ))],
                    ));
                    Ok(1)
                } else {
                    context.send_message(Text::translate("permissions.requires.player").red());
                    bail!("Requires a player")
                }
            })
            .argument("target", EntityArgument::PLAYERS, "minecraft:entity")
            .executes(
                move |mut context: CommandCtx, selector: &mut EntitySelector| {
                    if let Some(targets) =
                        context.find_non_empty_entities_by_selector(selector, true)
                    {
                        let mut len = 0;
                        for target in targets {
                            let name = get_entity_name(target, &context.ecs);
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
                                            Text::translate(format!(
                                                "gameMode.{}",
                                                gamemode.to_string()
                                            )),
                                        ],
                                    )
                                });
                            }
                        }
                        Ok(len)
                    } else {
                        bail!("No entities were found")
                    }
                },
            );
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
                            vec![get_entity_name(context.sender, &context.ecs)],
                        )
                        .red(),
                    );
                    bail!("No items were found");
                }
                // If the count is not zero, we return the count of items we deleted. Command succeeds.
                // "Removed {1} items from player {0}"
                context.send_message(Text::translate_with(
                    "commands.clear.success.single",
                    vec![
                        count.to_string(),
                        get_entity_name(context.sender, &context.ecs),
                    ],
                ));
                Ok(count)
            } else {
                context.send_message(Text::translate("permissions.requires.player").red());
                bail!("Requires a player")
            }
        })
        .argument("target", EntityArgument::PLAYERS, "minecraft:entity")
        .executes(|mut context: CommandCtx, selector: &mut EntitySelector| {
            if let Some(entities) = context.find_non_empty_entities_by_selector(selector, true) {
                let mut count = 0;
                for entity in &entities {
                    clear_items(&mut context, *entity, None, None, &mut count)?;
                }

                send_clear_message(&context, count, entities)
            } else {
                bail!("No entities were found")
            }
        })
        .argument("item", ItemPredicateArgument, "minecraft:item_predicate")
        .executes(
            |mut context: CommandCtx, selector: &mut EntitySelector, item: &mut ItemPredicate| {
                if let Some(entities) = context.find_non_empty_entities_by_selector(selector, true)
                {
                    let mut count = 0;
                    for entity in &entities {
                        clear_items(&mut context, *entity, Some(item), None, &mut count)?;
                    }

                    send_clear_message(&context, count, entities)
                } else {
                    bail!("No entities were found")
                }
            },
        )
        .argument("maxCount", IntegerArgument::new(0..=i32::MAX), None)
        .executes(
            |mut context: CommandCtx,
             selector: &mut EntitySelector,
             item: &mut ItemPredicate,
             max_count: &mut i32| {
                if let Some(entities) = context.find_non_empty_entities_by_selector(selector, true)
                {
                    let mut count = 0;
                    for entity in &entities {
                        clear_items(
                            &mut context,
                            *entity,
                            Some(item),
                            Some(*max_count),
                            &mut count,
                        )?;
                    }

                    if *max_count == 0 {
                        if entities.len() == 1 {
                            context.send_message(Text::translate_with(
                                "commands.clear.test.single",
                                vec![
                                    count.to_string(),
                                    get_entity_name(*entities.first().unwrap(), &context.ecs),
                                ],
                            ));
                        } else {
                            context.send_message(Text::translate_with(
                                "commands.clear.test.multiple",
                                vec![count.to_string(), entities.len().to_string()],
                            ));
                        }
                        Ok(count)
                    } else {
                        send_clear_message(&context, count, entities)
                    }
                } else {
                    bail!("No entities were found")
                }
            },
        );

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
                        vec![get_entity_name(*entities.first().unwrap(), &context.ecs)],
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
                        get_entity_name(*entities.first().unwrap(), &context.ecs),
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
        .argument("targets", EntityArgument::PLAYERS, "minecraft:entity")
        .executes(|mut context: CommandCtx, selector: &mut EntitySelector| {
            if let Some(targets) = context.find_non_empty_entities_by_selector(selector, true) {
                Ok(ban_players(&mut context, targets, BanReason::default()) as i32)
            } else {
                bail!("No entities were found")
            }
        })
        .argument("reason", MessageArgument, None)
        .executes(
            |mut context: CommandCtx, selector: &mut EntitySelector, reason: &mut Message| {
                if let Some(targets) = context.find_non_empty_entities_by_selector(selector, true) {
                    let reason = BanReason::new(reason.to_string(|s| {
                        get_entity_names(context.sender, &context, &EntitySelector::Selector(s))
                    }));

                    Ok(ban_players(&mut context, targets, reason) as i32)
                } else {
                    bail!("No entities were found")
                }
            },
        );

    fn ban_players(context: &mut CommandCtx, players: Vec<Entity>, reason: BanReason) -> usize {
        let source = get_entity_name(context.sender, &context.ecs);
        if players.is_empty() {
            0
        } else {
            let mut count = 0;
            for target in players {
                // TODO ban offline players
                let uuid = *context.ecs.get::<Uuid>(target).unwrap();
                let name = get_entity_name(target, &context.ecs);
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
        .argument("target", StringArgument::SINGLE_WORD, "player_names")
        .executes(|mut context, target: &mut String| {
            ban_ip(&mut context, target.to_owned(), BanReason::default()).map(|n| n as i32)
        })
        .argument("reason", MessageArgument, None)
        .executes(
            |mut context: CommandCtx, target: &mut String, reason: &mut Message| {
                let reason = BanReason::new(reason.to_string(|s| {
                    get_entity_names(context.sender, &context, &EntitySelector::Selector(s))
                }));
                ban_ip(&mut context, target.to_owned(), reason).map(|n| n as i32)
            },
        );

    fn ban_ip(context: &mut CommandCtx, ip: String, reason: BanReason) -> anyhow::Result<usize> {
        let sender = context.sender;
        let source = get_entity_name(sender, &context.ecs);
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

    // /pardon
    dispatcher
        .create_command("pardon")
        .argument("targets", EntityArgument::PLAYERS, "minecraft:banned_players")
        .executes(|context: CommandCtx, selector: &mut EntitySelector| {
            match selector {
                EntitySelector::Selector(_) => {
                    if context.find_non_empty_entities_by_selector(selector, true).is_some() {
                        // If targets are online, then they're not banned
                        context.send_message(Text::translate("commands.pardon.failed"));
                    }
                    bail!("Tried to /pardon a selector of online players, but since they're online, they're not banned")
                }
                EntitySelector::Name(name) => {
                    let mut banlist = context.resources.get_mut::<BanList>().unwrap();
                    if banlist.pardon_name(name) {
                        context.send_message(Text::translate_with(
                            "commands.pardon.success",
                            vec![name.to_owned()],
                        ));
                        Ok(1)
                    } else {
                        context.send_message(Text::translate("commands.pardon.failed").red());
                        bail!("This player is not banned")
                    }
                }
                EntitySelector::Uuid(uuid) => {
                    let mut banlist = context.resources.get_mut::<BanList>().unwrap();
                    if banlist.pardon_id(uuid) {
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
        .argument(
            "targets",
            StringArgument::SINGLE_WORD,
            "minecraft:banned_ips",
        )
        .executes(|context: CommandCtx, ip: &mut String| {
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
        .argument("message", MessageArgument, None)
        .executes(|context: CommandCtx, message: &mut Message| {
            let command_output = Text::translate_with(
                "chat.type.announcement",
                vec![
                    get_entity_name(context.sender, &context.ecs),
                    message.to_string(|s| {
                        get_entity_names(context.sender, &context, &EntitySelector::Selector(s))
                    }),
                ],
            );
            context
                .ecs
                .query::<&mut ChatBox>()
                .iter()
                .for_each(|(_, chat_box)| chat_box.send_chat(command_output.clone()));
            Ok(1)
        });

    // /execute
    let root = 0;
    let command = dispatcher.create_command("execute");
    let execute = command.current_node_id();
    command
        .with(|command| {
            command.subcommand("run").redirect(root);
        })
        .with(|command| {
            command
                .subcommand("as")
                .argument("executors", EntityArgument::ENTITIES, "minecraft:entity")
                .redirect(execute)
                .fork(|args, mut context, mut f| {
                    let arg = args.remove(0);
                    let selector = arg.downcast_ref::<EntitySelector>().unwrap();
                    if let Some(entities) =
                        context.find_non_empty_entities_by_selector(selector, false)
                    {
                        let position = context.position;
                        let anchor = context.anchor.clone();
                        if entities.len() == 1 {
                            f(
                                args,
                                CommandCtx::new(&mut *context, entities[0], position, anchor),
                            )
                        } else {
                            for entity in entities {
                                let _ = f(
                                    args,
                                    CommandCtx::new(
                                        &mut *context,
                                        entity,
                                        position,
                                        anchor.clone(),
                                    ),
                                );
                            }
                            Ok(0)
                        }
                    } else {
                        bail!("No entities were found")
                    }
                });
        })
        .with(|command| {
            command
                .subcommand("at")
                .argument("positions", EntityArgument::ENTITIES, "minecraft:entity")
                .redirect(execute)
                .fork(|args, mut context, mut f| {
                    let arg = args.remove(0);
                    let selector = arg.downcast_ref::<EntitySelector>().unwrap();
                    if let Some(entities) =
                        context.find_non_empty_entities_by_selector(selector, false)
                    {
                        let sender = context.sender;
                        let anchor = context.anchor.clone();
                        if entities.len() == 1 {
                            let pos = context
                                .ecs
                                .get::<Position>(entities[0])
                                .ok()
                                .map(|pos| *pos);
                            f(args, CommandCtx::new(&mut *context, sender, pos, anchor))
                        } else {
                            for entity in entities {
                                let pos = context.ecs.get::<Position>(entity).ok().map(|pos| *pos);
                                let _ = f(
                                    args,
                                    CommandCtx::new(&mut *context, sender, pos, anchor.clone()),
                                );
                            }
                            Ok(0)
                        }
                    } else {
                        bail!("No entities were found")
                    }
                });
        })
        .with(|command| {
            command
                .subcommand("align")
                .argument("positions", SwizzleArgument, None)
                .redirect(execute)
                .fork(|args, mut context, mut f| {
                    let arg = args.remove(0);
                    let swizzle = arg.downcast_ref::<Swizzle>().unwrap();
                    if let Some(pos) = context.position.as_mut() {
                        if swizzle.x {
                            pos.x = pos.x.floor();
                        }
                        if swizzle.y {
                            pos.y = pos.y.floor();
                        }
                        if swizzle.z {
                            pos.z = pos.z.floor();
                        }
                    }
                    f(args, context)
                });
        })
        .with(|command| {
            command
                .subcommand("anchored")
                .argument("anchor", EntityAnchorArgument, "minecraft:entity_anchor")
                .redirect(execute)
                .fork(|args, mut context, mut f| {
                    let arg = args.remove(0);
                    context.anchor = arg.downcast_ref::<EntityAnchor>().unwrap().clone();
                    f(args, context)
                });
        });

    // /banlist
    dispatcher
        .create_command("banlist")
        .executes(|ctx: CommandCtx| {
            let banlist = ctx.resources.get::<BanList>().unwrap();
            let bans = banlist.players().len() + banlist.ips().len();

            if bans == 0 {
                ctx.send_message(Text::translate("commands.banlist.none"));
                Ok(0)
            } else {
                ctx.send_message(Text::translate_with(
                    "commands.banlist.list",
                    vec![bans.to_string()],
                ));
                for entry in banlist.players() {
                    ctx.send_message(Text::translate_with(
                        "commands.banlist.entry",
                        vec![
                            entry.value.1.clone(),
                            entry
                                .source
                                .clone()
                                .unwrap_or_else(|| "Console".to_string()),
                            entry.reason.to_string(),
                        ],
                    ));
                }
                for entry in banlist.ips() {
                    ctx.send_message(Text::translate_with(
                        "commands.banlist.entry",
                        vec![
                            entry.value.to_string(),
                            entry
                                .source
                                .clone()
                                .unwrap_or_else(|| "Console".to_string()),
                            entry.reason.to_string(),
                        ],
                    ));
                }
                Ok(bans as i32)
            }
        });

    // /time
    dispatcher
        .create_command("time")
        .with(|command| {
            command
                .subcommand("query")
                .with(|command| {
                    command.subcommand("daytime").executes(|ctx: CommandCtx| {
                        Ok(time_query(&ctx, ctx.world.time.time_of_day()))
                    });
                })
                .with(|command| {
                    command
                        .subcommand("day")
                        .executes(|ctx: CommandCtx| Ok(time_query(&ctx, ctx.world.time.days())));
                })
                .with(|command| {
                    command.subcommand("gametime").executes(|ctx: CommandCtx| {
                        Ok(time_query(&ctx, ctx.world.time.world_age()))
                    });
                });
        })
        .with(|command| {
            command
                .subcommand("add")
                .argument("time", TimeArgument, "minecraft:time")
                .executes(|mut ctx: CommandCtx, time: &mut (TimeUnit, f32)| {
                    let time = ctx.world.time.time() + time_to_ticks(time);
                    Ok(set_time(&mut ctx, time))
                });
        })
        .with(|command| {
            command
                .subcommand("set")
                .with(|command| {
                    command
                        .argument("time", TimeArgument, "minecraft:time")
                        .executes(|mut ctx: CommandCtx, time: &mut (TimeUnit, f32)| {
                            let time = time_to_ticks(time);
                            Ok(set_time(&mut ctx, time))
                        });
                })
                .with(|command| {
                    command
                        .subcommand("day")
                        .executes(|mut ctx| Ok(set_time(&mut ctx, 1000)));
                })
                .with(|command| {
                    command
                        .subcommand("noon")
                        .executes(|mut ctx| Ok(set_time(&mut ctx, 6000)));
                })
                .with(|command| {
                    command
                        .subcommand("night")
                        .executes(|mut ctx| Ok(set_time(&mut ctx, 13000)));
                })
                .with(|command| {
                    command
                        .subcommand("midnight")
                        .executes(|mut ctx| Ok(set_time(&mut ctx, 18000)));
                });
        });

    // /defaultgamemode
    dispatcher
        .create_command("defaultgamemode")
        .with(|command| default_gamemode(command, "survival", Gamemode::Survival))
        .with(|command| default_gamemode(command, "creative", Gamemode::Creative))
        .with(|command| default_gamemode(command, "adventure", Gamemode::Adventure))
        .with(|command| default_gamemode(command, "spectator", Gamemode::Spectator));
    fn default_gamemode(command: CreateCommand<CommandCtx, Text, ()>, s: &str, gamemode: Gamemode) {
        command.subcommand(s).executes(move |context: CommandCtx| {
            **context.resources.get_mut::<DefaultGamemode>().unwrap() = gamemode;
            context.send_message(Text::translate_with(
                "commands.defaultgamemode.success",
                vec![Text::translate(format!(
                    "gameMode.{}",
                    gamemode.to_string()
                ))],
            ));
            Ok(0)
        });
    }

    dispatcher.register_tab_completion("minecraft:banned_players", |text, ctx| {
        (
            0,
            text.len(),
            ctx.resources
                .get::<BanList>()
                .unwrap()
                .players()
                .into_iter()
                .map(|entry| entry.value.1.clone())
                .filter(|name| name.starts_with(text) && name != text)
                .map(|name| (name, None))
                .collect(),
        )
    });

    dispatcher.register_tab_completion("minecraft:banned_ips", |text, ctx| {
        (
            0,
            text.len(),
            ctx.resources
                .get::<BanList>()
                .unwrap()
                .ips()
                .into_iter()
                .map(|entry| entry.value.to_string())
                .filter(|ip| ip.starts_with(text) && ip != text)
                .map(|ip| (ip, None))
                .collect(),
        )
    });

    fn time_to_ticks(time: &(TimeUnit, f32)) -> u64 {
        (time.1
            * match time.0 {
                TimeUnit::Days => 24000.0,
                TimeUnit::Seconds => 20.0,
                TimeUnit::Ticks => 1.0,
            }) as u64
    }

    fn time_query(context: &CommandCtx, time: u64) -> i32 {
        context.send_message(Text::translate_with(
            "commands.time.query",
            vec![time.to_string()],
        ));
        time as i32
    }

    fn set_time(context: &mut CommandCtx, time: u64) -> i32 {
        context.set_time(time);
        let time_of_day = context.world.time.time_of_day();
        context.send_message(Text::translate_with(
            "commands.time.set",
            vec![time_of_day.to_string()],
        ));
        time_of_day as i32
    }

    dispatcher.register_tab_completion(
        "minecraft:entity_anchor",
        fixed_completion(vec!["feet", "eyes"]),
    );

    dispatcher.register_tab_completion("minecraft:entity", |text, ctx| {
        let players = ctx
            .ecs
            .query::<(&Player, &Name)>()
            .iter()
            .map(|(_, (_, name))| name.to_string())
            .collect::<Vec<_>>();
        if text.is_empty() {
            let mut results = players
                .into_iter()
                .map(|name| (name, None))
                .collect::<Vec<_>>();
            results.extend([
                (
                    "@a".to_string(),
                    Some(Text::translate("argument.entity.selector.allPlayers")),
                ),
                (
                    "@e".to_string(),
                    Some(Text::translate("argument.entity.selector.allEntities")),
                ),
                (
                    "@p".to_string(),
                    Some(Text::translate("argument.entity.selector.nearestPlayer")),
                ),
                (
                    "@r".to_string(),
                    Some(Text::translate("argument.entity.selector.randomPlayer")),
                ),
                (
                    "@s".to_string(),
                    Some(Text::translate("argument.entity.selector.self")),
                ),
            ]);
            (0, 0, results)
        } else if text == "@" {
            (
                0,
                1,
                vec![
                    (
                        "@a".to_string(),
                        Some(Text::translate("argument.entity.selector.allPlayers")),
                    ),
                    (
                        "@e".to_string(),
                        Some(Text::translate("argument.entity.selector.allEntities")),
                    ),
                    (
                        "@p".to_string(),
                        Some(Text::translate("argument.entity.selector.nearestPlayer")),
                    ),
                    (
                        "@r".to_string(),
                        Some(Text::translate("argument.entity.selector.randomPlayer")),
                    ),
                    (
                        "@s".to_string(),
                        Some(Text::translate("argument.entity.selector.self")),
                    ),
                ],
            )
        } else if ["@a", "@e", "@p", "@r", "@s"].contains(&text) {
            (2, 0, vec![("[".to_string(), None)])
        } else if text.starts_with("@a[")
            || text.starts_with("@e[")
            || text.starts_with("@p[")
            || text.starts_with("@r[")
            || text.starts_with("@s[")
        {
            // TODO rewrite selector parser
            (0, 0, Vec::new())
        } else {
            (
                0,
                text.len(),
                players
                    .into_iter()
                    .filter(|name| name.starts_with(text))
                    .map(|name| (name, None))
                    .collect::<Vec<_>>(),
            )
        }
    });

    dispatcher.register_tab_completion("minecraft:item_predicate", |text, _ctx| {
        #[allow(clippy::if_same_then_else)]
        let items: Vec<String> = if text.starts_with('#') {
            // TODO tags
            vec![]
        } else {
            // TODO Item::values()
            vec![]
        };
        if let Some(item) = items.into_iter().find(|item| item.starts_with(text)) {
            if item == text {
                (item.len(), 0, vec![("{".to_string(), None)])
            } else {
                (0, text.len(), vec![(item, None)])
            }
        } else {
            (0, 0, Vec::new())
        }
    });

    dispatcher.register_tab_completion("minecraft:time", |text, _ctx| {
        if text.parse::<f64>().is_ok() {
            (
                text.len(),
                0,
                vec![
                    ("d".to_string(), None),
                    ("s".to_string(), None),
                    ("t".to_string(), None),
                ],
            )
        } else {
            (0, 0, Vec::new())
        }
    })
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
// fn teleport_entity(ecs: &mut Ecs, entity: Entity, location: Coordinates) {
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
// fn teleport_entity_to_pos(ecs: &mut Ecs, entity: Entity, pos: Position) {
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
//     targets: &mut EntitySelector,
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
