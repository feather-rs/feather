use std::collections::HashMap;

use vane::*;

struct Comp<T>(T);

impl<T: 'static> vane::Component for Comp<T> {}

#[test]
fn insert_and_get() {
    let mut world = Entities::new();

    let entity = EntityBuilder::new()
        .add(Comp(10i32))
        .add(Comp("name"))
        .spawn_into(&mut world);

    assert_eq!(world.get::<Comp<i32>>(entity).unwrap().0, 10);
    assert_eq!(world.get::<Comp<&'static str>>(entity).unwrap().0, "name");
    assert!(world.get::<Comp<u64>>(entity).is_err());
}

#[test]
#[cfg_attr(miri, ignore)]
fn spawn_many_entities() {
    let mut world = Entities::new();

    let mut entities = Vec::new();
    for i in 0..10_000 {
        let entity = EntityBuilder::new()
            .add(Comp(10i32))
            .add(Comp(format!("Entity #{}", i)))
            .spawn_into(&mut world);
        entities.push(entity);
    }

    for (i, entity) in entities.into_iter().enumerate() {
        assert_eq!(world.get::<Comp<i32>>(entity).unwrap().0, 10);
        assert_eq!(
            world.get::<Comp<String>>(entity).unwrap().0,
            format!("Entity #{}", i)
        );
    }
}

#[test]
fn zero_sized_components() {
    let mut world = Entities::new();

    #[derive(PartialEq, Debug)]
    struct ZeroSized;

    let entity = world.spawn_bundle((Comp(ZeroSized),));

    assert_eq!(world.get::<Comp<ZeroSized>>(entity).unwrap().0, ZeroSized);
}

#[test]
fn remove_components() {
    let mut world = Entities::new();

    let entity1 = world.spawn_bundle((Comp(10i32), Comp("string")));
    let entity2 = world.spawn_bundle((Comp(15i32), Comp("string2")));

    world.remove::<Comp<i32>>(entity1).unwrap();
    assert!(world.get::<Comp<i32>>(entity1).is_err());
    assert_eq!(world.get::<Comp<i32>>(entity2).unwrap().0, 15);
}

#[test]
#[cfg_attr(miri, ignore)]
fn remove_components_large_storage() {
    let mut world = Entities::new();

    let mut entities: Vec<Entity> = (0..10_000usize)
        .map(|i| world.spawn_bundle((Comp(i),)))
        .collect();

    let removed_entity = entities.remove(5000);
    world.remove::<Comp<usize>>(removed_entity).unwrap();
    assert!(world.get::<Comp<usize>>(removed_entity).is_err());

    for (i, entity) in entities.into_iter().enumerate() {
        let i = if i >= 5000 { i + 1 } else { i };
        assert_eq!(world.get::<Comp<usize>>(entity).unwrap().0, i);
    }
}

#[test]
fn remove_nonexisting() {
    let mut world = Entities::new();

    let entity = world.spawn_bundle((Comp(10i32),));
    assert!(world.remove::<Comp<usize>>(entity).is_err());
}

#[test]
fn query_basic() {
    let mut world = Entities::new();

    let entity1 = world.spawn_bundle((Comp(10i32), Comp("name1")));
    let entity2 = world.spawn_bundle((Comp(15i32), Comp("name2"), Comp(50.0f32)));

    let mut query = world.query::<(&Comp<i32>, &Comp<&'static str>)>();
    let mut iter = query.iter();

    let (entity, (i, name)) = iter.next().unwrap();
    assert_eq!(entity, entity1);
    assert_eq!(i.0, 10);
    assert_eq!(name.0, "name1");

    let (entity, (i, name)) = iter.next().unwrap();
    assert_eq!(entity, entity2);
    assert_eq!(i.0, 15);
    assert_eq!(name.0, "name2");

    assert!(iter.next().is_none());
}

#[test]
fn query_big_ecs_after_despawn() {
    let mut world = Entities::new();

    let mut entities = Vec::new();
    for i in 0..100usize {
        let mut builder = EntityBuilder::new();
        if i % 3 == 0 {
            builder.add(Comp(format!("entity #{}", i)));
        }
        builder.add(Comp(i));
        let entity = builder.spawn_into(&mut world);
        if i % 3 == 0 {
            entities.push(entity);
        }
    }

    let last = entities.len() - 1;
    world.despawn(entities.remove(last)).unwrap();

    let queried: HashMap<Entity, (Ref<Comp<String>>, Ref<Comp<usize>>)> = world
        .query::<(&Comp<String>, &Comp<usize>)>()
        .iter()
        .collect();

    for (i, entity) in entities.iter().copied().enumerate() {
        let (name, number) = &queried[&entity];
        assert_eq!(name.0, format!("entity #{}", i * 3));
        assert_eq!(number.0, i * 3);
    }

    assert_eq!(queried.len(), entities.len());
}

#[test]
fn empty_query() {
    let world = Entities::new();

    assert_eq!(world.query::<&Comp<i32>>().iter().count(), 0);
}

#[test]
fn mutable_access() {
    let mut world = Entities::new();
    let entity = world.spawn_bundle((Comp(10i32),));
    world.get_mut::<Comp<i32>>(entity).unwrap().0 = 15;
    assert_eq!(world.get::<Comp<i32>>(entity).unwrap().0, 15);
}

#[test]
fn borrow_conflict_mutable() {
    let mut world = Entities::new();
    let entity = world.spawn_bundle((Comp(10i32),));

    let mut reference = world.get_mut::<Comp<i32>>(entity).unwrap();

    assert!(matches!(
        world.get::<Comp<i32>>(entity),
        Err(ComponentError::BorrowConflict(_))
    ));
    reference.0 = 5;

    drop(reference);
    assert_eq!(world.get::<Comp<i32>>(entity).unwrap().0, 5);
}

#[test]
fn borrow_conflict_shared() {
    let mut world = Entities::new();
    let entity = world.spawn_bundle((Comp(10i32),));

    let _reference = world.get::<Comp<i32>>(entity).unwrap();
    assert!(matches!(
        world.get_mut::<Comp<i32>>(entity),
        Err(ComponentError::BorrowConflict(_))
    ));
}

#[test]
fn too_many_shared_borrows() {
    let mut world = Entities::new();
    let entity = world.spawn_bundle((Comp(10i32),));

    let refs: Vec<_> = (0..254)
        .map(|_| world.get::<Comp<i32>>(entity).unwrap())
        .collect();

    assert!(matches!(
        world.get::<Comp<i32>>(entity),
        Err(ComponentError::BorrowConflict(_))
    ));
    assert!(matches!(
        world.get_mut::<Comp<i32>>(entity),
        Err(ComponentError::BorrowConflict(_))
    ));

    drop(refs);

    assert_eq!(world.get::<Comp<i32>>(entity).unwrap().0, 10);
}

#[test]
fn query_after_removing() {
    let mut world = Entities::new();

    let entity = world.spawn_bundle((Comp(0i32), Comp("foo")));

    assert_eq!(world.query::<&Comp<i32>>().iter().count(), 1);

    world.remove::<Comp<i32>>(entity).unwrap();

    assert_eq!(world.query::<&Comp<i32>>().iter().count(), 0);

    for _ in 0..100 {
        world.spawn_bundle((Comp("foo"),));
    }

    let entity = world.spawn_bundle((Comp(0i32), Comp("foo")));

    assert_eq!(world.query::<&Comp<i32>>().iter().count(), 1);

    world.remove::<Comp<i32>>(entity).unwrap();

    assert_eq!(world.query::<&Comp<i32>>().iter().count(), 0);
}

#[test]
fn insert_twice_overwrites() {
    let mut world = Entities::new();

    let entity = world.spawn_empty();

    world.insert(entity, Comp(0i32)).unwrap();
    assert_eq!(world.query::<&Comp<i32>>().iter().count(), 1);

    world.insert(entity, Comp(1i32)).unwrap();
    assert_eq!(world.query::<&Comp<i32>>().iter().count(), 1);
    assert_eq!(world.get::<Comp<i32>>(entity).unwrap().0, 1);
}
