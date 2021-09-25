//! Implementation of the Minecraft chat component format.

use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::borrow::Cow;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;
use uuid::Uuid;

pub mod markdown;

#[derive(Debug, thiserror::Error)]
pub enum TextConversionError {
    #[error("'{0}' is not a recognized color")]
    InvalidColor(String),
    #[error("'{0}' is not a recognized style type")]
    InvalidStyle(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Color {
    DarkRed,
    Red,
    Gold,
    Yellow,
    DarkGreen,
    Green,
    Aqua,
    DarkAqua,
    DarkBlue,
    Blue,
    LightPurple,
    DarkPurple,
    White,
    Gray,
    DarkGray,
    Black,
    Custom(String),
}

impl FromStr for Color {
    type Err = TextConversionError;

    fn from_str(s: &str) -> Result<Self, TextConversionError> {
        match s {
            "dark_red" => Ok(Color::DarkRed),
            "red" => Ok(Color::Red),
            "gold" => Ok(Color::Gold),
            "yellow" => Ok(Color::Yellow),
            "dark_green" => Ok(Color::DarkGreen),
            "green" => Ok(Color::Green),
            "aqua" => Ok(Color::Aqua),
            "dark_aqua" => Ok(Color::DarkAqua),
            "dark_blue" => Ok(Color::DarkBlue),
            "blue" => Ok(Color::Blue),
            "light_purple" => Ok(Color::LightPurple),
            "dark_purple" => Ok(Color::DarkPurple),
            "white" => Ok(Color::White),
            "gray" => Ok(Color::Gray),
            "dark_gray" => Ok(Color::DarkGray),
            "black" => Ok(Color::Black),
            _ => Err(TextConversionError::InvalidColor(s.to_string())),
        }
    }
}

impl From<Color> for Text {
    fn from(color: Color) -> Self {
        Text::empty().color(color)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Style {
    Bold,
    Italic,
    Underlined,
    Strikethrough,
    Obfuscated,
}

impl FromStr for Style {
    type Err = TextConversionError;

    fn from_str(s: &str) -> Result<Self, TextConversionError> {
        match s {
            "bold" => Ok(Style::Bold),
            "italic" => Ok(Style::Italic),
            "underline" => Ok(Style::Underlined),
            "strikethrough" => Ok(Style::Strikethrough),
            "magic" => Ok(Style::Obfuscated),
            _ => Err(TextConversionError::InvalidStyle(s.to_string())),
        }
    }
}

impl From<Style> for Text {
    fn from(style: Style) -> Self {
        Text::empty().style(style)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// Represent all possible keybinds in vanilla.
pub enum Keybind {
    Attack,
    UseItem,
    Forward,
    Left,
    Back,
    Right,
    Jump,
    Sneak,
    Sprint,
    Drop,
    Inventory,
    Chat,
    ListPlayers,
    PickBlock,
    Command,
    Screenshot,
    Perspective,
    MouseSmoothing,
    Fullscreen,
    SpectatorOutlines,
    SwapHands,
    SaveToolbar,
    LoadToolbar,
    Advancements,
    Hotbar1,
    Hotbar2,
    Hotbar3,
    Hotbar4,
    Hotbar5,
    Hotbar6,
    Hotbar7,
    Hotbar8,
    Hotbar9,
    Custom(Cow<'static, str>),
}

impl From<Keybind> for Text {
    fn from(keybind: Keybind) -> Self {
        Text::keybind(keybind)
    }
}

impl Serialize for Keybind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(String::from(self).as_ref())
    }
}

impl<'de> Deserialize<'de> for Keybind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Keybind::from(s))
    }
}

impl<T> From<T> for Keybind
where
    T: Into<Cow<'static, str>>,
{
    fn from(keybind: T) -> Self {
        let keybind = keybind.into();
        match keybind.as_ref() {
            "key_key.attack" => Keybind::Attack,
            "key_key.use" => Keybind::UseItem,
            "key_key.forward" => Keybind::Forward,
            "key_key.left" => Keybind::Left,
            "key_key.back" => Keybind::Back,
            "key_key.right" => Keybind::Right,
            "key_key.jump" => Keybind::Jump,
            "key_key.sneak" => Keybind::Sneak,
            "key_key.sprint" => Keybind::Sprint,
            "key_key.drop" => Keybind::Drop,
            "key_key.inventory" => Keybind::Inventory,
            "key_key.chat" => Keybind::Chat,
            "key_key.playerlist" => Keybind::ListPlayers,
            "key_key.pickItem" => Keybind::PickBlock,
            "key_key.command" => Keybind::Command,
            "key_key.screenshot" => Keybind::Screenshot,
            "key_key.togglePerspective" => Keybind::Perspective,
            "key_key.smoothCamera" => Keybind::MouseSmoothing,
            "key_key.fullscreen" => Keybind::Fullscreen,
            "key_key.spectatorOutlines" => Keybind::SpectatorOutlines,
            "key_key.swapHands" => Keybind::SwapHands,
            "key_key.saveToolbarActivator" => Keybind::SaveToolbar,
            "key_key.loadToolbarActivator" => Keybind::LoadToolbar,
            "key_key.advancements" => Keybind::Advancements,
            "key_key.hotbar.1" => Keybind::Hotbar1,
            "key_key.hotbar.2" => Keybind::Hotbar2,
            "key_key.hotbar.3" => Keybind::Hotbar3,
            "key_key.hotbar.4" => Keybind::Hotbar4,
            "key_key.hotbar.5" => Keybind::Hotbar5,
            "key_key.hotbar.6" => Keybind::Hotbar6,
            "key_key.hotbar.7" => Keybind::Hotbar7,
            "key_key.hotbar.8" => Keybind::Hotbar8,
            "key_key.hotbar.9" => Keybind::Hotbar9,
            _ => Keybind::Custom(keybind),
        }
    }
}

impl From<&Keybind> for String {
    fn from(keybind: &Keybind) -> Self {
        match keybind {
            Keybind::Attack => "key_key.attack",
            Keybind::UseItem => "key_key.use",
            Keybind::Forward => "key_key.forward",
            Keybind::Left => "key_key.left",
            Keybind::Back => "key_key.back",
            Keybind::Right => "key_key.right",
            Keybind::Jump => "key_key.jump",
            Keybind::Sneak => "key_key.sneak",
            Keybind::Sprint => "key_key.sprint",
            Keybind::Drop => "key_key.drop",
            Keybind::Inventory => "key_key.inventory",
            Keybind::Chat => "key_key.chat",
            Keybind::ListPlayers => "key_key.playerlist",
            Keybind::PickBlock => "key_key.pickItem",
            Keybind::Command => "key_key.command",
            Keybind::Screenshot => "key_key.screenshot",
            Keybind::Perspective => "key_key.togglePerspective",
            Keybind::MouseSmoothing => "key_key.smoothCamera",
            Keybind::Fullscreen => "key_key.fullscreen",
            Keybind::SpectatorOutlines => "key_key.spectatorOutlines",
            Keybind::SwapHands => "key_key.swapHands",
            Keybind::SaveToolbar => "key_key.saveToolbarActivator",
            Keybind::LoadToolbar => "key_key.loadToolbarActivator",
            Keybind::Advancements => "key_key.advancements",
            Keybind::Hotbar1 => "key_key.hotbar.1",
            Keybind::Hotbar2 => "key_key.hotbar.2",
            Keybind::Hotbar3 => "key_key.hotbar.3",
            Keybind::Hotbar4 => "key_key.hotbar.4",
            Keybind::Hotbar5 => "key_key.hotbar.5",
            Keybind::Hotbar6 => "key_key.hotbar.6",
            Keybind::Hotbar7 => "key_key.hotbar.7",
            Keybind::Hotbar8 => "key_key.hotbar.8",
            Keybind::Hotbar9 => "key_key.hotbar.9",
            Keybind::Custom(bind) => bind.as_ref(),
        }
        .into()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// Represent all possible translation keys in vanilla.
pub enum Translate {
    ChatTypeText,
    MultiplayerPlayerJoined,
    Custom(Cow<'static, str>),
}

impl Serialize for Translate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(String::from(self).as_ref())
    }
}

impl<'de> Deserialize<'de> for Translate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Translate::from(s))
    }
}

