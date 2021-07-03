use quill_common::entities::Player;

use crate::{Entity, PluckerRef, Tuple};

pub trait SendMessage<Index> {
    fn send_message(&self, message: &str);
}

impl<T, Index> SendMessage<Index> for (&'_ Entity, T)
where
    T: Tuple,
    <T as Tuple>::HList: PluckerRef<Player, Index>,
{
    fn send_message(&self, _message: &str) {
        // let _player: &Player = PluckerRef::<Player, Index>::pluck(self.1.hlist());
    }
}
