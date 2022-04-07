use crate::{Component, Entities, Entity};

/// A bundle of components that can be added to an entity.
pub trait ComponentBundle: Sized {
    /// Adds components to the entity.
    fn add_to_entity(self, world: &mut Entities, entity: Entity);
}

macro_rules! bundle_tuple {
    ($($ty:ident),* $(,)?) => {
        impl <$($ty: Component),*> ComponentBundle for ($($ty,)*) {
            #[allow(non_snake_case)]
            fn add_to_entity(self, world: &mut Entities, entity: Entity) {
                let ($($ty,)*) = self;
                $(
                    world.insert(entity, $ty).unwrap();
                )*
            }
        }
    }
}

bundle_tuple!(T1);
bundle_tuple!(T1, T2);
bundle_tuple!(T1, T2, T3);
bundle_tuple!(T1, T2, T3, T4);
bundle_tuple!(T1, T2, T3, T4, T5);
bundle_tuple!(T1, T2, T3, T4, T5, T6);
