#![forbid(unsafe_code)]

mod chest;
mod init;

pub use chest::{
    on_chest_break_drop_contents, on_chest_close_decrement_viewers, on_chest_open_increment_viewers,
};
pub use init::on_block_update_create_block_entity;