impl<T> std::ops::Mul<T> for Translate
where
    T: IntoIterator,
    T::Item: Into<Text>,
{
    type Output = Text;
    fn mul(self, rhs: T) -> Text {
        Text::translate_with(self, rhs)
    }
}

impl<T> From<T> for Translate
where
    T: Into<Cow<'static, str>>,
{
    fn from(value: T) -> Translate {
        let value = value.into();
        match value.as_ref() {
            "chat.type.text" => Translate::ChatTypeText,
            "multiplayer.player.joined" => Translate::MultiplayerPlayerJoined,
            _ => Translate::Custom(value),
        }
    }
}

impl<'a> From<&Translate> for String {
    fn from(translate: &Translate) -> Self {
        match translate {
            Translate::ChatTypeText => "chat.type.text",
            Translate::MultiplayerPlayerJoined => "multiplayer.player.joined",
            Translate::Custom(key) => key.as_ref(),
        }
        .into()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "action", content = "value", rename_all = "snake_case")]
// TODO: Accept any json primitive as string
pub enum Click {
    OpenUrl(Cow<'static, str>),
    OpenFile(Cow<'static, str>),
    RunCommand(Cow<'static, str>),
    ChangePage(i32),
    SuggestCommand(Cow<'static, str>),
    CopyToClipboard(Cow<'static, str>),
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Entity {
    id: Uuid,
    ty: Option<Cow<'static, str>>,
    name: Cow<'static, str>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "action", content = "value")]
// TODO: Accept any json primitive as string
pub enum Hover {
    #[serde(rename = "show_text")]
    ShowText(Box<Text>),
    #[serde(rename = "show_item")]
    // TODO: Item struct
    ShowItem(String),
    #[serde(rename = "show_entity")]
    ShowEntity(Entity),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
/// Text component can either be Text, Translate, Score, Selector, Keybind, or Nbt.
pub enum TextValue {
    Text {
        text: Cow<'static, str>,
    },
    Translate {
        translate: Translate,
        with: Vec<Text>,
    },
    Score {
        name: Cow<'static, str>,
        objective: Cow<'static, str>,
        value: Option<Cow<'static, str>>,
    },
    Selector {
        selector: Cow<'static, str>,
    },
    Keybind {
        keybind: Keybind,
    },
    Nbt {
        nbt: nbt::Blob,
    },
}

impl<T> From<T> for TextValue
where
    T: Into<Cow<'static, str>>,
{
    fn from(value: T) -> Self {
        Self::text(value.into())
    }
}

impl TextValue {
    pub fn text<T: Into<Cow<'static, str>>>(text: T) -> Self {
        TextValue::Text { text: text.into() }
    }

    pub fn translate<A>(translate: A) -> Self
    where
        A: Into<Translate>,
    {
        TextValue::Translate {
            translate: translate.into(),
            with: Default::default(),
        }
    }

    pub fn translate_with<A, B>(translate: A, with: B) -> Self
    where
        A: Into<Translate>,
        B: IntoIterator,
        B::Item: Into<Text>,
    {
        let with = with.into_iter().map(|e| e.into()).collect();
        TextValue::Translate {
            translate: translate.into(),
            with,
        }
    }

    pub fn score<
        A: Into<Cow<'static, str>>,
        B: Into<Cow<'static, str>>,
        C: Into<Cow<'static, str>>,
    >(
        name: A,
        objective: B,
        value: Option<C>,
    ) -> Self {
        TextValue::Score {
            name: name.into(),
            objective: objective.into(),
            value: value.map(|v| v.into()),
        }
    }

    pub fn keybind<A: Into<Keybind>>(keybind: A) -> Self {
        TextValue::Keybind {
            keybind: keybind.into(),
        }
    }

    pub fn nbt<A: Into<nbt::Blob>>(nbt: A) -> Self {
        TextValue::Nbt { nbt: nbt.into() }
    }
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
/// Text json object that holds all styles.
pub struct TextComponent {
    #[serde(flatten)]
    value: TextValue,
    color: Option<Color>,
    bold: Option<bool>,
    italic: Option<bool>,
    underlined: Option<bool>,
    strikethrough: Option<bool>,
    obfuscated: Option<bool>,
    insertion: Option<Cow<'static, str>>,
    #[serde(rename = "clickEvent")]
    click: Option<Click>,
    #[serde(rename = "hoverEvent")]
    hover: Option<Hover>,
    extra: Option<Vec<Text>>,
}

impl Default for TextComponent {
    fn default() -> Self {
        Self::empty()
    }
}

pub trait IntoTextComponent {
    fn into_component(self) -> TextComponent;
}

impl TextComponent {
    pub fn empty() -> TextComponent {
        TextComponent::from("")
    }
}

pub enum Reset {
    Color,
    Style,
    Insertion,
    OnClick,
    OnHover,
}

/// Text component interface.
pub trait TextComponentBuilder {
    /// Sets the given style to either None, true, or false.
    fn set_style(self, style: Style, value: Option<bool>) -> Self;

    /// Applies the given style.
    fn style(self, style: Style) -> Self;
    fn bold(self) -> Self;
    fn italic(self) -> Self;
    fn obfuscated(self) -> Self;
    fn strikethrough(self) -> Self;
    fn underlined(self) -> Self;

    /// Removes the given style.
    fn not_style(self, style: Style) -> Self;
    fn not_bold(self) -> Self;
    fn not_italic(self) -> Self;
    fn not_obfuscated(self) -> Self;
    fn not_strikethrough(self) -> Self;
    fn not_underlined(self) -> Self;

    /// Resets the given style; the parent's color will be inherited.
    fn reset_style(self, style: Style) -> Self;
    fn reset_bold(self) -> Self;
    fn reset_italic(self) -> Self;
    fn reset_obfuscated(self) -> Self;
    fn reset_strikethrough(self) -> Self;
    fn reset_underlined(self) -> Self;
    fn reset_style_all(self) -> Self;

    /// Aplies the given color.
    fn color(self, color: Color) -> Self;
    fn dark_red(self) -> Self;
    fn red(self) -> Self;
    fn gold(self) -> Self;
    fn yellow(self) -> Self;
    fn dark_green(self) -> Self;
    fn green(self) -> Self;
    fn aqua(self) -> Self;
    fn dark_aqua(self) -> Self;
    fn dark_blue(self) -> Self;
    fn blue(self) -> Self;
    fn light_purple(self) -> Self;
    fn dark_purple(self) -> Self;
    fn white(self) -> Self;
    fn gray(self) -> Self;
    fn dark_gray(self) -> Self;
    fn black(self) -> Self;

    /// Resets the given color; the parent's color will be inherited.
    fn reset_color(self) -> Self;

    /// Inserts the given text into the chat, when shift is held and clicked.
    /// Only useable for messages in chat.
    fn insertion<A: Into<Cow<'static, str>>>(self, insertion: A) -> Self;

    /// Resets the insertions.
    fn reset_insertion(self) -> Self;

    fn on_click(self, click: Click) -> Self;
    fn on_click_change_page(self, page: i32) -> Self;
    fn on_click_copy_to_clipboard<A: Into<Cow<'static, str>>>(self, to_copy: A) -> Self;
    /// Can only be used on the client.
    fn on_click_open_file<A: Into<Cow<'static, str>>>(self, path: A) -> Self;
    fn on_click_open_url<A: Into<Cow<'static, str>>>(self, url: A) -> Self;
    fn on_click_run_command<A: Into<Cow<'static, str>>>(self, command: A) -> Self;
    /// Only useable for messages in chat.
    fn on_click_suggest_command<A: Into<Cow<'static, str>>>(self, command: A) -> Self;

    fn reset_on_click(self) -> Self;

    fn on_hover(self, hover: Hover) -> Self;
    fn on_hover_show_entity<A: Into<Entity>>(self, entity: A) -> Self;
    fn on_hover_show_item(self, item: String) -> Self;
    fn on_hover_show_text<A: Into<Text>>(self, text: A) -> Self;

    fn reset_on_hover(self) -> Self;

    /// Inherited Text; they will inherent the parent's style, color, insertion, on_click, and on_hover.
    fn extra<A>(self, extra: A) -> Self
    where
        A: IntoIterator,
        A::Item: Into<Text>;
    fn push_extra<A: Into<Text>>(self, extra: A) -> Self;

    fn reset_extra(self) -> Self;

    /// Will inherent the parent's style, color, insertion, on_click, and on_hover.
    fn reset_all(self) -> Self;

    /// Aplies the given reset
    fn reset(self, reset: Reset) -> Self;
}

impl IntoTextComponent for TextComponent {
    fn into_component(self) -> TextComponent {
        self
    }
}

impl<T> TextComponentBuilder for T
where
    T: IntoTextComponent + From<TextComponent>,
{
    fn set_style(self, style: Style, value: Option<bool>) -> Self {
        let mut component = self.into_component();
        match style {
            Style::Bold => component.bold = value,
            Style::Italic => component.italic = value,
            Style::Obfuscated => component.obfuscated = value,
            Style::Strikethrough => component.strikethrough = value,
            Style::Underlined => component.underlined = value,
        };
        component.into()
    }

    fn style(self, style: Style) -> Self {
        self.set_style(style, Some(true))
    }

    fn bold(self) -> Self {
        self.style(Style::Bold)
    }

    fn italic(self) -> Self {
        self.style(Style::Italic)
    }

    fn obfuscated(self) -> Self {
        self.style(Style::Obfuscated)
    }

    fn strikethrough(self) -> Self {
        self.style(Style::Strikethrough)
    }

    fn underlined(self) -> Self {
        self.style(Style::Underlined)
    }

    fn not_style(self, style: Style) -> Self {
        self.set_style(style, None)
    }

    fn not_bold(self) -> Self {
        self.style(Style::Bold)
    }

    fn not_italic(self) -> Self {
        self.style(Style::Italic)
    }

    fn not_obfuscated(self) -> Self {
        self.style(Style::Obfuscated)
    }

    fn not_strikethrough(self) -> Self {
        self.style(Style::Strikethrough)
    }

    fn not_underlined(self) -> Self {
        self.style(Style::Underlined)
    }

    fn reset_style(self, style: Style) -> Self {
        self.set_style(style, None)
    }

    fn reset_bold(self) -> Self {
        self.style(Style::Bold)
    }

    fn reset_italic(self) -> Self {
        self.style(Style::Italic)
    }

    fn reset_obfuscated(self) -> Self {
        self.style(Style::Obfuscated)
    }

    fn reset_strikethrough(self) -> Self {
        self.style(Style::Strikethrough)
    }

    fn reset_underlined(self) -> Self {
        self.style(Style::Underlined)
    }

    fn reset_style_all(self) -> Self {
        let mut component = self.into_component();
        component.bold = None;
        component.italic = None;
        component.obfuscated = None;
        component.strikethrough = None;
        component.underlined = None;
        component.into()
    }

    fn color(self, color: Color) -> Self {
        let mut component = self.into_component();
        component.color = Some(color);
        component.into()
    }

    fn dark_red(self) -> Self {
        self.color(Color::DarkRed)
    }

    fn red(self) -> Self {
        self.color(Color::Red)
    }

    fn gold(self) -> Self {
        self.color(Color::Gold)
    }

    fn yellow(self) -> Self {
        self.color(Color::Yellow)
    }

    fn dark_green(self) -> Self {
        self.color(Color::DarkGreen)
    }

    fn green(self) -> Self {
        self.color(Color::Green)
    }

    fn aqua(self) -> Self {
        self.color(Color::Aqua)
    }

    fn dark_aqua(self) -> Self {
        self.color(Color::DarkAqua)
    }

    fn dark_blue(self) -> Self {
        self.color(Color::DarkBlue)
    }

    fn blue(self) -> Self {
        self.color(Color::Blue)
    }

    fn light_purple(self) -> Self {
        self.color(Color::LightPurple)
    }

    fn dark_purple(self) -> Self {
        self.color(Color::DarkPurple)
    }

    fn white(self) -> Self {
        self.color(Color::White)
    }

    fn gray(self) -> Self {
        self.color(Color::Gray)
    }

    fn dark_gray(self) -> Self {
        self.color(Color::DarkGray)
    }

    fn black(self) -> Self {
        self.color(Color::Black)
    }

    fn reset_color(self) -> Self {
        let mut component = self.into_component();
        component.color = None;
        component.into()
    }

    fn insertion<A: Into<Cow<'static, str>>>(self, insertion: A) -> Self {
        let mut component = self.into_component();
        component.insertion = Some(insertion.into());
        component.into()
    }

    fn reset_insertion(self) -> Self {
        let mut component = self.into_component();
        component.insertion = None;
        component.into()
    }

    fn on_click(self, click: Click) -> Self {
        let mut component = self.into_component();
        component.click = Some(click);
        component.into()
    }

    fn on_click_change_page(self, page: i32) -> Self {
        self.on_click(Click::ChangePage(page))
    }

    fn on_click_copy_to_clipboard<A: Into<Cow<'static, str>>>(self, to_copy: A) -> Self {
        self.on_click(Click::CopyToClipboard(to_copy.into()))
    }

    fn on_click_open_file<A: Into<Cow<'static, str>>>(self, path: A) -> Self {
        self.on_click(Click::OpenFile(path.into()))
    }

    fn on_click_open_url<A: Into<Cow<'static, str>>>(self, url: A) -> Self {
        self.on_click(Click::OpenUrl(url.into()))
    }

    fn on_click_run_command<A: Into<Cow<'static, str>>>(self, command: A) -> Self {
        self.on_click(Click::RunCommand(command.into()))
    }
    fn on_click_suggest_command<A: Into<Cow<'static, str>>>(self, command: A) -> Self {
        self.on_click(Click::SuggestCommand(command.into()))
    }

    fn reset_on_click(self) -> Self {
        let mut component = self.into_component();
        component.click = None;
        component.into()
    }

    fn on_hover(self, hover: Hover) -> Self {
        let mut component = self.into_component();
        component.hover = Some(hover);
        component.into()
    }

    fn on_hover_show_entity<A: Into<Entity>>(self, entity: A) -> Self {
        self.on_hover(Hover::ShowEntity(entity.into()))
    }

    fn on_hover_show_item(self, item: String) -> Self {
        self.on_hover(Hover::ShowItem(item))
    }

    fn on_hover_show_text<A: Into<Text>>(self, text: A) -> Self {
        self.on_hover(Hover::ShowText(Box::new(text.into())))
    }

    fn reset_on_hover(self) -> Self {
        let mut component = self.into_component();
        component.hover = None;
        component.into()
    }

    fn extra<A>(self, extra: A) -> Self
    where
        A: IntoIterator,
        A::Item: Into<Text>,
    {
        let mut component = self.into_component();
        component.extra = Some(extra.into_iter().map(|e| e.into()).collect());
        component.into()
    }

    fn push_extra<A: Into<Text>>(self, extra: A) -> Self {
        let mut component = self.into_component();
        match component.extra {
            Some(ref mut extras) => extras.push(extra.into()),
            None => component.extra = Some(vec![extra.into()]),
        };
        component.into()
    }

    fn reset_extra(self) -> Self {
        let mut component = self.into_component();
        component.extra = None;
        component.into()
    }

    fn reset_all(self) -> Self {
        let mut component = self.into_component();
        component.color = None;
        component.bold = None;
        component.italic = None;
        component.underlined = None;
        component.strikethrough = None;
        component.obfuscated = None;
        component.insertion = None;
        component.click = None;
        component.hover = None;
        component.extra = None;
        component.into()
    }

    fn reset(self, reset: Reset) -> Self {
        match reset {
            Reset::Color => self.reset_color(),
            Reset::Insertion => self.reset_insertion(),
            Reset::OnClick => self.reset_on_click(),
            Reset::OnHover => self.reset_on_hover(),
            Reset::Style => self.reset_style_all(),
        }
    }
}

impl<T> From<T> for TextComponent
where
    T: Into<TextValue>,
{
    fn from(value: T) -> Self {
        TextComponent {
            value: value.into(),
            color: None,
            bold: None,
            italic: None,
            underlined: None,
            strikethrough: None,
            obfuscated: None,
            insertion: None,
            click: None,
            hover: None,
            extra: None,
        }
    }
}

impl From<Text> for TextComponent {
    fn from(value: Text) -> Self {
        match value {
            Text::String(s) => TextComponent::from(s),
            Text::Component(c) => *c,
            Text::Array(arr) => TextComponent::from("").extra(arr),
        }
    }
}

/// Text can either be a json String, Object, or an Array.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Text {
    String(Cow<'static, str>),
    Array(Vec<Text>),
    Component(Box<TextComponent>),
}

impl IntoTextComponent for Text {
    fn into_component(self) -> TextComponent {
        match self {
            Text::Component(c) => *c,
            Text::String(text) => TextComponent::from(text),
            Text::Array(arr) => TextComponent::empty().extra(arr),
        }
    }
}

impl Text {
    pub fn empty() -> Self {
        Self::from("")
    }

    pub fn of<A: Into<Cow<'static, str>>>(text: A) -> Self {
        Text::from(text)
    }

    pub fn translate_with<A, B>(translate: A, with: B) -> Self
    where
        A: Into<Translate>,
        B: IntoIterator,
        B::Item: Into<Text>,
    {
        Text::from(TextValue::translate_with(translate, with))
    }

    pub fn score<
        A: Into<Cow<'static, str>>,
        B: Into<Cow<'static, str>>,
        C: Into<Cow<'static, str>>,
    >(
        name: A,
        objective: B,
        value: Option<C>,
    ) -> Text {
        Text::from(TextValue::score(name, objective, value))
    }

    pub fn keybind<A: Into<Keybind>>(keybind: A) -> Text {
        Text::from(TextValue::keybind(keybind))
    }

    pub fn nbt<A: Into<nbt::Blob>>(nbt: A) -> Text {
        Text::from(TextValue::nbt(nbt))
    }
}

impl From<Text> for String {
    fn from(text: Text) -> Self {
        TextRoot(text).into()
    }
}

impl Display for Text {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(&serde_json::to_string(self).unwrap())
    }
}

