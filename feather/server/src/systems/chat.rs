use commands::arguments::EntityArgument;

use common::Game;
use ecs::{EntityBuilder, HasResources, SysResult, SystemExecutor};
use feather_commands::{CommandCtx, CommandDispatcher};
use libcraft_text::text::{Color, IntoTextComponent, Style};
use quill_common::components::{ChatBox, ChatPreference, ClientId, Console, Name};

use crate::console_input::ConsoleInput;
use crate::Server;

pub fn register(game: &mut Game, systems: &mut SystemExecutor<Game>) {
    // Create the console entity so the console can receive messages
    let mut console = EntityBuilder::new();
    console
        .add(Console)
        .add(Name::new("Console"))
        .add(ChatBox::new(ChatPreference::All));

    // We can use the raw spawn method because
    // the console isn't a "normal" entity.
    game.ecs.spawn(console.build());

    systems
        .add_system(flush_console_chat_box)
        .add_system(flush_stdout)
        .add_system(flush_console_commands)
        .add_system(tab_complete_console)
        .group::<Server>()
        .add_system(flush_chat_boxes)
        .add_system(flush_title_chat_boxes);
}

/// Flushes players' chat mailboxes and sends the needed packets.
fn flush_chat_boxes(game: &mut Game, server: &mut Server) -> SysResult {
    for (_, (&client_id, mailbox)) in game.ecs.query::<(&ClientId, &mut ChatBox)>().iter() {
        if let Some(client) = server.clients.get(client_id) {
            for message in mailbox.drain() {
                client.send_chat_message(message);
            }
        }
    }

    Ok(())
}

/// Prints chat messages to the console.
fn flush_console_chat_box(game: &mut Game) -> SysResult {
    for (console, (_, mailbox)) in game.ecs.query::<(&Console, &mut ChatBox)>().iter() {
        for message in mailbox.drain() {
            // TODO: translate components
            log::info!(
                "{}",
                message.text().clone().into_component().to_string(
                    &|translate, with| format!("{}{:?}", String::from(translate), with),
                    &|color| match color {
                        Color::DarkRed => "\x1B[31m".to_string(),
                        Color::Red => "\x1B[31;1m".to_string(),
                        Color::Gold => "\x1B[33m".to_string(),
                        Color::Yellow => "\x1B[33;1m".to_string(),
                        Color::DarkGreen => "\x1B[32m".to_string(),
                        Color::Green => "\x1B[32;1m".to_string(),
                        Color::Aqua => "\x1B[36;1m".to_string(),
                        Color::DarkAqua => "\x1B[36m".to_string(),
                        Color::DarkBlue => "\x1B[34m".to_string(),
                        Color::Blue => "\x1B[34;1m".to_string(),
                        Color::LightPurple => "\x1B[31;5m".to_string(),
                        Color::DarkPurple => "\x1B[35m".to_string(),
                        Color::White => "\x1B[37;1m".to_string(),
                        Color::Gray => "\x1B[37;1m".to_string(),
                        Color::DarkGray => "\x1B[30m".to_string(),
                        Color::Black => "\x1B[30m".to_string(),
                        Color::Custom(rgb) => format!(
                            "\033[38;2;{};{};{}m",
                            u8::from_str_radix(&rgb[1..3], 16).unwrap(),
                            u8::from_str_radix(&rgb[3..5], 16).unwrap(),
                            u8::from_str_radix(&rgb[5..7], 16).unwrap()
                        ),
                    },
                    &|style| match style {
                        Style::Bold => "\x1B[1m".to_string(),
                        Style::Italic => "\x1B[3m".to_string(),
                        Style::Underlined => "\x1B[4m".to_string(),
                        Style::Strikethrough => "\x1B[9m".to_string(),
                        Style::Obfuscated => "".to_string(),
                    },
                    &|selector| {
                        let s = EntityArgument::ENTITIES.parse(selector, false);
                        if let Some((_, s)) = s {
                            feather_commands::utils::get_entity_names(console, game, &s).join(", ")
                        // TODO gray commas
                        } else {
                            String::new()
                        }
                    },
                    "\x1B[0m"
                )
            );
        }
    }

    Ok(())
}

/// Prints chat messages to the console.
fn flush_console_commands(game: &mut Game) -> SysResult {
    for command in game.resources().get::<ConsoleInput>().unwrap().try_iter() {
        log::debug!("Command: {}", command);
        let console = game.ecs.query::<&Console>().iter().next().unwrap().0;
        let _result = feather_commands::dispatch_command(
            &*game
                .resources()
                .get::<CommandDispatcher<CommandCtx>>()
                .unwrap(),
            game,
            console,
            &command,
            true,
        );
    }

    Ok(())
}

fn flush_title_chat_boxes(game: &mut Game, server: &mut Server) -> SysResult {
    for (_, (&client_id, mailbox)) in game.ecs.query::<(&ClientId, &mut ChatBox)>().iter() {
        if let Some(client) = server.clients.get(client_id) {
            for message in mailbox.drain_titles() {
                client.send_title(message);
            }
        }
    }

    Ok(())
}

fn flush_stdout(game: &mut Game) -> SysResult {
    game.resources.get::<ConsoleInput>().unwrap().flush_stdout();
    Ok(())
}

/// Prints chat messages to the console.
fn tab_complete_console(game: &mut Game) -> SysResult {
    let console = game.ecs.query::<&Console>().iter().next().unwrap().0;
    game.resources()
        .get::<ConsoleInput>()
        .unwrap()
        .tab_complete_if_needed(game, console);

    Ok(())
}
