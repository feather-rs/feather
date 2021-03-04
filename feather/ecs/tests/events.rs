#![allow(clippy::unnecessary_wraps)]

use feather_ecs::{Ecs, HasEcs, SysResult, SystemExecutor};

#[derive(Debug, PartialEq, Eq)]
struct Event {
    x: i32,
}

struct Input {
    ecs: Ecs,
    is_first_run: bool,
}

impl HasEcs for Input {
    fn ecs(&self) -> &Ecs {
        &self.ecs
    }

    fn ecs_mut(&mut self) -> &mut Ecs {
        &mut self.ecs
    }
}

fn pre_system(input: &mut Input) -> SysResult {
    if !input.is_first_run {
        let mut query = input.ecs.query::<&Event>();
        assert_eq!(query.iter().next().unwrap().1, &Event { x: 10 });
    }
    Ok(())
}

fn trigger_system(input: &mut Input) -> SysResult {
    if input.is_first_run {
        let entity = input.ecs.spawn(());
        let event = Event { x: 10 };
        input.ecs.insert_entity_event(entity, event)?;
    } else {
        let mut query = input.ecs.query::<&Event>();
        assert_eq!(query.iter().next(), None);
    }

    Ok(())
}

fn post_system(input: &mut Input) -> SysResult {
    let mut query = input.ecs.query::<&Event>();
    let next = query.iter().next();
    if input.is_first_run {
        assert_eq!(next.unwrap().1, &Event { x: 10 });
    } else {
        assert_eq!(next, None);
    }
    Ok(())
}

#[test]
fn events_observed_once() {
    let mut systems = SystemExecutor::<Input>::new();
    systems
        .add_system(pre_system)
        .add_system(trigger_system)
        .add_system(post_system);

    let mut input = Input {
        ecs: Ecs::new(),
        is_first_run: true,
    };
    systems.run(&mut input);
    input.is_first_run = false;
    systems.run(&mut input);

    assert_eq!(input.ecs.inner().len(), 1);
}
