use crate::Component;

pub trait EntityBuilder {
    fn add<C: Component>(&mut self, component: C) -> &mut Self;
}

pub trait ComponentBundle {
    fn add_to_builder(self, builder: &mut impl EntityBuilder);
}

impl ComponentBundle for () {
    #[inline]
    fn add_to_builder(self, _builder: &mut impl EntityBuilder) {}
}

macro_rules! tuple_impl {
    ($($idents:ident),*$(,)?) => {
        impl<$($idents),*> ComponentBundle for ($($idents,)*)
        where
            $($idents: ComponentBundle),*
        {
            #[inline]
            #[allow(non_snake_case)]
            fn add_to_builder(self, builder: &mut impl EntityBuilder) {
                let ($($idents,)*) = self;
                $(ComponentBundle::add_to_builder($idents, builder);)*
            }
        }
    };
}

macro_rules! smaller_tuples_too {
    ($macro:ident, $head:ident $(,)?) => {
        $macro!($head);
    };
    ($macro:ident, $head:ident, $($tail:ident),* $(,)?) => {
        $macro!($head, $($tail),*);
        smaller_tuples_too!($macro, $($tail),*);
    };
}

smaller_tuples_too!(
    tuple_impl, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18,
    T19, T20
);
