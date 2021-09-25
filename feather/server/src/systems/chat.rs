use common::{chat::ChatPreference, ChatBox, Game};
use ecs::{EntityBuilder, SysResult, SystemExecutor};

use crate::{ClientId, Server};

/// Marker component for the console entity.
struct Console;

pub fn register(game: &mut Game, systems: &mut SystemExecutor<Game>) {
    // Create the console entity so the console can receive messages
    let mut console = EntityBuilder::new();
    console.add(Console).add(ChatBox::new(ChatPreference::All));

    // We can use the raw spawn method because
    // the console isn't a "normal" entity.
    game.ecs.spawn(console.build());

    systems.add_system(flush_console_chat_box);
    systems.group::<Server>().add_system(flush_chat_boxes);
    systems.group::<Server>().add_system(flush_title_chat_boxes);
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
    for (_, (_console, mailbox)) in game.ecs.query::<(&Console, &mut ChatBox)>().iter() {
        for message in mailbox.drain() {
            // TODO: properly display chat message
            log::info!("{:?}", message.text());
        }
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
