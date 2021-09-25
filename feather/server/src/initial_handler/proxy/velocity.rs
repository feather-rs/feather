use std::io::Cursor;

use anyhow::anyhow;
use anyhow::bail;
use base::ProfileProperty;
use protocol::{
    io::VarIntPrefixedVec, packets::server::LoginPluginRequest, ClientLoginPacket, ProtocolVersion,
    Readable, ServerLoginPacket, VarInt,
};
use ring::{
    digest::{self},
    hmac::{self, Key},
};
use uuid::Uuid;

use crate::connection_worker::Worker;

use super::ProxyData;

/// The plugin messaging channel used to receive the proxy data.
pub const CHANNEL: &str = "velocity:player_info";

/// Matches the version in VelocityConstants.java
const FORWARDING_VERSION: i32 = 1;

const TAG_LENGTH: usize = digest::SHA256_OUTPUT_LEN;

const MESSAGE_ID: i32 = 100000; // arbitrary

/// Runs Velocity IP forwarding.
pub async fn run(worker: &mut Worker) -> anyhow::Result<ProxyData> {
    send_plugin_message(worker).await?;
    receive_response(worker).await
}

async fn send_plugin_message(worker: &mut Worker) -> anyhow::Result<()> {
    worker
        .write(ServerLoginPacket::LoginPluginRequest(LoginPluginRequest {
            message_id: MESSAGE_ID,
            channel: CHANNEL.to_owned(),
            data: Vec::new(),
        }))
        .await
}

async fn receive_response(worker: &mut Worker) -> anyhow::Result<ProxyData> {
    loop {
        let response = worker.read::<ClientLoginPacket>().await?;

        match response {
            ClientLoginPacket::LoginPluginResponse(packet) => {
                if packet.message_id == MESSAGE_ID {
                    return read_player_info(&worker.options().velocity_secret, &packet.data);
                }
            }
            _ => continue,
        }
    }
}

fn read_player_info(key: &str, payload: &[u8]) -> anyhow::Result<ProxyData> {
    let payload = verify_hmac(key, payload)?;

    let mut payload = Cursor::new(payload);
    let mcversion = ProtocolVersion::V1_16_2;

    let version = VarInt::read(&mut payload, mcversion)?;
    if version.0 != FORWARDING_VERSION {
        bail!(
            "Velocity version mismatch: Feather supports version {} but Velocity is version {}",
            FORWARDING_VERSION,
            version.0
        );
    }

    let client = String::read(&mut payload, mcversion)?;
    let uuid = Uuid::read(&mut payload, mcversion)?;
    let _name = String::read(&mut payload, mcversion)?;
    let properties = VarIntPrefixedVec::<Property>::read(&mut payload, mcversion)?;

    Ok(ProxyData {
        host: "".to_owned(),
        client,
        uuid,
        profile: properties.0.iter().map(|prop| prop.0.clone()).collect(),
    })
}

fn verify_hmac<'a>(key: &str, payload: &'a [u8]) -> anyhow::Result<&'a [u8]> {
    if payload.len() < TAG_LENGTH {
        bail!("player info payload too small (check that Velocity has IP forwarding enabled)");
    }
    let (tag, payload) = payload.split_at(TAG_LENGTH);

    let algorithm = hmac::HMAC_SHA256;
    hmac::verify(&Key::new(algorithm, key.as_bytes()), payload, tag).map_err(|_| anyhow!(
        "failed to verify payload: check that velocity_key is set correctly in config.toml",
    ))?;

    Ok(payload)
}

#[derive(Debug, Clone)]
struct Property(ProfileProperty);

impl Readable for Property {
    fn read(buffer: &mut Cursor<&[u8]>, version: ProtocolVersion) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let name = String::read(buffer, version)?;
        let value = String::read(buffer, version)?;
        let has_signature = bool::read(buffer, version)?;
        let signature = has_signature
            .then(|| String::read(buffer, version))
            .transpose()?;
        Ok(Self(ProfileProperty {
            name,
            value,
            signature: signature.unwrap_or_default(),
        }))
    }
}
