use shrev::EventChannel;
use specs::{Entity, Read, ReaderId, System, SystemData, World};

use feather_core::{Item, Position};

use crate::util::Util;

/// Event triggered when arrow is shot.
#[derive(Debug, Clone)]
pub struct ShootArrowEvent {
    pub arrow_type: Item,
    pub shooter: Option<Entity>,
    pub position: Position,
    pub critical: bool,
}

#[derive(Default)]
pub struct ShootArrowSystem {
    reader: Option<ReaderId<ShootArrowEvent>>,
}

impl<'a> System<'a> for ShootArrowSystem {
    type SystemData = (Read<'a, Util>, Read<'a, EventChannel<ShootArrowEvent>>);

    fn run(&mut self, data: Self::SystemData) {
        let (util, shoot_arrow_events) = data;

        for event in shoot_arrow_events.read(self.reader.as_mut().unwrap()) {
            debug!("Shooting an arrow.");
        }
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        self.reader = Some(world.fetch_mut::<EventChannel<_>>().register_reader());
    }
}
