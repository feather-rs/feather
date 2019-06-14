#[macro_use]
extern crate log;

pub mod io;

pub struct Server {}

fn main() {
    simple_logger::init_with_level(log::Level::Trace);

    info!("Starting Feather; please wait...");
}
