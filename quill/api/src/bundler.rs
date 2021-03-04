use quill_common::Component;

use crate::EntityBuilder;

pub trait DynamicBundle {
    fn add_to_builder(self, builder: &mut EntityBuilder);
}

impl DynamicBundle for () {
    fn add_to_builder(self, _builder: &mut EntityBuilder) {}
}

macro_rules! tuple_impl {
    ($head:ident $(,$tail:ident)*$(,)?) => {
        impl<$head, $($tail),*> DynamicBundle for ($head, $($tail),*)
        where
            $head: Component
            $(,
                $tail: Component
            )*
        {
            #[allow(non_snake_case)]
            #[inline]
            fn add_to_builder(self, builder: &mut EntityBuilder) {
                let ($head, $($tail),*) = self;
                builder.add($head);
                DynamicBundle::add_to_builder(($($tail,)*), builder);
            }
        }
    };
}

macro_rules! smaller_tuples_too {
    ($macro:ident, $head:ident) => {
        $macro!($head);
    };
    ($macro:ident, $head:ident, $($tail:ident),*$(,)?) => {
        $macro!($head, $($tail),*);
        smaller_tuples_too!($macro, $($tail),*);
    };
}

smaller_tuples_too!(
    tuple_impl, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18,
    T19, T20
);
