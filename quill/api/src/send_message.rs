use quill_common::entities::Player;

use crate::{Entity, PluckerRef, TupleRef};

pub trait SendMessage<'a, Index> {
    fn send_message(&'a self, message: &str);
}

impl<'a, T, Index> SendMessage<'a, Index> for (&'a Entity, T)
where
    T: TupleRef<'a>,
    <T as TupleRef<'a>>::HList: PluckerRef<Player, Index>,
{
    fn send_message(&'a self, _message: &str) {
        let foo = self.1.hlist();
        let _player: &Player = PluckerRef::<Player, Index>::pluck(&foo);
    }
}
