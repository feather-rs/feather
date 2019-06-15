#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

#[cfg(test)]
#[macro_use]
extern crate mockers_derive;

pub mod config;
pub mod initialhandler;
pub mod io;

pub struct Server {}

fn main() {
    let config = config::load()
        .expect("Failed to load configuration. Please ensure that the file feather.toml exists and is correct.");


    simple_logger::init_with_level(log::Level::Trace).unwrap();

    info!("Starting Feather; please wait...");
}
