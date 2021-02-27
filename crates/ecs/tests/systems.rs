#![allow(clippy::unnecessary_wraps)]

use feather_ecs::{Ecs, HasEcs, SysResult, SystemExecutor};

struct Input {
    x: i32,
    ecs: Ecs,
}

impl HasEcs for Input {
    fn ecs(&self) -> &Ecs {
        &self.ecs
    }

    fn ecs_mut(&mut self) -> &mut Ecs {
        &mut self.ecs
    }
}

fn system1(input: &mut Input) -> SysResult {
    input.x += 10;
    Ok(())
}

fn system2(input: &mut Input) -> SysResult {
    input.x *= 10;
    Ok(())
}

#[test]
fn systems_are_executed_in_order() {
    let mut executor = SystemExecutor::new();
    executor.add_system(system1);
    executor.add_system(system2);

    let mut input = Input {
        x: 1,
        ecs: Ecs::new(),
    };
    executor.run(&mut input);
    assert_eq!(input.x, 110);
}
