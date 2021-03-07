use quill_common::entities::Player;

use crate::{Entity, PluckerRef, TupleRef};

pub trait SendMessage<'a, Index> {
    fn send_message(&'a self, message: &str);
}

impl<'a, T, Index> SendMessage<'a, Index> for (&'a Entity, &'a T)
where
    &'a T: TupleRef<'a>,
    <&'a T as TupleRef<'a>>::HList: PluckerRef<Player, Index>
{
    fn send_message(&'a self, message: &str) {
        let foo = self.1.hlist();
        let player = PluckerRef::<Player, Index>::pluck(&foo);
    }
}