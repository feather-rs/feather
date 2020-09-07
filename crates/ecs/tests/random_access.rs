use feather_ecs::{ComponentError, Ecs, EntityBuilder};

#[test]
fn add_simple_entity() {
    let mut ecs = Ecs::new();
    let entity = ecs.inner_mut().spawn(
        EntityBuilder::new()
            .add(10i32)
            .add(15u32)
            .add(usize::MAX)
            .build(),
    );

    assert_eq!(*ecs.get::<i32>(entity).unwrap(), 10);
    assert_eq!(*ecs.get::<u32>(entity).unwrap(), 15);
    assert_eq!(*ecs.get::<usize>(entity).unwrap(), usize::MAX);

    *ecs.get_mut::<i32>(entity).unwrap() = 324;
    assert_eq!(*ecs.get::<i32>(entity).unwrap(), 324);
}

#[test]
fn add_remove_entities() {
    let mut ecs = Ecs::new();
    let entity = ecs
        .inner_mut()
        .spawn(EntityBuilder::new().add("test").build());

    assert_eq!(*ecs.get::<&'static str>(entity).unwrap(), "test");

    ecs.inner_mut().despawn(entity).unwrap();

    assert!(matches!(
        ecs.get::<&'static str>(entity).err(),
        Some(ComponentError::NoSuchEntity)
    ));
    assert!(ecs.inner_mut().despawn(entity).is_err());
}

#[test]
fn fine_grained_borrow_checking() {
    let mut ecs = Ecs::new();
    let entity1 = ecs
        .inner_mut()
        .spawn(EntityBuilder::new().add(()).add(16i32).build());
    let entity2 = ecs
        .inner_mut()
        .spawn(EntityBuilder::new().add(14i32).build());

    let mut e1 = ecs.get_mut::<i32>(entity1).unwrap();
    let e2 = ecs.get_mut::<i32>(entity2).unwrap();

    assert_eq!(*e1, 16);
    assert_eq!(*e2, 14);

    *e1 = 12;
    assert_eq!(*e1, 12);

    drop(e1);
    assert_eq!(*ecs.get_mut::<i32>(entity1).unwrap(), 12);

    assert!(ecs.get_mut::<()>(entity1).is_ok());
}
