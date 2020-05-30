#![forbid(unsafe_code)]

mod chest;
mod init;

pub use chest::on_chest_break_drop_contents;
pub use init::on_block_update_create_block_entity;
