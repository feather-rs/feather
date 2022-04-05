use std::collections::HashMap;

use vane::*;

#[test]
fn insert_and_get() {
    let mut world = Entities::new();

    let entity = EntityBuilder::new()
        .add(10i32)
        .add("name")
        .spawn_into(&mut world);

    assert_eq!(*world.get::<i32>(entity).unwrap(), 10);
    assert_eq!(*world.get::<&'static str>(entity).unwrap(), "name");
    assert!(world.get::<u64>(entity).is_err());
}

#[test]
#[cfg_attr(miri, ignore)]
fn spawn_many_entities() {
    let mut world = Entities::new();

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
    let mut world = Entities::new();

    #[derive(PartialEq, Debug)]
    struct ZeroSized;

    let entity = world.spawn_bundle((ZeroSized,));

    assert_eq!(*world.get::<ZeroSized>(entity).unwrap(), ZeroSized);
}

#[test]
fn remove_components() {
    let mut world = Entities::new();

    let entity1 = world.spawn_bundle((10i32, "string"));
    let entity2 = world.spawn_bundle((15i32, "string2"));

    world.remove::<i32>(entity1).unwrap();
    assert!(world.get::<i32>(entity1).is_err());
    assert_eq!(*world.get::<i32>(entity2).unwrap(), 15);
}

#[test]
#[cfg_attr(miri, ignore)]
fn remove_components_large_storage() {
    let mut world = Entities::new();

    let mut entities: Vec<Entity> = (0..10_000usize).map(|i| world.spawn_bundle((i,))).collect();

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
    let mut world = Entities::new();

    let entity = world.spawn_bundle((10i32,));
    assert!(world.remove::<usize>(entity).is_err());
}

#[test]
fn query_basic() {
    let mut world = Entities::new();

    let entity1 = world.spawn_bundle((10i32, "name1"));
    let entity2 = world.spawn_bundle((15i32, "name2", 50.0f32));

    let mut query = world.query::<(&i32, &&'static str)>();
    let mut iter = query.iter();

    let (entity, (i, name)) = iter.next().unwrap();
    assert_eq!(entity, entity1);
    assert_eq!(*i, 10);
    assert_eq!(*name, "name1");

    let (entity, (i, name)) = iter.next().unwrap();
    assert_eq!(entity, entity2);
    assert_eq!(*i, 15);
    assert_eq!(*name, "name2");

    assert!(iter.next().is_none());
}

#[test]
fn query_big_ecs_after_despawn() {
    let mut world = Entities::new();

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

    let queried: HashMap<Entity, (Ref<String>, Ref<usize>)> =
        world.query::<(&String, &usize)>().iter().collect();

    for (i, entity) in entities.iter().copied().enumerate() {
        let (name, number) = &queried[&entity];
        assert_eq!(*name, &format!("entity #{}", i * 3));
        assert_eq!(**number, i * 3);
    }

    assert_eq!(queried.len(), entities.len());
}

#[test]
fn empty_query() {
    let world = Entities::new();

    assert_eq!(world.query::<&i32>().iter().count(), 0);
}

#[test]
fn mutable_access() {
    let mut world = Entities::new();
    let entity = world.spawn_bundle((10i32,));
    *world.get_mut::<i32>(entity).unwrap() = 15;
    assert_eq!(*world.get::<i32>(entity).unwrap(), 15);
}

#[test]
fn borrow_conflict_mutable() {
    let mut world = Entities::new();
    let entity = world.spawn_bundle((10i32,));

    let mut reference = world.get_mut::<i32>(entity).unwrap();

    assert!(matches!(
        world.get::<i32>(entity),
        Err(ComponentError::BorrowConflict(_))
    ));
    *reference = 5;

    drop(reference);
    assert_eq!(*world.get::<i32>(entity).unwrap(), 5);
}

#[test]
fn borrow_conflict_shared() {
    let mut world = Entities::new();
    let entity = world.spawn_bundle((10i32,));

    let _reference = world.get::<i32>(entity).unwrap();
    assert!(matches!(
        world.get_mut::<i32>(entity),
        Err(ComponentError::BorrowConflict(_))
    ));
}

#[test]
fn too_many_shared_borrows() {
    let mut world = Entities::new();
    let entity = world.spawn_bundle((10i32,));

    let refs: Vec<_> = (0..254)
        .map(|_| world.get::<i32>(entity).unwrap())
        .collect();

    assert!(matches!(
        world.get::<i32>(entity),
        Err(ComponentError::BorrowConflict(_))
    ));
    assert!(matches!(
        world.get_mut::<i32>(entity),
        Err(ComponentError::BorrowConflict(_))
    ));

    drop(refs);

    assert_eq!(*world.get::<i32>(entity).unwrap(), 10);
}
