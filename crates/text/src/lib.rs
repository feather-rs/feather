mod click_event;
mod color;
mod content;
mod hover_event;
mod keybind;
mod style;
mod simplifier;

pub use click_event::*;
pub use color::*;
pub use content::*;
pub use hover_event::*;
pub use keybind::*;
pub use style::*;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::borrow::Cow;

/// A way to propperly serialize and deserialize extra components.
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum Extra<'a> {
    String(Cow<'a, str>),
    Object(Text<'a>),
    Array(Vec<Text<'a>>),
}

impl<'a> From<Text<'a>> for Extra<'a> {
    fn from(text: Text<'a>) -> Self {
        // Shoud be colapsed into a function on Text, just don't know what to name it.
        // !has_atributes?
        if text.color.is_none()
            && text.bold.is_none()
            && text.italic.is_none()
            && text.obfuscated.is_none()
            && text.hover_event.is_none()
            && text.extra.is_empty()
            && text.click_event.is_none()
            && text.font.is_none()
            && text.insertion.is_none()
            && text.strikethrough.is_none()
            && text.underlined.is_none()
        {
            if let Content::Text { text } = text.content {
                return Extra::String(text);
            }
        }
        // Should be colapsed into a function Text, just don't know what to name it.
        // !has_atributes_other_than_extra?
        if text.color.is_none()
            && text.bold.is_none()
            && text.italic.is_none()
            && text.obfuscated.is_none()
            && text.hover_event.is_none()
            && text.click_event.is_none()
            && text.font.is_none()
            && text.insertion.is_none()
            && text.strikethrough.is_none()
            && text.underlined.is_none()
        {
            if let Content::Text {
                text: Cow::Borrowed(""),
            } = text.content
            {
                return Extra::Array(text.extra);
            }
        }

        Extra::Object(text)
    }
}

struct ExtraVec;

impl ExtraVec {
    #[inline]
    fn serialize<'a, S>(texts: &Vec<Text<'a>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let bars: Vec<Extra> = texts.iter().map(|text| Extra::from(text.clone())).collect();
        Serialize::serialize(&bars, serializer)
    }
}

impl ExtraVec {
    #[inline]
    fn deserialize<'a, 'de, D>(deserializer: D) -> Result<Vec<Text<'a>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let extras: Vec<Extra> = Deserialize::deserialize(deserializer)?;
        let extras: Vec<Text> = extras
            .into_iter()
            .flat_map(|extra| match extra {
                Extra::String(text) => vec![Text::of(text)].into_iter(),
                Extra::Object(text) => vec![text].into_iter(),
                Extra::Array(array) => array.into_iter(),
            })
            .collect();
        Ok(extras)
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct Text<'a> {
    /// TODO
    #[serde(flatten)]
    pub content: Content<'a>,
    /// A list of additional raw JSON text components to be displayed after this one.
    /// They inherent the formatting of their parent.???
    #[serde(skip_serializing_if = "Vec::is_empty", with = "ExtraVec")]
    pub extra: Vec<Text<'a>>,
    /// The resource location of the font for this component in the resource pack within `assets/<namespace>/font`.
    /// Defaults to `"minecraft:default"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font: Option<Cow<'a, str>>,
    /// The color to render the content in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,
    /// Whether to render the content in bold.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bold: Option<bool>,
    /// Whether to render the content in italics.
    /// Note that text that is italicized by default, such as custom item names, can be unitalicized by setting this to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub italic: Option<bool>,
    /// Whether to underline the content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlined: Option<bool>,
    /// Whether to strikethrough the content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strikethrough: Option<bool>,
    /// Whether the component should randomly switch between characters of the same width.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obfuscated: Option<bool>,
    /// When the text is shift-clicked by a player, this string is inserted in their chat input.
    /// It does not overwrite any existing text the player was writing. This only works in chat messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insertion: Option<Cow<'a, str>>,
    /// Allows for events to occur when the player clicks on text. Only work in chat messages and written books, unless specified otherwise.
    #[serde(rename = "clickEvent", skip_serializing_if = "Option::is_none")]
    pub click_event: Option<ClickEvent<'a>>,
    /// Allows for a tooltip to be displayed when the player hovers their mouse over text.
    #[serde(rename = "hoverEvent", skip_serializing_if = "Option::is_none")]
    pub hover_event: Option<HoverEvent<'a>>,
}

