//! Components not associated with a specific type of entity.
//!
//! See the [entities module](crate::entities) for entity-specific
//! components.

use std::fmt::Display;
use std::net::IpAddr;
use std::sync::atomic::{AtomicI32, Ordering};

use serde::{Deserialize, Serialize};
use smartstring::{LazyCompact, SmartString};

pub use libcraft_core::Gamemode;
use libcraft_text::{Text, Title};

/// Whether an entity is touching the ground.
#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    derive_more::Deref,
    derive_more::DerefMut,
)]
pub struct OnGround(pub bool);

bincode_component_impl!(OnGround);

/// A player's username.
///
/// This component is immutable. Do not
/// attempt to change it.
///
/// Non-player entities cannot have this component. See [`CustomName`]
/// if you need to name an entity.
#[derive(Clone, Debug, Serialize, Deserialize, derive_more::Deref)]
pub struct Name(SmartString<LazyCompact>);

bincode_component_impl!(Name);

impl Name {
    pub fn new(string: &str) -> Self {
        Self(string.into())
    }

    pub fn as_str(&self) -> &str {
        &*self
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

/// An entity's custom name.
///
/// Adding this component to an entity
/// will give it a custom name, visible on the client.
///
/// Giving a player a custom name has no effect.
#[derive(Clone, Debug, Serialize, Deserialize, derive_more::Deref, derive_more::DerefMut)]
pub struct CustomName(SmartString<LazyCompact>);

bincode_component_impl!(CustomName);

impl CustomName {
    /// Creates a custom name from a string.
    pub fn new(string: &str) -> Self {
        Self(string.into())
    }

    pub fn as_str(&self) -> &str {
        &*self
    }

    pub fn as_mut_str(&mut self) -> &mut str {
        &mut *self
    }
}

impl Display for CustomName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

/// A player's walk speed
#[derive(
    Copy, Clone, Debug, PartialEq, Serialize, Deserialize, derive_more::Deref, derive_more::DerefMut,
)]
pub struct WalkSpeed(pub f32);

bincode_component_impl!(WalkSpeed);

impl Default for WalkSpeed {
    fn default() -> Self {
        WalkSpeed(0.1)
    }
}

/// A player's fly speed
#[derive(
    Copy, Clone, Debug, PartialEq, Serialize, Deserialize, derive_more::Deref, derive_more::DerefMut,
)]
pub struct CreativeFlyingSpeed(pub f32);

bincode_component_impl!(CreativeFlyingSpeed);

impl Default for CreativeFlyingSpeed {
    fn default() -> Self {
        CreativeFlyingSpeed(0.05)
    }
}

/// Whether a player can fly like in creative mode
#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    derive_more::Deref,
    derive_more::DerefMut,
)]
pub struct CanCreativeFly(pub bool);

bincode_component_impl!(CanCreativeFly);

/// Whether a player is flying (like in creative mode, so it does not reflect if the player is flying by other means)
#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    derive_more::Deref,
    derive_more::DerefMut,
)]
pub struct CreativeFlying(pub bool);

bincode_component_impl!(CreativeFlying);

/// Whether a player can place and destroy blocks
#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    derive_more::Deref,
    derive_more::DerefMut,
)]
pub struct CanBuild(pub bool);

bincode_component_impl!(CanBuild);

/// Whether a player breaks blocks instantly (like in creative mode)
#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    derive_more::Deref,
    derive_more::DerefMut,
)]
pub struct Instabreak(pub bool);

bincode_component_impl!(Instabreak);

/// Whether a player is immune to damage
#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    derive_more::Deref,
    derive_more::DerefMut,
)]
pub struct Invulnerable(pub bool);

bincode_component_impl!(Invulnerable);

/// Whether an entity is sneaking, like in pressing shift.
#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    derive_more::Deref,
    derive_more::DerefMut,
)]
pub struct Sneaking(pub bool);
bincode_component_impl!(Sneaking);

/// A player's previous gamemode
#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    derive_more::Deref,
    derive_more::DerefMut,
)]
pub struct PreviousGamemode(pub Option<Gamemode>);

bincode_component_impl!(PreviousGamemode);

impl PreviousGamemode {
    /// Gets a previous gamemode from its ID.
    pub fn from_id(id: i8) -> Self {
        PreviousGamemode(match id {
            0 => Some(Gamemode::Survival),
            1 => Some(Gamemode::Creative),
            2 => Some(Gamemode::Adventure),
            3 => Some(Gamemode::Spectator),
            _ => None,
        })
    }

    /// Gets this gamemode's id
    pub fn id(&self) -> i8 {
        match self.0 {
            Some(Gamemode::Survival) => 0,
            Some(Gamemode::Creative) => 1,
            Some(Gamemode::Adventure) => 2,
            Some(Gamemode::Spectator) => 3,
            None => -1,
        }
    }
}

