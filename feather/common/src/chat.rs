use base::{Text, Title};

/// An entity's "mailbox" for receiving chat messages.
///
/// Internally stores a list of [`ChatMessage`]s.
/// It is up to the user to flush the mailbox.
/// (`feather-server` flushes mailboxes by sending chat packets.)
#[derive(Debug)]
pub struct ChatBox {
    messages: Vec<ChatMessage>,
    titles: Vec<Title>,
    preference: ChatPreference,
}

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
#[derive(Debug, Clone)]
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
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
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
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ChatPreference {
    /// Receive only game info messages.
    GameInfoOnly,
    /// Receive only messages from commands and game info messages.
    System,
    /// Receive all messages.
    All,
}