impl<'a> Text<'a> {
    pub fn of<T: Into<Cow<'a, str>>>(text: T) -> Self {
        Text {
            content: Content::text(text),
            ..Default::default()
        }
    }

    pub fn translation<T: Into<Cow<'a, str>>>(translate: T, with: Vec<Text<'a>>) -> Self {
        Text {
            content: Content::translation(translate, with),
            ..Default::default()
        }
    }

    pub fn extra(mut self, text: Text<'a>) -> Self {
        self.extra.push(text);
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    pub fn clear_color(mut self) -> Self {
        self.color = None;
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        match style {
            Style::Obfuscated => self.obfuscated = Some(true),
            Style::Bold => self.bold = Some(true),
            Style::Strikethrough => self.strikethrough = Some(true),
            Style::Underlined => self.underlined = Some(true),
            Style::Italic => self.italic = Some(true),
        }
        self
    }

    pub fn not_style(mut self, style: Style) -> Self {
        match style {
            Style::Obfuscated => self.obfuscated = Some(false),
            Style::Bold => self.bold = Some(false),
            Style::Strikethrough => self.strikethrough = Some(false),
            Style::Underlined => self.underlined = Some(false),
            Style::Italic => self.italic = Some(false),
        }
        self
    }

    // TODO: Text::style_clear_all
    pub fn clear_style(mut self, style: Style) -> Self {
        match style {
            Style::Obfuscated => self.obfuscated = None,
            Style::Bold => self.bold = None,
            Style::Strikethrough => self.strikethrough = None,
            Style::Underlined => self.underlined = None,
            Style::Italic => self.italic = None,
        }
        self
    }

    pub fn insertion<S: Into<Cow<'a, str>>>(mut self, value: S) -> Self {
        let value = value.into();
        // TODO: is there a difference betwteen None and Some("")?
        if value.is_empty() {
            self.insertion = None;
        } else {
            self.insertion = Some(value);
        }
        self
    }

    pub fn on_click(mut self, click_event: ClickEvent<'a>) -> Self {
        self.click_event = Some(click_event);
        self
    }

    pub fn clear_on_click(mut self) -> Self {
        self.click_event = None;
        self
    }

    pub fn append(self, text: Text<'a>) -> Self {
        Text::of("").extra(self).extra(text)
    }

    pub fn has_color(&self) -> bool {
        self.color.is_none()
    }

    pub fn has_style(&self) -> bool {
        self.bold.is_some()
            && self.italic.is_some()
            && self.obfuscated.is_some()
            && self.strikethrough.is_some()
            && self.underlined.is_some()
    }

    // is_bold? Bold can either be None, Some(false), and Some(true).
    // Maybe a has_bold instead? self.bold.is_some()
}

impl<'a> From<Vec<Text<'a>>> for Text<'a> {
    fn from(extra: Vec<Text<'a>>) -> Self {
        let mut text = Text::of("");
        text.extra = extra;
        text
    }
}

// More and better tests
#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn simple() {
        assert_eq!(
            json!({
                "text": "Some basic text, with no formatting at all!",
            }),
            serde_json::to_value(Text::of("Some basic text, with no formatting at all!")).unwrap()
        );
    }

    #[test]
    fn with_color() {
        Text::of("Red colored text!").red();
        Text::of("Red colored text!").color(Color::Red);
    }

    #[test]
    fn with_single_style() {}

    #[test]
    fn multiple_styles() {}

    #[test]
    fn complex() {
        let complex_json = json!({
            "text": "Complex text",
            "extra": [{
                "text": "Un formatted extra",
            }, {
                "translate": "Formatted translation",
                "with": [],
                "color": "gold",
                "bold": true,
                "italic": false,
            }],
            "italic": true,
            "clickEvent": {
                "action": "open_url",
                "value": "https://feather.rs",
            }
        });

        let complex_text = Text::of("Complex text")
            .extra(Text::of("Un formatted extra"))
            .extra(
                Text::translation("Formatted translation", vec![])
                    .gold()
                    .bold()
                    .not_italic(),
            )
            .italic()
            .on_click_open_url("https://feather.rs");

        assert_eq!(complex_json, serde_json::to_value(complex_text).unwrap())
    }

    #[test]
    fn custom_deserializing() {
        let json_value = json!({
            "text": "Hello",
            "extra": [" ", "world"],
        });

        let json_text: Text = serde_json::from_value(json_value.clone()).unwrap();

        let text = Text::of("Hello")
            .extra(Text::of(" "))
            .extra(Text::of("world"));

        assert_eq!(json_text, text);

        let text_value = serde_json::to_value(text).unwrap();
        assert_eq!(json_value, text_value);
    }

    #[test]
    fn custom_serializing() {
        assert_eq!(
            json!({
                "text": "",
                "extra": vec!["Hello", " ", "world"],
            }),
            serde_json::to_value(Text::from(vec![
                Text::of("Hello").red(),
                Text::of(" "),
                Text::of("world").blue(),
            ]))
            .unwrap(),
        );
    }

    #[test]
    fn json() {
        let json = serde_json::to_string_pretty(&Text::of("hello world").red().bold()).unwrap();
        println!("{}", json);
    }
}
