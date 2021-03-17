use std::collections::HashMap;

use quill_ecs::*;

#[test]
fn insert_and_get() {
    let mut world = World::new();

    let entity = EntityBuilder::new()
        .add(10i32)
        .add("name")
        .spawn_into(&mut world);

    assert_eq!(*world.get::<i32>(entity).unwrap(), 10);
    assert_eq!(*world.get::<&'static str>(entity).unwrap(), "name");
    assert!(world.get::<u64>(entity).is_err());
}

#[test]
fn spawn_many_entities() {
    let mut world = World::new();

    let mut entities = Vec::new();
    for i in 0..10_000 {
        let entity = EntityBuilder::new()
            .add(10i32)
            .add(format!("Entity #{}", i))
            .spawn_into(&mut world);
        entities.push(entity);
    }

    for (i, entity) in entities.into_iter().enumerate() {
        assert_eq!(*world.get::<i32>(entity).unwrap(), 10);
        assert_eq!(
            *world.get::<String>(entity).unwrap(),
            format!("Entity #{}", i)
        );
    }
}

#[test]
fn zero_sized_components() {
    let mut world = World::new();

    #[derive(PartialEq, Debug)]
    struct ZeroSized;

    let entity = world.spawn_bundle((ZeroSized,));

    assert_eq!(*world.get::<ZeroSized>(entity).unwrap(), ZeroSized);
}

#[test]
fn remove_components() {
    let mut world = World::new();

    let entity1 = world.spawn_bundle((10i32, "string"));
    let entity2 = world.spawn_bundle((15i32, "string2"));

    world.remove::<i32>(entity1).unwrap();
    assert!(world.get::<i32>(entity1).is_err());
    assert_eq!(*world.get::<i32>(entity2).unwrap(), 15);
}

#[test]
fn remove_components_large_storage() {
    let mut world = World::new();

    let mut entities: Vec<EntityId> = (0..10_000usize).map(|i| world.spawn_bundle((i,))).collect();

    let removed_entity = entities.remove(5000);
    world.remove::<usize>(removed_entity).unwrap();
    assert!(world.get::<usize>(removed_entity).is_err());

    for (i, entity) in entities.into_iter().enumerate() {
        let i = if i >= 5000 { i + 1 } else { i };
        assert_eq!(*world.get::<usize>(entity).unwrap(), i);
    }
}

#[test]
fn remove_nonexisting() {
    let mut world = World::new();

    let entity = world.spawn_bundle((10i32,));
    assert!(world.remove::<usize>(entity).is_err());
}

#[test]
fn query_basic() {
    let mut world = World::new();

    let entity1 = world.spawn_bundle((10i32, "name1"));
    let entity2 = world.spawn_bundle((15i32, "name2", 50.0f32));

    let mut query = world.query::<(&i32, &&'static str)>();
    let mut iter = query.iter();

    assert_eq!(iter.next(), Some((entity1, (&10, &"name1"))));
    assert_eq!(iter.next(), Some((entity2, (&15, &"name2"))));
    assert_eq!(iter.next(), None);
}

#[test]
fn query_big_ecs_after_despawn() {
    let mut world = World::new();

    let mut entities = Vec::new();
    for i in 0..100usize {
        let mut builder = EntityBuilder::new();
        if i % 3 == 0 {
            builder.add(format!("entity #{}", i));
        }
        builder.add(i);
        let entity = builder.spawn_into(&mut world);
        if i % 3 == 0 {
            entities.push(entity);
        }
    }

    let last = entities.len() - 1;
    world.despawn(entities.remove(last)).unwrap();

    let queried: HashMap<EntityId, (&String, &usize)> =
        world.query::<(&String, &usize)>().iter().collect();

    for (i, entity) in entities.iter().copied().enumerate() {
        assert_eq!(queried[&entity], (&format!("entity #{}", i * 3), &(i * 3)));
    }

    assert_eq!(queried.len(), entities.len());
}

#[test]
fn empty_query() {
    let world = World::new();

    assert_eq!(world.query::<&i32>().iter().count(), 0);
}
