use std::str::FromStr;

use anyhow::bail;
use base::ProfileProperty;
use protocol::packets::client::Handshake;
use uuid::Uuid;

use super::ProxyData;

/// Tries to extract the player information that is sent in the `server_address` field of a
/// Handshake packet that originates from a BungeeCord style proxy. This is used to enable IP
/// forwarding for BungeeCord style proxies.
///
/// The server address field should have 4 parts if a client is connecting via BungeeCord. The field
/// has the following format:
///
/// format!("{}\0{}\0{}\0{}", host, address, uuid, mojang_response);
///
/// | Variable        | Definition                                          |
/// |-----------------|-----------------------------------------------------|
/// | Host            | The IP address of the BungeeCord instance           |
/// | Address         | The IP address of the connecting client             |
/// | UUID            | The UUID that is associated to the clients account  |
/// | Mojang response | A JSON formatted version of the `properties` field
/// in [Mojangs response](https://wiki.vg/Protocol_Encryption#Server)       |
#[allow(clippy::match_ref_pats)]
pub fn extract(packet: &Handshake) -> anyhow::Result<ProxyData> {
    let parts: Vec<&str> = packet.server_address.split('\0').collect();
    match parts.as_slice() {
        &[host, client, uuid, json_properties] => Ok(ProxyData {
            host: host.to_owned(),
            client: client.to_owned(),
            uuid: Uuid::from_str(uuid)?,
            profile: serde_json::from_str::<Vec<ProfileProperty>>(json_properties)?,
        }),
        _ => bail!("IP forwarding is not enabled on the proxy"),
    }
}

#[cfg(test)]
mod tests {
    use base::ProfileProperty;
    use protocol::packets::client::HandshakeState;

    use crate::initial_handler::PROTOCOL_VERSION;

    use super::*;

    #[test]
    fn extract_bungeecord_data_normal() {
        let handshake = Handshake {
           protocol_version: PROTOCOL_VERSION,
           server_address: "192.168.1.87\0192.168.1.67\0905c7e4fb96b45139645d123225575e2\0[{\"name\":\"textures\",\"value\":\"textures_value\",\"signature\":\"textures_signature\"}]".to_string(),
           server_port: 25565,
           next_state: HandshakeState::Login,
        };

        assert_eq!(
            extract(&handshake).unwrap(),
            ProxyData {
                host: "192.168.1.87".to_string(),
                client: "192.168.1.67".to_string(),
                uuid: Uuid::parse_str("905c7e4fb96b45139645d123225575e2").unwrap(),
                profile: vec![ProfileProperty {
                    name: "textures".to_string(),
                    value: "textures_value".to_string(),
                    signature: "textures_signature".to_string(),
                }],
            }
        );
    }

    #[test]
    fn extract_bungeecord_data_too_short() {
        let handshake = Handshake {
            protocol_version: PROTOCOL_VERSION,
            server_address: "192.168.1.87\0192.168.1.67\0905c7e4fb96b45139645d123225575e2"
                .to_string(),
            server_port: 25565,
            next_state: HandshakeState::Login,
        };

        extract(&handshake).unwrap_err();
    }

    #[test]
    fn extract_bungeecord_data_too_long() {
        let handshake = Handshake {
            protocol_version: PROTOCOL_VERSION,
            server_address: "192.168.1.87\0192.168.1.67\0905c7e4fb96b45139645d123225575e2\0a\0b"
                .to_string(),
            server_port: 25565,
            next_state: HandshakeState::Login,
        };

        extract(&handshake).unwrap_err();
    }

    #[test]
    fn extract_bungeecord_data_localhost_host_ip() {
        let handshake = Handshake {
            protocol_version: PROTOCOL_VERSION,
            server_address: "localhost\0192.168.1.67\0905c7e4fb96b45139645d123225575e2\0[{\"name\":\"textures\",\"value\":\"textures_value\",\"signature\":\"textures_signature\"}]".to_string(),
            server_port: 25565,
            next_state: HandshakeState::Login,
        };

        assert_eq!(
            extract(&handshake).unwrap(),
            ProxyData {
                host: "localhost".to_string(),
                client: "192.168.1.67".to_string(),
                uuid: Uuid::parse_str("905c7e4fb96b45139645d123225575e2").unwrap(),
                profile: vec![ProfileProperty {
                    name: "textures".to_string(),
                    value: "textures_value".to_string(),
                    signature: "textures_signature".to_string(),
                }],
            }
        );
    }

    #[test]
    fn extract_bungeecord_data_localhost_client_ip() {
        let handshake = Handshake {
            protocol_version: PROTOCOL_VERSION,
            server_address: "192.168.1.87\0localhost\0905c7e4fb96b45139645d123225575e2\0[{\"name\":\"textures\",\"value\":\"textures_value\",\"signature\":\"textures_signature\"}]".to_string(),
            server_port: 25565,
            next_state: HandshakeState::Login,
        };

        assert_eq!(
            extract(&handshake).unwrap(),
            ProxyData {
                host: "192.168.1.87".to_string(),
                client: "localhost".to_string(),
                uuid: Uuid::parse_str("905c7e4fb96b45139645d123225575e2").unwrap(),
                profile: vec![ProfileProperty {
                    name: "textures".to_string(),
                    value: "textures_value".to_string(),
                    signature: "textures_signature".to_string(),
                }],
            }
        );
    }

    #[test]
    fn extract_bungeecord_data_invalid_uuid() {
        let handshake = Handshake {
            protocol_version: PROTOCOL_VERSION,
            server_address: "192.168.1.87\0192.168.1.67\005c7e4fb9675e2\0[{\"name\":\"textures\",\"value\":\"textures_value\",\"signature\":\"textures_signature\"}]".to_string(),
            server_port: 25565,
            next_state: HandshakeState::Login,
        };

        extract(&handshake).unwrap_err();
    }

    #[test]
    fn extract_bungeecord_data_invalid_properties() {
        let handshake = Handshake {
            protocol_version: PROTOCOL_VERSION,
            server_address: "192.168.1.87\0192.168.1.67\0905c7e4fb96b45139645d123225575e2\0[{\"name\":\"textures\",\"value\":\"textures_value\",\"sinature\":\"textures_signature\"}]".to_string(),
            server_port: 25565,
            next_state: HandshakeState::Login,
        };

        extract(&handshake).unwrap_err();
    }
}
