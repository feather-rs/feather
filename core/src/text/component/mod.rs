use crate::text::{Color, Keybind, Text, Translate};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use uuid::Uuid;

mod value;
pub use value::TextValue;
pub mod builder;

pub enum Style {
    Bold,
    Italic,
    Underlined,
    Strikethrough,
    Obfuscated,
}

pub enum Reset {
    Color,
    Style,
    Insertion,
    OnClick,
    OnHover,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "action", content = "value")]
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
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Entity {
    id: Uuid,
    ty: Option<Cow<'static, str>>,
    name: Cow<'static, str>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
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

#[serde_with::skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
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

impl TextComponent {
    pub fn empty() -> TextComponent {
        TextComponent::from("")
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
        TextValue::score(name, objective, value).into()
    }

    pub fn translate_with<A, B>(translate: A, with: B) -> Self
    where
        A: Into<Translate>,
        B: IntoIterator,
        B::Item: Into<Text>,
    {
        TextValue::translate_with(translate, with).into()
    }

    pub fn keybind<A: Into<Keybind>>(keybind: A) -> Self {
        TextValue::keybind(keybind).into()
    }

    pub fn nbt<A: Into<nbt::Blob>>(nbt: A) -> Self {
        TextValue::nbt(nbt).into()
    }
}

impl Default for TextComponent {
    fn default() -> Self {
        Self::empty()
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

impl From<Color> for TextComponent {
    fn from(color: Color) -> Self {
        TextComponent::empty().color(color)
    }
}

impl From<Style> for TextComponent {
    fn from(style: Style) -> Self {
        TextComponent::empty().style(style)
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

impl<T> std::ops::Add<T> for TextComponent
where
    T: Into<Text>,
{
    type Output = Text;
    fn add(self, rhs: T) -> Self::Output {
        Text::from(self) + rhs.into()
    }
}

impl std::ops::Mul<Color> for TextComponent {
    type Output = Self;
    fn mul(self, rhs: Color) -> Self {
        self.color(rhs)
    }
}

impl std::ops::Mul<Style> for TextComponent {
    type Output = Self;
    fn mul(self, rhs: Style) -> Self {
        self.style(rhs)
    }
}

impl std::ops::Div<Style> for TextComponent {
    type Output = Self;
    fn div(self, rhs: Style) -> Self {
        self.not_style(rhs)
    }
}

impl std::ops::Div<Reset> for TextComponent {
    type Output = Self;
    fn div(self, rhs: Reset) -> Self {
        self.reset(rhs)
    }
}

impl std::ops::Mul<Color> for &str {
    type Output = TextComponent;
    fn mul(self, rhs: Color) -> Self::Output {
        TextComponent::from(String::from(self)).color(rhs)
    }
}

impl std::ops::Mul<Style> for &str {
    type Output = TextComponent;
    fn mul(self, rhs: Style) -> Self::Output {
        TextComponent::from(String::from(self)).style(rhs)
    }
}

impl std::ops::Mul<Color> for String {
    type Output = TextComponent;
    fn mul(self, rhs: Color) -> Self::Output {
        self.as_str() * rhs
    }
}

impl std::ops::Mul<Style> for String {
    type Output = TextComponent;
    fn mul(self, rhs: Style) -> Self::Output {
        self.as_str() * rhs
    }
}
