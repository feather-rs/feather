use super::super::mctypes::*;
use super::*;

lazy_static! {
    pub static ref IMPL_MAP: im::HashMap<PacketType, PacketBuilder> = {
        let mut m = im::HashMap::new();

        m.insert(PacketType::Handshake, PacketBuilder::with(|| Box::new(Handshake::default())));
        m.insert(PacketType::LoginStart, PacketBuilder::with(|| Box::new(LoginStart::default())));
        m.insert(PacketType::EncryptionResponse, PacketBuilder::with(|| Box::new(EncryptionResponse::default())));

        m
    };
}

#[derive(Default)]
pub struct Handshake {
    pub protocol_version: u32,
    pub server_address: String,
    pub server_port: u16,
    pub next_state: HandshakeState,
}

impl Packet for Handshake {
    fn read_from(&mut self, buf: &mut ByteBuf) -> Option<()> {
        self.protocol_version = buf.read_u32();
        self.server_address = buf.read_string()?;
        self.server_port = buf.read_u16();
        let state = buf.read_var_int()?;

        self.next_state = match state {
            1 => HandshakeState::Status,
            2 => HandshakeState::Login,
            _ => return None,
        };

        Some(())
    }

    fn write_to(&self, buf: &mut ByteBuf) {
        buf.write_var_int(self.protocol_version as i32);
        buf.write_string(self.server_address.as_str());
        buf.write_u16(self.server_port);
        let state = match self.next_state {
            HandshakeState::Status => 1,
            HandshakeState::Login => 2,
        };
        buf.write_var_int(state);
    }
}

pub enum HandshakeState {
    Status,
    Login,
}

impl Default for HandshakeState {
    fn default() -> Self {
        HandshakeState::Status
    }
}

#[derive(Default)]
pub struct LoginStart {
    username: String,
}

impl Packet for LoginStart {
    fn read_from(&mut self, buf: &mut ByteBuf) -> Option<()> {
        self.username = buf.read_string()?;

        Some(())
    }

    fn write_to(&self, buf: &mut ByteBuf) {
        unimplemented!()
    }
}

#[derive(Default)]
pub struct EncryptionResponse {
    secret_length: i32,
    secret: Vec<u8>,
    verify_token_length: i32,
    verify_token: Vec<u8>,
}

impl Packet for EncryptionResponse {
    fn read_from(&mut self, buf: &mut ByteBuf) -> Option<()> {
        self.secret_length = buf.read_var_int()?;

        let mut secret = vec![];
        for _ in 0..self.secret_length {
            secret.push(buf.read_u8());
        }
        self.secret = secret;

        self.verify_token_length = buf.read_var_int()?;

        let mut verify_token = vec![];
        for _ in 0..self.secret_length {
            verify_token.push(buf.read_u8());
        }

        self.verify_token = verify_token;
        Some(())
    }

    fn write_to(&self, buf: &mut ByteBuf) {
        unimplemented!()
    }
}