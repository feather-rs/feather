use std::fmt::{Debug, Formatter};
use std::io::Cursor;
use std::net::IpAddr;
use std::ops::{Add, Deref};
use std::path::Path;

use chrono::{DateTime, Duration, Local};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug)]
pub struct BanList {
    banned_players: Vec<BanEntry<(Uuid, String)>>,
    banned_ips: Vec<BanEntry<IpAddr>>,
}

impl BanList {
    /// Ban the player. Returns false if the player is already banned
    pub fn ban(
        &mut self,
        uuid: Uuid,
        name: String,
        by: Option<String>,
        reason: BanReason,
        duration: impl Into<Option<Duration>>,
    ) -> bool {
        if self.banned_players.iter().any(|e| e.value.0 == uuid) {
            false
        } else {
            let time = Local::now().into();
            self.banned_players.push(BanEntry {
                value: (uuid, name),
                banned: time,
                source: by,
                expires: duration.into().map(|duration| time.clone().add(duration)),
                reason,
            });
            true
        }
    }
    /// Ban the IP address. Returns false if the ip is already banned
    pub fn ban_ip(
        &mut self,
        ip: IpAddr,
        by: Option<String>,
        reason: BanReason,
        duration: impl Into<Option<Duration>>,
    ) -> bool {
        if self.banned_ips.iter().any(|e| e.value == ip) {
            false
        } else {
            let time = Local::now().into();
            self.banned_ips.push(BanEntry {
                value: ip,
                banned: time,
                source: by,
                expires: duration.into().map(|duration| time.clone().add(duration)),
                reason,
            });
            true
        }
    }

    /// Returns none if not banned
    pub fn get_ban_reason(&self, uuid: &Uuid) -> Option<BanReason> {
        self.banned_players
            .iter()
            .find(|entry| entry.value.0 == *uuid)
            .map(|entry| entry.reason.clone())
    }

    /// Returns none if not banned
    pub fn get_ip_ban_reason(&self, ip: &IpAddr) -> Option<BanReason> {
        self.banned_ips
            .iter()
            .find(|entry| entry.value == *ip)
            .map(|entry| entry.reason.clone())
    }

    /// Returns true if unbanned, false if the player is not banned
    pub fn pardon_name(&mut self, name: &str) -> bool {
        let old_len = self.banned_players.len();
        self.banned_players.retain(|entry| entry.value.1 != *name);
        old_len != self.banned_players.len()
    }

    /// Returns true if unbanned, false if the player is not banned
    pub fn pardon_id(&mut self, id: &Uuid) -> bool {
        let old_len = self.banned_players.len();
        self.banned_players.retain(|entry| entry.value.0 != *id);
        old_len != self.banned_players.len()
    }

    /// Returns true if unbanned, false if the ip is not banned
    pub fn pardon_ip(&mut self, ip: &IpAddr) -> bool {
        let old_len = self.banned_ips.len();
        self.banned_ips.retain(|entry| entry.value != *ip);
        old_len != self.banned_ips.len()
    }
}

#[derive(Debug)]
pub struct BanEntry<T> {
    value: T,
    /// Timestamp when the player/ip was banned
    banned: DateTime<Local>,
    /// Some if banned by a player, None if banned by console
    source: Option<String>,
    /// Timestamp when the player/ip should be unbanned
    expires: Option<DateTime<Local>>,
    reason: BanReason,
}

#[derive(Clone, PartialEq)]
pub struct BanReason(String);

impl BanReason {
    pub fn new(s: impl ToString) -> BanReason {
        BanReason(s.to_string())
    }
}

impl Deref for BanReason {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Debug for BanReason {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&*self)
    }
}

impl Default for BanReason {
    fn default() -> Self {
        Self("Banned by an operator.".to_string())
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct PlayerEntry {
    uuid: Uuid,
    name: String,
    created: String,
    source: String,
    expires: String,
    reason: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct IpEntry {
    ip: IpAddr,
    created: String,
    source: String,
    expires: String,
    reason: String,
}

pub const DATETIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S %z";

pub fn read_banlist(server_dir: impl AsRef<Path>) -> BanList {
    let s = std::fs::read_to_string(format!(
        "{}/banned-players.json",
        server_dir.as_ref().to_str().unwrap()
    ));
    let players: Vec<PlayerEntry> = if let Ok(s) = s {
        serde_json::from_reader(Cursor::new(&s)).unwrap_or_else(|_| {
            log::warn!("Invalid banned-players.json: \n{}", s);
            Vec::new()
        })
    } else {
        Vec::new()
    };
    let players = players
        .into_iter()
        .map(|entry| BanEntry {
            value: (entry.uuid, entry.name),
            banned: DateTime::parse_from_str(&entry.created, DATETIME_FORMAT)
                .expect("Invalid datetime format in banned-players.json")
                .into(),
            source: if entry.source == "Server" {
                None
            } else {
                Some(entry.source)
            },
            expires: DateTime::parse_from_str(&entry.expires, DATETIME_FORMAT)
                .map(Into::into)
                .ok(),
            reason: BanReason(entry.reason),
        })
        .collect();

    let s = std::fs::read_to_string(format!(
        "{}/banned-ips.json",
        server_dir.as_ref().to_str().unwrap()
    ));
    let ips: Vec<IpEntry> = if let Ok(s) = s {
        serde_json::from_reader(Cursor::new(&s)).unwrap_or_else(|_| {
            log::warn!("Invalid banned-ips.json: \n{}", s);
            Vec::new()
        })
    } else {
        Vec::new()
    };
    let ips = ips
        .into_iter()
        .map(|entry| BanEntry {
            value: entry.ip,
            banned: DateTime::parse_from_str(&entry.created, DATETIME_FORMAT)
                .expect("Invalid datetime format in banned-ips.json")
                .into(),
            source: if entry.source == "Server" {
                None
            } else {
                Some(entry.source)
            },
            expires: DateTime::parse_from_str(&entry.expires, DATETIME_FORMAT)
                .map(Into::into)
                .ok(),
            reason: BanReason(entry.reason),
        })
        .collect();

    BanList {
        banned_players: players,
        banned_ips: ips,
    }
}

#[allow(unused)]
pub fn write_banlist(_server_dir: impl AsRef<Path>) {
    // There's no shutdown yet
    unimplemented!()
}
