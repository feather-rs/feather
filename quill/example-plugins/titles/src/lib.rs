use quill::{entities::Player, Game, Plugin, Text, TextComponent, TextComponentBuilder, Title};

#[quill::plugin]
struct TitleExample {
    tick_count: u32,
    title_active: bool,
}

impl Plugin for TitleExample {
    fn enable(_game: &mut quill::Game, setup: &mut quill::Setup<Self>) -> Self {
        setup.add_system(title_system);

        TitleExample {
            tick_count: 0,
            title_active: false,
        }
    }

    fn disable(self, _game: &mut quill::Game) {}
}

fn title_system(plugin: &mut TitleExample, game: &mut Game) {
    // Run once every 100 ticks (5 seconds)
    if plugin.tick_count % 100 == 0 && !plugin.title_active {
        let component = TextComponentBuilder::gray(TextComponent::from("Wicked fast Minecraft!"));
        let title_component = TextComponentBuilder::white(TextComponent::from("Hello Feather!"));

        // Create a title to send to the player
        let title = Title {
            title: Some(Text::from(title_component)),
            sub_title: Some(Text::from(component)),
            fade_in: 5,
            stay: 400,
            fade_out: 5,
        };

        // Send the title to all online players
        for (entity, _) in game.query::<&Player>() {
            entity.send_title(&title);
            plugin.title_active = true;
        }
    }

    if plugin.tick_count % 250 == 0 && plugin.title_active {
        // Reset the title for all players
        for (entity, _) in game.query::<&Player>() {
            entity.reset_title();
            plugin.title_active = false;
        }
    }

    plugin.tick_count += 1;
}
