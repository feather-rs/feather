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

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "action", content = "value")]
pub enum Click {
    OpenUrl(Cow<'static, str>),
    OpenFile(Cow<'static, str>),
    RunCommand(Cow<'static, str>),
    ChangePage(i32),
    SuggestCommand(Cow<'static, str>),
    CopyToClipboard(Cow<'static, str>),
}

#[serde_with::skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Entity {
    id: Uuid,
    #[serde(rename = "type")]
    kind: Option<Cow<'static, str>>,
    name: Cow<'static, str>,
}

impl Entity {
    pub fn new<A, B, C, D>(id: A, kind: B, name: C) -> Entity
    where
        A: Into<Uuid>,
        B: Into<Option<D>>,
        D: Into<Cow<'static, str>>,
        C: Into<Cow<'static, str>>,
    {
        Entity {
            id: id.into(),
            kind: kind.into().map(Into::into),
            name: name.into(),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "action", content = "value")]
pub enum Hover {
    #[serde(rename = "show_text")]
    ShowText(Box<Text>),
    #[serde(rename = "show_item")]
    ShowItem(Cow<'static, str>),
    #[serde(rename = "show_entity")]
    ShowEntity(Entity),
}

#[serde_with::skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
/// Text json object that holds all styles.
pub struct TextComponent {
    #[serde(flatten)]
    pub value: TextValue,
    pub color: Option<Color>,
    pub bold: Option<bool>,
    pub italic: Option<bool>,
    pub underlined: Option<bool>,
    pub strikethrough: Option<bool>,
    pub obfuscated: Option<bool>,
    pub insertion: Option<Cow<'static, str>>,
    pub click: Option<Click>,
    pub hover: Option<Hover>,
    pub extra: Option<Vec<Text>>,
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

    pub fn is_plain(&self) -> bool {
        self.color.is_none()
            && self.bold.is_none()
            && self.italic.is_none()
            && self.underlined.is_none()
            && self.strikethrough.is_none()
            && self.obfuscated.is_none()
            && self.insertion.is_none()
            && self.hover.is_none()
            && self.extra.is_none()
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
            Text::Array(mut arr) => {
                if arr.is_empty() {
                    TextComponent::empty()
                } else {
                    TextComponent::from(arr.remove(0)).extra(arr)
                }
            }
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
