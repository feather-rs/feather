use feather_core::network::packets::NamedSoundEffect;
use feather_server_types::{Game, NamedSoundEffectEvent};
use fecs::World;
use feather_core::util::{ChunkPosition};

/// Broadcasts sounds.
#[fecs::event_handler]
pub fn on_sound_broadcast(
	event: &NamedSoundEffectEvent,
	game: &mut Game,
	world: &mut World,
) {
	let packet = NamedSoundEffect {
		sound_name: event.sound_name.clone(),
		sound_category: event.sound_category,
		effect_pos_x: event.effect_pos.x,
		effect_pos_y: event.effect_pos.y,
		effect_pos_z: event.effect_pos.z,
		volume: event.volume,
		pitch: event.pitch
	};
	game.broadcast_chunk_update(
		world,
		packet,
		ChunkPosition::from(event.effect_pos),
		None
	);
}
