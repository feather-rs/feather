use quill_ecs::*;

#[test]
fn insert_and_get() {
    let mut ecs = Ecs::new();

    let entity = EntityBuilder::new()
        .add(10i32)
        .add("name")
        .spawn_into(&mut ecs);

    assert_eq!(*ecs.get::<i32>(entity).unwrap(), 10);
    assert_eq!(*ecs.get::<&'static str>(entity).unwrap(), "name");
    assert!(ecs.get::<u64>(entity).is_err());
}

#[test]
fn spawn_many_entities() {
    let mut ecs = Ecs::new();

    let mut entities = Vec::new();
    for i in 0..10_000 {
        let entity = EntityBuilder::new()
            .add(10.0f32)
            .add(format!("Entity #{}", i))
            .spawn_into(&mut ecs);
        entities.push(entity);
    }

    for (i, entity) in entities.into_iter().enumerate() {
        assert_eq!(*ecs.get::<f32>(entity).unwrap(), 10.0);
        assert_eq!(
            *ecs.get::<String>(entity).unwrap(),
            format!("Entity #{}", i)
        );
    }
}

#[test]
fn zero_sized_components() {
    let mut ecs = Ecs::new();

    #[derive(PartialEq, Debug)]
    struct ZeroSized;

    let entity = ecs.spawn_bundle((ZeroSized,));

    assert_eq!(*ecs.get::<ZeroSized>(entity).unwrap(), ZeroSized);
}

#[test]
fn remove_components() {
    let mut ecs = Ecs::new();

    let entity1 = ecs.spawn_bundle((10i32, "string"));
    let entity2 = ecs.spawn_bundle((15i32, "string2"));

    ecs.remove::<i32>(entity1).unwrap();
    assert!(ecs.get::<i32>(entity1).is_err());
    assert_eq!(*ecs.get::<i32>(entity2).unwrap(), 15);
}

#[test]
fn remove_components_large_storage() {
    let mut ecs = Ecs::new();

    let mut entities: Vec<EntityId> = (0..10_000usize).map(|i| ecs.spawn_bundle((i,))).collect();

    let removed_entity = entities.remove(5000);
    ecs.remove::<usize>(removed_entity).unwrap();
    assert!(ecs.get::<usize>(removed_entity).is_err());

    for (i, entity) in entities.into_iter().enumerate() {
        let i = if i >= 5000 { i + 1 } else { i };
        assert_eq!(*ecs.get::<usize>(entity).unwrap(), i);
    }
}

#[test]
fn remove_nonexisting() {
    let mut ecs = Ecs::new();

    let entity = ecs.spawn_bundle((10i32,));
    assert!(ecs.remove::<usize>(entity).is_err());
}
