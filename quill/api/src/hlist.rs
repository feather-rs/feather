use std::marker::PhantomData;

pub struct HCons<Head, Tail> {
    pub head: Head,
    pub tail: Tail,
}

pub trait HList: Sized {
    type Tuple: Tuple<HList = Self>;

    fn flatten(self) -> Self::Tuple;
}

pub trait HListRef<'a>: Sized {
    type Tuple: TupleRef<'a, HList = Self>;

    fn flatten(&'a self) -> Self::Tuple;
}

pub trait Tuple: Sized {
    type HList: HList<Tuple = Self>;

    fn hlist(self) -> Self::HList;
}

pub trait TupleRef<'a>: Sized {
    type HList: HListRef<'a, Tuple = Self>;

    fn hlist(&'a self) -> Self::HList;
}

impl HList for () {
    type Tuple = ();

    #[inline]
    fn flatten(self) -> Self::Tuple {}
}

impl<'a> HListRef<'a> for &'a () {
    type Tuple = &'a ();

    #[inline]
    fn flatten(&'a self) -> Self::Tuple {
        &()
    }
}

impl<'a, T1> HListRef<'a> for HCons<&'a T1, &'a ()> {
    type Tuple = (&'a T1,);

    #[inline]
    fn flatten(&'a self) -> Self::Tuple {
        (&self.head,)
    }
}

impl Tuple for () {
    type HList = ();

    #[inline]
    fn hlist(self) -> Self::HList {}
}

impl<'a> TupleRef<'a> for &'a () {
    type HList = &'a ();

    #[inline]
    fn hlist(&'a self) -> Self::HList {
        &()
    }
}

impl<'a, T1> TupleRef<'a> for (&'a T1,) {
    type HList = HCons<&'a T1, &'a ()>;

    #[inline]
    fn hlist(&'a self) -> Self::HList {
        HCons {
            head: self.0,
            tail: &(),
        }
    }
}

macro_rules! Product {
    () => { () };
    ($head:ident $(,$tail:ident)* $(,)?) => {
        HCons<$head, Product!($($tail),*)>
    };
}

macro_rules! product_pat {
    () => { () };
    ($head:ident $(,$tail:ident)* $(,)?) => {
        HCons {
            head: $head, 
            tail: product_pat!($($tail),*)
        }
    };
}

macro_rules! impl_tuple {
    ($head:ident $(,$tail:ident)* $(,)?) => {
        impl<$head $(,$tail)*> Tuple for ($head, $($tail),*) {
            type HList = HCons<$head, <($($tail,)*) as Tuple>::HList>;

            #[inline]
            fn hlist(self) -> Self::HList {
                #[allow(non_snake_case)]
                let ($head, $($tail),*) = self;
                HCons {
                    head: $head,
                    tail: ($($tail,)*).hlist(),
                }
            }
        }

        impl<$head $(,$tail)*> HList for Product!($head $(,$tail)*) {
            type Tuple = ($head, $($tail),*);

            #[inline]
            fn flatten(self) -> Self::Tuple {
                #[allow(non_snake_case)]
                let product_pat!($head, $($tail),*) = self;
                ($head, $($tail),*)
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
    impl_tuple, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18,
    T19, T20
);


pub trait Plucker<Target, Index> {
    type Remainder;

    fn pluck(self) -> (Target, Self::Remainder);
}

pub trait PluckerRef<Target, Index> {
    fn pluck(&self) -> &Target;
}

pub struct Here();

impl<Target, Tail> Plucker<Target, Here> for HCons<Target, Tail> {
    type Remainder = Tail;

    #[inline]
    fn pluck(self) -> (Target, Self::Remainder) {
        (self.head, self.tail)
    }
}

impl<'a, Target, Tail> PluckerRef<Target, Here> for &'a HCons<&'a Target, &'a Tail> {
    #[inline]
    fn pluck(&self) -> &Target {
        self.head
    }
}

pub struct There<T>(PhantomData<T>);

impl<Target, Head, Tail, TailIndex> Plucker<Target, There<TailIndex>> for HCons<Head, Tail>
where
    Tail: Plucker<Target, TailIndex>,
{
    type Remainder = HCons<Head, <Tail as Plucker<Target, TailIndex>>::Remainder>;

    #[inline]
    fn pluck(self) -> (Target, Self::Remainder) {
        let (target, tail_remainder): (Target, <Tail as Plucker<Target, TailIndex>>::Remainder) =
            <Tail as Plucker<Target, TailIndex>>::pluck(self.tail);
        (
            target,
            HCons {
                head: self.head,
                tail: tail_remainder,
            },
        )
    }
}

impl<'a, Target, Head, Tail, TailIndex> PluckerRef<Target, There<TailIndex>> for &'a HCons<&'a Head, &'a Tail>
where
    &'a Tail: PluckerRef<Target, TailIndex>,
{
    #[inline]
    fn pluck(&self) -> &Target {
        <&'a Tail as PluckerRef<Target, TailIndex>>::pluck(&self.tail)
    }
}

