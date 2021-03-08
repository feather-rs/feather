use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::{ClickEvent, Color, HoverEvent};

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct Attributes<'a> {
    /// The resource location of the font for this component in the resource pack within `assets/<namespace>/font`.
    /// Defaults to `"minecraft:default"`.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub font: Option<Cow<'a, str>>,
    /// The color to render the content in.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub color: Option<Color>,
    /// Whether to render the content in bold.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub bold: Option<bool>,
    /// Whether to render the content in italics.
    /// Note that text that is italicized by default, such as custom item names, can be unitalicized by setting this to false.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub italic: Option<bool>,
    /// Whether to underline the content.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub underlined: Option<bool>,
    /// Whether to strikethrough the content.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub strikethrough: Option<bool>,
    /// Whether the component should randomly switch between characters of the same width.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub obfuscated: Option<bool>,
    /// When the text is shift-clicked by a player, this string is inserted in their chat input.
    /// It does not overwrite any existing text the player was writing. This only works in chat messages.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub insertion: Option<Cow<'a, str>>,
    /// Allows for events to occur when the player clicks on text. Only work in chat messages and written books, unless specified otherwise.
    #[serde(
        rename = "clickEvent",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub click_event: Option<ClickEvent<'a>>,
    /// Allows for a tooltip to be displayed when the player hovers their mouse over text.
    #[serde(
        rename = "hoverEvent",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub hover_event: Option<HoverEvent<'a>>,
}

macro_rules! apply_attributes {
    [$($attribute:ident),*$(,)?] => {
        impl<'a> Attributes<'a> {
            pub fn apply(&mut self, other: &Attributes<'a>) {
                $(
                if self.$attribute.is_none() {
                    self.$attribute = other.$attribute.clone();
                }
                )*
            }
        }
    };
}

apply_attributes![
    font,
    color,
    bold,
    italic,
    underlined,
    strikethrough,
    obfuscated,
    insertion,
    click_event,
    hover_event,
];
