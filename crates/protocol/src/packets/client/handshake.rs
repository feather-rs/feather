use super::*;

def_enum! {
    HandshakeState (VarInt) {
        1 = Status,
        2 = Login,
    }
}

packets! {
    Handshake {
        protocol_version VarInt;
        server_address String;
        server_port u16;
        next_state HandshakeState;
    }
}
