use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use crate::Text;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "action", content = "value")]
pub enum ClickEvent<'a> {
    /// Opens the given URL in the default web browser.
    /// Ignored if the player has opted to disable links in chat; may open a GUI prompting the user if the setting for that is enabled.
    /// The link's protocol must be set and must be http or https, for security reasons.
    OpenUrl(Cow<'a, str>),
    /// In chat and written books, this value is sent in chat as though the player typed it themselves and pressed enter.
    /// If used in a book GUI, the GUI is closed after clicking.
    /// This can be used to run commands, provided the player has the required permissions.
    /// Since they are being run from chat, commands must be prefixed with the usual "/" slash.
    /// Works in signs, but only on the root text component, not on any children.
    /// Activated by using the sign and the command is run by the server at the sign's location, with the player who used the sign as @s.
    /// Since they are run by the server, sign commands have the same permission level as a command block instead of using the player's permission level,
    /// are not restricted by chat length limits, and do not need to be prefixed with a "/" slash.
    RunCommand(Cow<'a, str>),
    /// Only usable for messages in chat.
    /// Replaces the content of the chat box with the given text - usually a command, but it is not required to be a command (commands should be prefixed with /).
    SuggestCommand(Cow<'a, str>),
    /// Only usable within written books.
    /// Changes the page of the book to the given page, starting at 1.
    /// If the page is less than one or beyond the number of pages in the book, the event is ignored.
    ChangePage(u8),
}

impl<'a> ClickEvent<'a> {
    pub fn open_url<T: Into<Cow<'a, str>>>(link: T) -> Self {
        ClickEvent::OpenUrl(link.into())
    }

    pub fn run_command<T: Into<Cow<'a, str>>>(command: T) -> Self {
        ClickEvent::RunCommand(command.into())
    }

    pub fn suggest_command<T: Into<Cow<'a, str>>>(command: T) -> Self {
        ClickEvent::SuggestCommand(command.into())
    }

    pub fn change_page(page: u8) -> Self {
        ClickEvent::ChangePage(page)
    }
}

impl<'a> Text<'a> {
    pub fn on_click_open_url<T: Into<Cow<'a, str>>>(mut self, link: T) -> Self {
        self.attributes.click_event = Some(ClickEvent::open_url(link));
        self
    }

    pub fn on_click_run_command<T: Into<Cow<'a, str>>>(mut self, command: T) -> Self {
        self.attributes.click_event = Some(ClickEvent::run_command(command));
        self
    }

    pub fn on_click_suggest_command<T: Into<Cow<'a, str>>>(mut self, command: T) -> Self {
        self.attributes.click_event = Some(ClickEvent::suggest_command(command));
        self
    }

    pub fn on_click_change_page(mut self, page: u8) -> Self {
        self.attributes.click_event = Some(ClickEvent::change_page(page));
        self
    }
}
