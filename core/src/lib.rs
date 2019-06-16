#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate derive_new;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate feather_codegen;

pub mod bytebuf;
pub mod network;
pub mod prelude;

#[derive(Serialize, Deserialize, Debug)]
pub enum Gamemode {
    Survival,
    Creative,
    Spectator,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Difficulty {
    Peaceful,
    Easy,
    Medium,
    Hard,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PvpStyle {
    Classic,
    New,
}
