use quill_common::entities::Player;

use crate::{Entity, Plucker, Tuple};

pub trait SendMessage<Index> {
    fn send_message(&self, message: &str);
}

impl<T, Index> SendMessage<Index> for (Entity, T)
where
    T: Tuple,
    <T as Tuple>::HList: Plucker<Player, Index>
{
    fn send_message(&self, message: &str) {
        let components = &self.1;
        let player = Plucker::<Player, Index>::pluck(&components.hlist());
    }
}