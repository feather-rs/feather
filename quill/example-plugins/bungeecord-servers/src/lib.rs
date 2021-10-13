use std::io::{Cursor, ErrorKind, Read, Write};

use commands::arguments::*;

use anyhow::bail;
use quill::entities::Player;
use quill::events::PluginMessageReceiveEvent;
use quill::{Caller, Game, Plugin, Setup, Text, TextComponentBuilder};

const BUNGEECORD: &str = "bungeecord:main";

quill::plugin!(BungeecordServersPlugin);

struct BungeecordServersPlugin {
    servers: Option<Vec<String>>,
    server: Option<String>,
    tick_counter: Option<usize>,
}

impl Plugin for BungeecordServersPlugin {
    fn enable(_game: &mut Game, setup: &mut Setup<Self>) -> Self {
        let plugin = BungeecordServersPlugin {
            servers: None,
            server: None,
            tick_counter: Some(0),
        };

        setup.add_system(get_servers);

        setup.with_dispatcher(move |dispatcher| {
            dispatcher.register_tab_completion("bungeecord_servers:servers_without_this", Box::new(move |text, context| {
                context.plugin.servers
                    .iter()
                    .flatten()
                    .filter(|&s| Some(s) != context.plugin.server.as_ref() && s.starts_with(text))
                    .map(|s| (s.clone(), None))
                    .collect()
            }));
            dispatcher.create_command("test").unwrap()
                .argument("server", StringArgument::GREEDY_PHRASE, "bungeecord_servers:servers_without_this")
                .executes(|context: CommandContext, target_server| {
                    let (server, servers) = (context.plugin.server.as_ref(), context.plugin.servers.as_ref());
                    if server.is_some() && *server.unwrap() == target_server {
                        context.caller.send_message(format!("You are already on {}", target_server));
                        bail!("Already on this server")
                    } else if servers.is_some() && servers.unwrap().contains(&target_server) {
                        context.caller.send_message(format!("Sending you to {}", target_server));
                        if let Caller::Player(entity) = context.caller {
                            context.game.send_plugin_message(entity.id(), BUNGEECORD, &{
                                let mut vec = Vec::new();
                                write_str("Connect", &mut vec)?;
                                write_str(&target_server, &mut vec)?;
                                vec
                            }[..]);
                            Ok(1)
                        } else {
                            context.caller.send_message(Text::translate("permissions.requires.player").red());
                            bail!("Requires a player")
                        }
                    } else {
                        context.caller.send_message(
                            Text::of(format!("Server \"{}\" doesn't exist! Choose one of these:", target_server))
                                .extra(servers
                                    .map(|servers| servers
                                        .iter()
                                        .filter(|s| server.is_none() || server.unwrap() != *s)
                                        .map(|server| Text::of(format!(" [{}]", server))
                                            .on_hover_show_text("Click here!")
                                            .on_click_run_command(format!("/test {}", server)))
                                        .collect())
                                    .unwrap_or_else(|| vec![Text::of(" (not found any servers). Are you running a bungeecord server?")])));
                        bail!("Server doesn't exist")
                    }
                });
        });

        plugin
    }

    fn disable(self, _game: &mut Game) {}
}

fn get_servers(plugin: &mut BungeecordServersPlugin, game: &mut Game) {
    if plugin.servers.is_none() || plugin.server.is_none() {
        if let Some(a) = plugin.tick_counter.as_mut() {
            *a += 1;
        }
        if plugin.tick_counter.unwrap() % 10 == 0 {
            if let Some((player, _)) = game.query::<&Player>().next() {
                if plugin.servers.is_none() {
                    game.send_plugin_message(
                        player.id(),
                        BUNGEECORD,
                        &{
                            let mut vec = Vec::new();
                            write_str("GetServers", &mut vec).unwrap();
                            vec
                        }[..],
                    );
                }
                if plugin.server.is_none() {
                    game.send_plugin_message(
                        player.id(),
                        BUNGEECORD,
                        &{
                            let mut vec = Vec::new();
                            write_str("GetServer", &mut vec).unwrap();
                            vec
                        }[..],
                    );
                }
            }
        }
        for (_, plugin_message) in game.query::<&PluginMessageReceiveEvent>() {
            if plugin_message.channel.as_str() == BUNGEECORD {
                let mut data = Cursor::new(plugin_message.data);
                match read_string(&mut data).unwrap().as_str() {
                    "GetServers" => {
                        plugin.servers = Some(
                            read_string(&mut data)
                                .unwrap()
                                .split(", ")
                                .map(ToOwned::to_owned)
                                .collect(),
                        );
                    }
                    "GetServer" => {
                        plugin.server = Some(read_string(&mut data).unwrap());
                    }
                    _ => (),
                }
            }
        }
    }
}

pub fn write_str(str: &str, mut out: impl Write) -> std::io::Result<()> {
    out.write_all(&(str.len() as u16).to_be_bytes())?;
    out.write_all(str.as_bytes())?;
    Ok(())
}

pub fn read_string(mut input: impl Read) -> std::io::Result<String> {
    let mut buf = [0; 2];
    input.read_exact(&mut buf)?;
    let len = u16::from_be_bytes(buf) as usize;
    let mut buf = vec![0; len];
    input.read_exact(&mut buf)?;
    String::from_utf8(buf).map_err(|err| std::io::Error::new(ErrorKind::InvalidInput, err))
}