impl From<TextComponent> for Text {
    fn from(component: TextComponent) -> Self {
        Text::Component(Box::new(component))
    }
}

impl From<TextValue> for Text {
    fn from(value: TextValue) -> Self {
        Text::from(TextComponent::from(value))
    }
}

impl<T> From<T> for Text
where
    T: Into<Cow<'static, str>>,
{
    fn from(value: T) -> Self {
        Text::String(value.into())
    }
}

impl std::ops::Add<TextComponent> for Text {
    type Output = Text;
    fn add(self, rhs: TextComponent) -> Text {
        self + Text::from(rhs)
    }
}

impl std::ops::Add<Text> for Text {
    type Output = Text;
    fn add(mut self, rhs: Text) -> Text {
        match self {
            s @ Text::String(_) => Text::Array(vec![s, rhs]),
            c @ Text::Component(_) => Text::Array(vec![Text::empty(), c, rhs]),
            Text::Array(ref mut inner) => {
                inner.push(rhs);
                self
            }
        }
    }
}

/// A `Deserialize` impl for `Text` which uses the text markdown format.
pub fn deserialize_text<'de, D>(deserializer: D) -> Result<Text, D::Error>
where
    D: Deserializer<'de>,
{
    let string = String::deserialize(deserializer)?;

    let component = markdown::translator::translate_text(&string)
        .map_err(|e| de::Error::custom(e.to_string()))?;
    Ok(Text::Component(Box::new(component)))
}

