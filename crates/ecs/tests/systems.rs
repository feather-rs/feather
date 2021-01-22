use feather_ecs::{SysResult, SystemExecutor};

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
    executor.add_system(system1);
    executor.add_system(system2);

    let mut x = 1;
    executor.run(&mut x);
    assert_eq!(x, 110);
}
