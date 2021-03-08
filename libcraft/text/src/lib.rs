extern crate self as text;

mod ansi;
mod attributes;
mod click_event;
mod color;
mod content;
mod hover_event;
mod keybind;
mod sprintf;
mod style;

pub use ansi::*;
use anyhow::Result;
pub use attributes::*;
pub use click_event::*;
pub use color::*;
pub use content::*;
pub use hover_event::*;
pub use keybind::*;
pub use sprintf::*;
pub use style::*;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{borrow::Cow, iter};

/// A way to propperly serialize and deserialize extra components.
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum Extra<'a> {
    // Should treat numbers and bools as strings.
    String(Cow<'a, str>),
    Bool(bool),
    Object(Text<'a>),
    Array(Vec<Extra<'a>>),
}

impl<'a> From<Text<'a>> for Extra<'a> {
    fn from(text: Text<'a>) -> Self {
        if text.attributes == Attributes::default() && text.extra.is_empty() {
            if let Content::Text { text } = text.content {
                return Extra::String(text);
            }
        }
        if text.attributes == Attributes::default() {
            if let Content::Text {
                text: Cow::Borrowed(""),
            } = text.content
            {
                return Extra::Array(text.extra.into_iter().map(Extra::from).collect());
            }
        }

        Extra::Object(text)
    }
}

impl<'a> From<Extra<'a>> for Text<'a> {
    fn from(extra: Extra<'a>) -> Self {
        match extra {
            Extra::String(text) => Text::of(text),
            Extra::Object(text) => text,
            Extra::Array(extras) => {
                let mut text = Text::default();
                text.extra = extras.into_iter().map(Text::from).collect();
                text
            }
            Extra::Bool(true) => Text::of("true"),
            Extra::Bool(false) => Text::of("false"),
        }
    }
}

struct ExtraVec;

impl ExtraVec {
    fn serialize<'a, S>(texts: &Vec<Text<'a>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let bars: Vec<Extra> = texts.iter().map(|text| Extra::from(text.clone())).collect();
        Serialize::serialize(&bars, serializer)
    }
}

impl ExtraVec {
    fn deserialize<'a, 'de, D>(deserializer: D) -> Result<Vec<Text<'a>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let extras: Vec<Extra> = Deserialize::deserialize(deserializer)?;
        let extras: Vec<Text> = extras.into_iter().map(Text::from).collect();
        Ok(extras)
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct Text<'a> {
    #[serde(flatten)]
    pub content: Content<'a>,
    /// A list of additional raw JSON text components to be displayed after this one.
    /// They inherent the formatting of their parent.???
    #[serde(skip_serializing_if = "Vec::is_empty", default, with = "ExtraVec")]
    pub extra: Vec<Text<'a>>,
    #[serde(flatten)]
    pub attributes: Attributes<'a>,
}

impl<'a> Text<'a> {
    pub fn of<T>(text: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        Text {
            content: Content::text(text),
            ..Default::default()
        }
    }

    pub fn translate_with<T, Iter, Item>(translate: T, with: Iter) -> Self
    where
        T: Into<Cow<'a, str>>,
        Item: Into<Text<'a>>,
        Iter: IntoIterator<Item = Item>,
    {
        Text {
            content: Content::translation(translate, with.into_iter().map(Into::into).collect()),
            ..Default::default()
        }
    }

    pub fn extra(mut self, text: Text<'a>) -> Self {
        self.extra.push(text);
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.attributes.color = Some(color);
        self
    }

    pub fn clear_color(mut self) -> Self {
        self.attributes.color = None;
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        match style {
            Style::Obfuscated => self.attributes.obfuscated = Some(true),
            Style::Bold => self.attributes.bold = Some(true),
            Style::Strikethrough => self.attributes.strikethrough = Some(true),
            Style::Underlined => self.attributes.underlined = Some(true),
            Style::Italic => self.attributes.italic = Some(true),
        }
        self
    }

    pub fn not_style(mut self, style: Style) -> Self {
        match style {
            Style::Obfuscated => self.attributes.obfuscated = Some(false),
            Style::Bold => self.attributes.bold = Some(false),
            Style::Strikethrough => self.attributes.strikethrough = Some(false),
            Style::Underlined => self.attributes.underlined = Some(false),
            Style::Italic => self.attributes.italic = Some(false),
        }
        self
    }

    pub fn clear_style(mut self, style: Style) -> Self {
        match style {
            Style::Obfuscated => self.attributes.obfuscated = None,
            Style::Bold => self.attributes.bold = None,
            Style::Strikethrough => self.attributes.strikethrough = None,
            Style::Underlined => self.attributes.underlined = None,
            Style::Italic => self.attributes.italic = None,
        }
        self
    }

    pub fn clear_all(self) -> Self {
        self.clear_bold()
            .clear_color()
            .clear_italic()
            .clear_obfuscated()
            .clear_on_click()
            .clear_strikethrough()
            .clear_underlined()
    }

    pub fn insertion<S: Into<Cow<'a, str>>>(mut self, value: S) -> Self {
        let value = value.into();
        if value.is_empty() {
            self.attributes.insertion = None;
        } else {
            self.attributes.insertion = Some(value);
        }
        self
    }

    pub fn on_click(mut self, click_event: ClickEvent<'a>) -> Self {
        self.attributes.click_event = Some(click_event);
        self
    }

    pub fn clear_on_click(mut self) -> Self {
        self.attributes.click_event = None;
        self
    }

    pub fn append(self, text: Text<'a>) -> Self {
        Text::of("").extra(self).extra(text)
    }

    pub fn has_color(&self) -> bool {
        self.attributes.color.is_none()
    }

    pub fn iter(
        &self,
        parent: &Attributes<'a>,
    ) -> Box<dyn Iterator<Item = (&Content<'a>, Attributes<'a>)> + '_> {
        let mut attributes = self.attributes.clone();
        attributes.apply(parent);
        Box::new(
            iter::once((&self.content, attributes.clone())).chain(
                self.extra
                    .iter()
                    .flat_map(move |child| child.iter(&attributes)),
            ),
        )
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl<'a, T> From<T> for Text<'a>
where
    T: Into<Cow<'a, str>>
{
    fn from(text: T) -> Self {
        Text::of(text)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn json_null() {
        let json = json!([null]);
        assert!(serde_json::from_value::<Text>(json).is_err());
    }

    #[test]
    fn json_string() {
        let json = json!(["hello"]);
        assert_eq!(
            serde_json::from_value::<Text>(json).unwrap(),
            Text::of("hello")
        );
    }

    #[test]
    fn json_number() {
        // let json = json!([4269, 42.69]);
        // assert_eq!(
        //     serde_json::from_value::<Text>(json).unwrap(),
        //     Text::from(vec![Text::of("4269"), Text::of("42.69")])
        // )
    }
}
