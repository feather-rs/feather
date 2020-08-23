use feather_ecs::{Stage, SysResult, SystemExecutor};

fn system1(x: &mut i32) -> SysResult {
    *x += 10;
    Ok(())
}

fn system2(x: &mut i32) -> SysResult {
    *x *= 10;
    Ok(())
}

#[test]
fn systems_are_executed_in_order() {
    let mut executor = SystemExecutor::new();
    executor
        .add_system(Stage::Tick, system1)
        .add_system(Stage::Tick, system2);

    let mut x = 0;
    executor.tick(&mut x);

    assert_eq!(x, 100);
}
