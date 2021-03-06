use quill_common::entities::Player;

use crate::{Entity, Plucker, PluckerRef, Tuple};

pub trait SendMessage<Index> {
    fn send_message(&self, message: &str);
}

impl<T, Index> SendMessage<Index> for (Entity, T)
where
    T: Tuple,
    <T as Tuple>::HList: PluckerRef<Player, Index>
{
    fn send_message(&self, message: &str) {
        let components = &self.1;
        let foo = components.hlist();
        let player = PluckerRef::<Player, Index>::pluck(components.hlist());
    }
}