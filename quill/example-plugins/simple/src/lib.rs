use quill::{
    components::{CustomName, Name},
    entities::Cow,
    EntityKind, Game, Gamemode, Plugin, Position, Setup, Uuid,
};
use rand::Rng;

quill::plugin!(SimplePlugin);

struct SimplePlugin {
    tick_counter: u64,
}

impl Plugin for SimplePlugin {
    fn enable(_game: &mut Game, setup: &mut Setup<Self>) -> Self {
        setup.add_system(test_system);
        SimplePlugin { tick_counter: 0 }
    }

    fn disable(self, _game: &mut Game) {}
}

fn test_system(plugin: &mut SimplePlugin, game: &mut Game) {
    for (entity, (position, name, gamemode, uuid)) in
        game.query::<(&Position, &Name, &Gamemode, &Uuid)>()
    {
        entity.send_message(format!(
            "[{}] Hi {}. Your gamemode is {:?} and your position is {:.1?} and your UUID is {}",
            plugin.tick_counter,
            name,
            gamemode,
            position,
            uuid.to_hyphenated()
        ));

        if plugin.tick_counter % 100 == 0 {
            entity.send_message("Spawning a mob on you");
            game.create_entity_builder(position, random_mob())
                .with(CustomName::new("Custom name"))
                .finish();
        }
    }
    for (_, (mut position, _)) in game.query::<(&mut Position, &Cow)>() {
        position.y += 0.1;
    }

    plugin.tick_counter += 1;
}

fn random_mob() -> EntityKind {
    let mut entities = vec![
        EntityKind::Zombie,
        EntityKind::Piglin,
        EntityKind::Zoglin,
        EntityKind::Skeleton,
        EntityKind::Enderman,
        EntityKind::Cow,
    ];
    let index = rand::thread_rng().gen_range(0..entities.len());
    entities.remove(index)
}
