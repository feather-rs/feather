use super::super::mctypes::{McTypeWrite, McTypeRead};
use super::*;
use bytes::{Buf, BufMut};

lazy_static! {
    pub static ref IMPL_MAP: im::HashMap<PacketType, PacketBuilder> = {
        let mut m = im::HashMap::new();

        // Serverbound
        m.insert(PacketType::Handshake, PacketBuilder::with(|| Box::new(Handshake::default())));
        m.insert(PacketType::LoginStart, PacketBuilder::with(|| Box::new(LoginStart::default())));
        m.insert(PacketType::EncryptionResponse, PacketBuilder::with(|| Box::new(EncryptionResponse::default())));

        m.insert(PacketType::Request, PacketBuilder::with(|| Box::new(Request::default())));
        m.insert(PacketType::Ping, PacketBuilder::with(|| Box::new(Ping::default())));

        // Clientbound
        m.insert(PacketType::DisconnectLogin, PacketBuilder::with(|| Box::new(DisconnectLogin::default())));
        m.insert(PacketType::EncryptionRequest, PacketBuilder::with(|| Box::new(EncryptionRequest::default())));
        m.insert(PacketType::LoginSuccess, PacketBuilder::with(|| Box::new(LoginSuccess::default())));
        m.insert(PacketType::SetCompression, PacketBuilder::with(|| Box::new(SetCompression::default())));

        m.insert(PacketType::Response, PacketBuilder::with(|| Box::new(Response::default())));
        m.insert(PacketType::Pong, PacketBuilder::with(|| Box::new(Pong::default())));

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
    fn read_from(&mut self, mut buf: &mut Buf) -> Result<(), ()> {
        self.protocol_version = buf.get_u32_be();
        self.server_address = buf.read_string()?;
        self.server_port = buf.get_u16_be();
        let state = buf.read_var_int()?;

        self.next_state = match state {
            1 => HandshakeState::Status,
            2 => HandshakeState::Login,
            _ => return Err(()),
        };

        Ok(())
    }

    fn write_to(&self, mut buf: &mut BufMut) {
        unimplemented!()
    }

    fn ty(&self) -> PacketType {
        PacketType::Handshake
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
    fn read_from(&mut self, mut buf: &mut Buf) -> Result<(), ()> {
        self.username = buf.read_string()?;

        Ok(())
    }

    fn write_to(&self, mut buf: &mut BufMut) {
        unimplemented!()
    }

    fn ty(&self) -> PacketType {
        PacketType::LoginStart
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
    fn read_from(&mut self, mut buf: &mut Buf) -> Result<(), ()> {
        self.secret_length = buf.read_var_int()?;

        let mut secret = vec![];
        for _ in 0..self.secret_length {
            secret.push(buf.get_u8());
        }
        self.secret = secret;

        self.verify_token_length = buf.read_var_int()?;

        let mut verify_token = vec![];
        for _ in 0..self.secret_length {
            verify_token.push(buf.get_u8());
        }

        self.verify_token = verify_token;
        Ok(())
    }

    fn write_to(&self, mut buf: &mut BufMut) {
        unimplemented!()
    }

    fn ty(&self) -> PacketType {
        PacketType::EncryptionResponse
    }
}

#[derive(Default)]
pub struct Request {}

impl Packet for Request {
    fn read_from(&mut self, mut buf: &mut Buf) -> Result<(), ()>{
        Ok(())
    }

    fn write_to(&self, buf: &mut BufMut) {
        unimplemented!()
    }

    fn ty(&self) -> PacketType {
        PacketType::Request
    }
}

#[derive(Default)]
pub struct Ping {
    payload: u64,
}

impl Packet for Ping {
    fn read_from(&mut self, mut buf: &mut Buf) -> Result<(), ()> {
        self.payload = buf.get_u64_be();
        Ok(())
    }

    fn write_to(&self, buf: &mut BufMut) {
        unimplemented!()
    }

    fn ty(&self) -> PacketType {
        PacketType::Ping
    }
}

// CLIENTBOUND
#[derive(Default)]
pub struct DisconnectLogin {
    reason: String,
}

impl Packet for DisconnectLogin {
    fn read_from(&mut self, mut buf: &mut Buf) -> Result<(), ()> {
        self.reason = buf.read_string()?;
        Ok(())
    }

    fn write_to(&self, mut buf: &mut BufMut) {
        buf.write_string(self.reason.as_str());
    }

    fn ty(&self) -> PacketType {
        PacketType::DisconnectLogin
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
    fn read_from(&mut self, mut buf: &mut Buf) -> Result<(), ()>{
        unimplemented!()
    }

    fn write_to(&self,mut  buf: &mut BufMut) {
        buf.write_string(self.server_id.as_str());
        buf.write_var_int(self.public_key_len);

        for val in self.public_key.iter() {
            buf.put_u8(val.clone());
        }

        buf.write_var_int(self.verify_token_len);
        for val in self.verify_token.iter() {
            buf.put_u8(val.clone());
        }
    }

    fn ty(&self) -> PacketType {
        PacketType::EncryptionRequest
    }
}

#[derive(Default)]
pub struct LoginSuccess {
    uuid: String,
    username: String,
}

impl Packet for LoginSuccess {
    fn read_from(&mut self, mut buf: &mut Buf) -> Result<(), ()>{
        unimplemented!()
    }

    fn write_to(&self, mut buf: &mut BufMut) {
        buf.write_string(self.uuid.as_str());
        buf.write_string(self.username.as_str());
    }

    fn ty(&self) -> PacketType {
        PacketType::LoginSuccess
    }
}

#[derive(Default)]
pub struct SetCompression {
    threshold: i32,
}

impl Packet for SetCompression {
    fn read_from(&mut self, mut buf: &mut Buf) -> Result<(), ()>{
        unimplemented!()
    }

    fn write_to(&self, mut buf: &mut BufMut) {
        buf.write_var_int(self.threshold);
    }

    fn ty(&self) -> PacketType {
        PacketType::SetCompression
    }
}

#[derive(Default)]
pub struct Response {
    json_response: String,
}

impl Packet for Response {
    fn read_from(&mut self, mut buf: &mut Buf) -> Result<(), ()> {
        unimplemented!()
    }

    fn write_to(&self, mut buf: &mut BufMut) {
        buf.write_string(self.json_response.as_str());
    }

    fn ty(&self) -> PacketType {
        PacketType::Response
    }
}

#[derive(Default)]
pub struct Pong {
    payload: u64,
}

impl Packet for Pong {
    fn read_from(&mut self, mut buf: &mut Buf) -> Result<(), ()> {
        unimplemented!()
    }

    fn write_to(&self, mut buf: &mut BufMut) {
        buf.put_u64_be(self.payload);
    }

    fn ty(&self) -> PacketType {
        PacketType::Ping
    }
}
