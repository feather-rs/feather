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

// SERVERBOUND

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

#[derive(Default)]
pub struct Request {

}

impl Packet for Request {
    fn read_from(&mut self, buf: &mut ByteBuf) -> Option<()> {
        Some(())
    }

    fn write_to(&self, buf: &mut ByteBuf) {
        unimplemented!()
    }
}

#[derive(Default)]
pub struct Ping {
    payload: u64,
}

impl Packet for Ping {
    fn read_from(&mut self, buf: &mut ByteBuf) -> Option<()> {
        self.payload = buf.read_u64();
        Some(())
    }

    fn write_to(&self, buf: &mut ByteBuf) {
        unimplemented!()
    }
}

// CLIENTBOUND
#[derive(Default)]
pub struct DisconnectLogin {
    reason: String,
}

impl Packet for DisconnectLogin {
    fn read_from(&mut self, buf: &mut ByteBuf) -> Option<()> {
        self.reason = buf.read_string()?;
        Some(())
    }

    fn write_to(&self, buf: &mut ByteBuf) {
        buf.write_string(self.reason.as_str());
    }
}

#[derive(Default)]
pub struct EncryptionRequest {
    server_id: String,
    public_key_len: i32,
    public_key: Vec<u8>,
    verify_token_len: i32,
    verify_token: Vec<u8>,
}

impl Packet for EncryptionRequest {
    fn read_from(&mut self, buf: &mut ByteBuf) -> Option<()> {
        unimplemented!()
    }

    fn write_to(&self, buf: &mut ByteBuf) {
        buf.write_string(self.server_id.as_str());
        buf.write_var_int(self.public_key_len);

        for val in self.public_key {
            buf.write_u8(val);
        }

        buf.write_var_int(self.verify_token_len);
        for val in self.verify_token {
            buf.write_u8(val);
        }
    }
}

#[derive(Default)]
pub struct LoginSuccess {
    uuid: String,
    username: String,
}

impl Packet for LoginSuccess {
    fn read_from(&mut self, buf: &mut ByteBuf) -> Option<()> {
        unimplemented!()
    }

    fn write_to(&self, buf: &mut ByteBuf) {
        buf.write_string(self.uuid.as_str());
        buf.write_string(self.username.as_str());
    }
}

#[derive(Default)]
pub struct SetCompression {
    threshold: i32,
}

impl Packet for SetCompression {
    fn read_from(&mut self, buf: &mut ByteBuf) -> Option<()> {
        unimplemented!()
    }

    fn write_to(&self, buf: &mut ByteBuf) {
        buf.write_var_int(self.threshold);
    }
}

#[derive(Default)]
pub struct Response {
    json_response: String,
}

impl Packet for Response {
    fn read_from(&mut self, buf: &mut ByteBuf) -> Option<()> {
        unimplemented!()
    }

    fn write_to(&self, buf: &mut ByteBuf) {
        buf.write_string(self.json_response.as_str());
    }
}

#[derive(Default)]
pub struct Pong {
    payload: u64,
}

impl Packet for Pong {
    fn read_from(&mut self, buf: &mut ByteBuf) -> Option<()> {
        unimplemented!()
    }

    fn write_to(&self, buf: &mut ByteBuf) {
        buf.write_u64(self.payload);
    }
}
