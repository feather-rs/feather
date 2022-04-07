#![allow(clippy::unnecessary_wraps)] // systems are required to return Results

pub mod builder;
mod chunk_subscriptions;
pub mod client;
pub mod config;
mod connection_worker;
mod entities;
pub mod favicon;
pub mod init;
mod initial_handler;
mod listener;
mod logging;
mod network_id_registry;
mod options;
mod packet_handlers;
mod player_count;
mod plugin;
pub mod server;
mod systems;

pub use builder::ServerBuilder;
pub use client::{Client, ClientId, Clients};
pub use network_id_registry::NetworkId;
pub use options::Options;
pub use server::Server;
