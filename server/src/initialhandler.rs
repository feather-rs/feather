use crate::prelude::*;
use feather_core::network::packet::{implementation::*, Packet, PacketType};
use openssl::pkey::Private;
use openssl::rsa::{self, Rsa};
use std::rc::Rc;

const PROTOCOL_VERSION: u32 = 404;

#[derive(Debug, Eq, PartialEq)]
pub enum State {
    Handshake,
    Status,
    Login,
    Play,
}
