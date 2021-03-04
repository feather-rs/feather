use feather_core::text::{Color, TextRoot, Translate};
use feather_server_types::{ChatEvent, ChatPosition, Game, Name, PlayerJoinEvent};
use fecs::World;

#[fecs::event_handler]
pub fn on_player_join_broadcast_join_message(
    event: &PlayerJoinEvent,
    game: &mut Game,
    world: &mut World,
) {
    let message: String = {
        let name = world.get::<Name>(event.player);
        TextRoot::from(
            Translate::MultiplayerPlayerJoined * vec![name.0.to_string()] * Color::Yellow,
        )
        .into()
    };

    game.handle(
        world,
        ChatEvent {
            message,
            position: ChatPosition::Chat,
        },
    );
}
