#[macro_use]
extern crate log;

#[cfg(test)]
#[macro_use]
extern crate mockers_derive;

pub mod io;

pub struct Server {}

fn main() {
    simple_logger::init_with_level(log::Level::Trace).unwrap();

    info!("Starting Feather; please wait...");
}
