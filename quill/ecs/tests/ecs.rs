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