/// Ensures Text is either an Array or Object.
/// This is required at some places when sending to the client.
pub struct TextRoot(Text);

impl From<TextRoot> for String {
    fn from(text: TextRoot) -> String {
        text.0.to_string()
    }
}

impl<T> From<T> for TextRoot
where
    T: Into<Text>,
{
    fn from(text: T) -> Self {
        match text.into() {
            s @ Text::String(_) => TextRoot(s.into_component().into()),
            c @ Text::Component(_) => TextRoot(c),
            Text::Array(arr) if arr.is_empty() => TextRoot(Text::empty()),
            arr @ Text::Array(_) => TextRoot(arr),
        }
    }
}

impl IntoTextComponent for TextRoot {
    fn into_component(self) -> TextComponent {
        self.0.into_component()
    }
}

macro_rules! impl_operators {
    ($ty:ident) => {
        impl std::ops::Mul<Color> for $ty {
            type Output = Self;
            fn mul(self, rhs: Color) -> Self {
                self.color(rhs)
            }
        }

        impl std::ops::Mul<Style> for $ty {
            type Output = Self;
            fn mul(self, rhs: Style) -> Self {
                self.style(rhs)
            }
        }

        impl std::ops::Div<Style> for $ty {
            type Output = Self;
            fn div(self, rhs: Style) -> Self {
                self.not_style(rhs)
            }
        }

        impl std::ops::Div<Reset> for $ty {
            type Output = Self;
            fn div(self, rhs: Reset) -> Self {
                self.reset(rhs)
            }
        }
    };
    ($($ty:ident),+) => {
        $(
            impl_operators!($ty);
        )+
    }
}