/// Represents an entity's health
#[derive(
    Copy, Clone, Debug, PartialEq, Serialize, Deserialize, derive_more::Deref, derive_more::DerefMut,
)]
pub struct Health(pub f32);
bincode_component_impl!(Health);

/// A component on players that tracks if they are sprinting or not.
#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    derive_more::Deref,
    derive_more::DerefMut,
)]
pub struct Sprinting(pub bool);
impl Sprinting {
    pub fn new(value: bool) -> Self {
        Sprinting(value)
    }
}
bincode_component_impl!(Sprinting);

/// An entity's ID used by the protocol
/// in `entity_id` fields.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct NetworkId(pub i32);

impl NetworkId {
    /// Creates a new, unique network ID.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        static NEXT: AtomicI32 = AtomicI32::new(0);
        // In theory, this can overflow if the server
        // creates 4 billion entities. The hope is that
        // old entities will have died out at that point.
        Self(NEXT.fetch_add(1, Ordering::SeqCst))
    }
}

/// ID of a client. Can be reused.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ClientId(pub usize);

/// An entity's "mailbox" for receiving chat messages.
///
/// Internally stores a list of [`ChatMessage`]s.
/// It is up to the user to flush the mailbox.
/// (`feather-server` flushes mailboxes by sending chat packets.)
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatBox {
    messages: Vec<ChatMessage>,
    titles: Vec<Title>,
    preference: ChatPreference,
}

bincode_component_impl!(ChatBox);

impl ChatBox {
    pub fn new(preference: ChatPreference) -> Self {
        Self {
            messages: Vec::new(),
            titles: Vec::new(),
            preference,
        }
    }

    pub fn set_preference(&mut self, preference: ChatPreference) {
        self.preference = preference;
    }

    pub fn send(&mut self, message: ChatMessage) {
        self.messages.push(message);
    }

    pub fn send_chat(&mut self, message: impl Into<Text>) {
        self.send(ChatMessage::new(ChatKind::PlayerChat, message.into()));
    }

    pub fn send_system(&mut self, message: impl Into<Text>) {
        self.send(ChatMessage::new(ChatKind::System, message.into()));
    }

    pub fn send_above_hotbar(&mut self, message: impl Into<Text>) {
        self.send(ChatMessage::new(ChatKind::AboveHotbar, message.into()));
    }

    /// Adds the [`Title`] to the title queue.
    pub fn send_title(&mut self, title: Title) {
        self.titles.push(title);
    }

    /// Drains titles in the mailbox
    pub fn drain_titles(&mut self) -> impl Iterator<Item = Title> + '_ {
        self.titles.drain(..)
    }

    /// Drains messages in the mailbox.
    pub fn drain(&mut self) -> impl Iterator<Item = ChatMessage> + '_ {
        let preference = self.preference;
        self.messages
            .drain(..)
            .filter(move |msg| msg.kind.should_send(preference))
    }
}

/// Represents a chat message.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    kind: ChatKind,
    message: Text,
}

impl ChatMessage {
    pub fn new(kind: ChatKind, message: Text) -> Self {
        Self { kind, message }
    }

    pub fn kind(&self) -> ChatKind {
        self.kind
    }

    pub fn text(&self) -> &Text {
        &self.message
    }
}

/// Kind of chat message. The client determines whether
/// to display a message based on this kind.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChatKind {
    /// A player chat message or similar.
    PlayerChat,
    /// The output of a command or other messages
    /// not originating from players.
    System,
    /// A message displayed above the hotbar.
    AboveHotbar,
}

impl ChatKind {
    pub fn should_send(self, preference: ChatPreference) -> bool {
        match self {
            ChatKind::PlayerChat => preference == ChatPreference::All,
            ChatKind::System => preference >= ChatPreference::System,
            ChatKind::AboveHotbar => true,
        }
    }
}

/// A player's chat preference.
/// Determines which [`ChatKind`]s will
/// be sent to this player.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ChatPreference {
    /// Receive only game info messages.
    GameInfoOnly,
    /// Receive only messages from commands and game info messages.
    System,
    /// Receive all messages.
    All,
}

/// A player's real ip address
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, derive_more::Deref)]
pub struct RealIp(pub IpAddr);

bincode_component_impl!(RealIp);

/// Marker component for the console entity.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Console;

bincode_component_impl!(Console);

/// A player's previous gamemode
#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    derive_more::Deref,
    derive_more::DerefMut,
)]
pub struct DefaultGamemode(Gamemode);

impl DefaultGamemode {
    pub fn new(gamemode: Gamemode) -> DefaultGamemode {
        DefaultGamemode(gamemode)
    }
}

bincode_component_impl!(DefaultGamemode);
