use feather_ecs::{ComponentError, Ecs, EntityBuilder};

#[test]
fn add_simple_entity() {
    let mut ecs = Ecs::new();
    let entity = EntityBuilder::new()
        .with(10i32)
        .with(15u32)
        .with(usize::MAX)
        .spawn_into(&mut ecs);

    assert_eq!(*ecs.get::<i32>(entity).unwrap(), 10);
    assert_eq!(*ecs.get::<u32>(entity).unwrap(), 15);
    assert_eq!(*ecs.get::<usize>(entity).unwrap(), usize::MAX);

    *ecs.get_mut::<i32>(entity).unwrap() = 324;
    assert_eq!(*ecs.get::<i32>(entity).unwrap(), 324);
}

#[test]
fn add_remove_entities() {
    let mut ecs = Ecs::new();
    let entity = EntityBuilder::new().with("test").spawn_into(&mut ecs);

    assert_eq!(*ecs.get::<&'static str>(entity).unwrap(), "test");

    ecs.despawn(entity).unwrap();

    assert!(matches!(
        ecs.get::<&'static str>(entity).err(),
        Some(ComponentError::InvalidEntity(_))
    ));
    assert!(ecs.despawn(entity).is_err());
}

#[test]
fn double_mutable_borrow() {
    let mut ecs = Ecs::new();
    let entity = EntityBuilder::new().with(10i32).spawn_into(&mut ecs);

    let ref1 = ecs.get_mut::<i32>(entity).unwrap();

    assert!(matches!(
        ecs.get_mut::<i32>(entity),
        Err(ComponentError::Borrowed(_))
    ));
    assert!(matches!(
        ecs.get::<i32>(entity),
        Err(ComponentError::MutablyBorrowed(_))
    ));

    drop(ref1);
    assert!(ecs.get::<i32>(entity).is_ok());
}

#[test]
fn fine_grained_borrow_checking() {
    let mut ecs = Ecs::new();
    let entity1 = EntityBuilder::new()
        .with(())
        .with(16i32)
        .spawn_into(&mut ecs);
    let entity2 = EntityBuilder::new().with(14i32).spawn_into(&mut ecs);

    let mut e1 = ecs.get_mut::<i32>(entity1).unwrap();
    let e2 = ecs.get_mut::<i32>(entity2).unwrap();

    assert_eq!(*e1, 16);
    assert_eq!(*e2, 14);

    *e1 = 12;
    assert_eq!(*e1, 12);

    assert!(ecs.get_mut::<i32>(entity1).is_err());
    assert!(ecs.get::<i32>(entity1).is_err());

    drop(e1);
    assert_eq!(*ecs.get_mut::<i32>(entity1).unwrap(), 12);

    assert!(ecs.get_mut::<()>(entity1).is_ok());
}