impl_operators!(TextRoot, Text, TextComponent);

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    pub fn text_text_single() -> Result<(), Box<dyn Error>> {
        let text_orignal: Text = Text::from("hello");

        let text_json = serde_json::to_string(&text_orignal)?;

        assert_eq!(&text_json, r#""hello""#);

        let text: Text = serde_json::from_str(&text_json)?;
        assert_eq!(text_orignal, text);

        Ok(())
    }

    #[test]
    fn text_text_array() -> Result<(), Box<dyn Error>> {
        let text_orignal = Text::from("hello") + Text::from(" ") + Text::from("world!");

        let text_json = serde_json::to_string(&text_orignal)?;

        assert_eq!(&text_json, r#"["hello"," ","world!"]"#);

        let text: Text = serde_json::from_str(&text_json)?;
        assert_eq!(text_orignal, text);

        Ok(())
    }

    #[test]
    fn text_text_color() -> Result<(), Box<dyn Error>> {
        let text_original: Text = Text::from("hello world") * Color::DarkRed;

        let text_json = serde_json::to_string(&text_original)?;

        assert_eq!(&text_json, r#"{"text":"hello world","color":"dark_red"}"#);

        let text: Text = serde_json::from_str(&text_json)?;
        assert_eq!(text_original, text);

        Ok(())
    }

    #[test]
    fn text_hello_space_world() -> Result<(), Box<dyn Error>> {
        let hello: Text = Text::from("hello") * Color::Red * Style::Italic * Style::Bold;
        let space: Text = Text::from(" ");
        let world: Text = Text::from("world") * Color::Blue * Style::Bold;
        let hello_space_world: Text = hello + space + world;

        let text_json = serde_json::to_string(&hello_space_world)?;

        assert_eq!(
            text_json,
            r#"["",{"text":"hello","color":"red","bold":true,"italic":true}," ",{"text":"world","color":"blue","bold":true}]"#
        );

        Ok(())
    }

    #[test]
    fn text_translate() -> Result<(), Box<dyn Error>> {
        let join =
            Translate::from("multiplayer.player.joined") * vec!["The_Defman"] * Color::Yellow;

        let text_json = serde_json::to_string(&join)?;

        assert_eq!(
            text_json,
            r#"{"translate":"multiplayer.player.joined","with":["The_Defman"],"color":"yellow"}"#
        );

        let join = Translate::MultiplayerPlayerJoined * vec!["The_Defman"] * Color::Yellow;

        let text_json = serde_json::to_string(&join)?;

        assert_eq!(
            text_json,
            r#"{"translate":"multiplayer.player.joined","with":["The_Defman"],"color":"yellow"}"#
        );

        Ok(())
    }

    #[test]
    fn text_root() {
        let hello = Text::from("hello");

        let root = TextRoot::from(hello);

        let root_json = String::from(root);

        assert_eq!(root_json, r#"{"text":"hello"}"#);
    }

    #[test]
    fn text_hover_and_click() -> Result<(), Box<dyn Error>> {
        let text = Text::from("hello")
            .on_hover_show_text("hover")
            .on_click_run_command("/say hello");
        assert_eq!(
            serde_json::to_string(&text)?,
            r#"{"text":"hello","clickEvent":{"action":"run_command","value":"/say hello"},"hoverEvent":{"action":"show_text","value":"hover"}}"#
        );

        Ok(())
    }
}
