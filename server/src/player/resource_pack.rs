//! System for sending resource pack to new players.

use crate::config::Config;
use crate::joinhandler::PlayerJoinEvent;
use crate::network::{send_packet_to_player, NetworkComponent};
use feather_core::network::packet::implementation::ResourcePackSend;
use shrev::{EventChannel, ReaderId};
use specs::{Read, ReadStorage, System};
use std::sync::Arc;

/// System for sending resource pack to new players,
/// if enabled.
///
/// This system listens to `PlayerJoinEvent`s.
#[derive(Default)]
pub struct ResourcePackSendSystem {
    reader: Option<ReaderId<PlayerJoinEvent>>,
}

impl<'a> System<'a> for ResourcePackSendSystem {
    type SystemData = (
        ReadStorage<'a, NetworkComponent>,
        Read<'a, Arc<Config>>,
        Read<'a, EventChannel<PlayerJoinEvent>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (networks, config, join_events) = data;
        if config.resource_pack.url.is_empty() {
            return; // Resource pack not enabled
        }

        for event in join_events.read(self.reader.as_mut().unwrap()) {
            let network = networks.get(event.player).unwrap();

            let packet = ResourcePackSend {
                url: config.resource_pack.url.clone(),
                hash: config.resource_pack.hash.to_lowercase(),
            };

            send_packet_to_player(&network, packet);
        }
    }

    setup_impl!(reader);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testframework as t;
    use feather_core::network::cast_packet;
    use feather_core::network::packet::PacketType;
    use specs::WorldExt;

    #[test]
    fn test_resource_pack_send_system() {
        let (mut w, mut d) = t::init_world();

        let player = t::add_player(&mut w);

        let event = PlayerJoinEvent {
            player: player.entity,
        };
        t::trigger_event(&w, event);

        let url = "https://rust-lang.org/".to_string();
        let hash = "bLa".to_string();

        {
            let mut config = Config::clone(&w.fetch::<Arc<Config>>());
            config.resource_pack.url = url.clone();
            config.resource_pack.hash = hash.clone();
            w.insert(Arc::new(config));
        }

        d.dispatch(&w);
        w.maintain();

        let packet = t::assert_packet_received(&player, PacketType::ResourcePackSend);
        let packet = cast_packet::<ResourcePackSend>(&*packet);

        assert_eq!(packet.url, url);
        assert_eq!(packet.hash, hash.to_lowercase());
    }
}
