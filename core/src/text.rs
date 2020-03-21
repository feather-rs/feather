use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
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
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Style {
    Bold,
    Italic,
    Underlined,
    Strikethrough,
    Obfuscated,
}

// TODO: use instead of string as keybind
pub enum KeyBind {
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
    Iventory,
    Chat,
    ListPlayers,
    PickBlock,
    Command,
    Screenshot,
    Perspective,
    MouseSmoothing,
    Fullscreen,
    SpectatorOutlines,
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
    // Suport custom keybinds?
    Custom(String),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "action", content = "value")]
// TODO: Accept any json primitive as string
pub enum Click {
    OpenUrl(String),
    OpenFile(String),
    RunCommand(String),
    ChangePage(i32),
    SuggestCommand(String),
    CopyToClipboard(String),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "action", content = "value")]
// TODO: Accept any json primitive as string
pub enum Hover {
    #[serde(rename = "show_text")]
    ShowText(Box<RawText>),
    #[serde(rename = "show_item")]
    // TODO: Item struct 
    ShowItem(String),
    #[serde(rename = "show_entity")] 
    // TODO: Entity struct
    ShowEntity(String),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TextValue {
    Text {
        text: String,
    },
    Translate {
        // TODO: Use translate enum?
        translate: String,
        with: Vec<String>,
    },
    Score {
        name: String,
        objective: String,
        value: Option<String>,
    },
    Selector {
        selector: String,
    },
    // TODO: Use keybind
    Keybind {
        keybind: String,
    },
    // TODO: use NBT
    Nbt {
        nbt: String,
    },
}

impl<'a> From<&'a str> for TextValue {
    fn from(value: &'a str) -> Self {
        Self::text(value)
    }
}

impl TextValue {
    pub fn text(text: &str) -> Self {
        TextValue::Text { text: text.into() }
    }

    pub fn translate(translate: String, with: Vec<String>) -> Self {
        TextValue::Translate { translate, with }
    }

    pub fn score(name: String, objective: String, value: Option<String>) -> Self {
        TextValue::Score {
            name,
            objective,
            value,
        }
    }

    // TODO: Use Keybind enum
    pub fn keybind(keybind: String) -> Self {
        TextValue::Keybind { keybind }
    }

    // TODO: use Nbt struct
    pub fn nbt(nbt: String) -> Self {
        TextValue::Nbt { nbt }
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Text {
    #[serde(flatten)]
    value: TextValue,
    color: Option<Color>,
    bold: Option<bool>,
    italic: Option<bool>,
    underlined: Option<bool>,
    strikethrough: Option<bool>,
    obfuscated: Option<bool>,
    insertion: Option<String>,
    #[serde(rename = "clickEvent")] 
    click: Option<Click>,
    #[serde(rename = "hoverEvent")] 
    hover: Option<Hover>,
    extra: Option<Vec<RawText>>,
}

impl Text {
    pub fn style(mut self, style: Style) -> Self {
        let value = Some(true);
        match style {
            Style::Bold => self.bold = value,
            Style::Italic => self.italic = value,
            Style::Obfuscated => self.obfuscated = value,
            Style::Strikethrough => self.strikethrough = value,
            Style::Underlined => self.underlined = value,
        };
        self
    }

    pub fn bold(mut self) -> Self {
        self.style(Style::Bold)
    }

    pub fn italic(mut self) -> Self {
        self.style(Style::Italic)
    }

    pub fn obfuscated(mut self) -> Self {
        self.style(Style::Obfuscated)
    }

    pub fn strikethrough(mut self) -> Self {
        self.style(Style::Strikethrough)
    }

    pub fn underlined(mut self) -> Self {
        self.style(Style::Underlined)
    }

    // Colors
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    pub fn dark_red(mut self) -> Self {
        self.color(Color::DarkRed)
    }

    pub fn red(mut self) -> Self {
        self.color(Color::Red)
    }

    pub fn gold(mut self) -> Self {
        self.color(Color::Gold)
    }

    pub fn yellow(mut self) -> Self {
        self.color(Color::Yellow)
    }

    pub fn dark_green(mut self) -> Self {
        self.color(Color::DarkGreen)
    }

    pub fn green(mut self) -> Self {
        self.color(Color::Green)
    }

    pub fn aqua(mut self) -> Self {
        self.color(Color::Aqua)
    }

    pub fn dark_aqua(mut self) -> Self {
        self.color(Color::DarkAqua)
    }

    pub fn dark_blue(mut self) -> Self {
        self.color(Color::DarkBlue)
    }

    pub fn blue(mut self) -> Self {
        self.color(Color::Blue)
    }

    pub fn light_purple(mut self) -> Self {
        self.color(Color::LightPurple)
    }

    pub fn dark_purple(mut self) -> Self {
        self.color(Color::DarkPurple)
    }

    pub fn white(mut self) -> Self {
        self.color(Color::White)
    }

    pub fn gray(mut self) -> Self {
        self.color(Color::Gray)
    }

    pub fn dark_gray(mut self) -> Self {
        self.color(Color::DarkGray)
    }

    pub fn black(mut self) -> Self {
        self.color(Color::Black)
    }

    pub fn insertion(mut self, insertion: String) -> Self {
        self.insertion = Some(insertion);
        self
    }

    pub fn on_click(mut self, click: Click) -> Self {
        self.click = Some(click);
        self
    }

    pub fn on_click_change_page(mut self, page: i32) -> Self {
        self.on_click(Click::ChangePage(page))
    }

    pub fn on_click_copy_to_clipboard(mut self, to_copy: String) -> Self {
        self.on_click(Click::CopyToClipboard(to_copy))
    }

    pub fn on_click_open_file(mut self, path: String) -> Self {
        self.on_click(Click::OpenFile(path))
    }

    pub fn on_click_open_url(mut self, url: String) -> Self {
        self.on_click(Click::OpenUrl(url))
    }

    pub fn on_click_run_command(mut self, command: String) -> Self {
        self.on_click(Click::RunCommand(command))
    }
    
    pub fn on_click_(mut self, command: String) -> Self {
        self.on_click(Click::SuggestCommand(command))
    }

    pub fn on_hover(mut self, hover: Hover) -> Self {
        self.hover = Some(hover);
        self
    }

    // TODO: Entity
    pub fn on_hover_show_entity(mut self, entity: String) -> Self {
        self.on_hover(Hover::ShowEntity(entity))
    }

    // TODO: ItemStack
    pub fn on_hover_show_item(mut self, item: String) -> Self {
        self.on_hover(Hover::ShowItem(item))
    }

    // TODO: any kind of text
    pub fn on_hover_show_text<T: Into<RawText>>(mut self, text: T) -> Self {
        self.on_hover(Hover::ShowText(Box::new(text.into())))
    }

    pub fn extra(mut self, extra: Vec<RawText>) -> Self {
        self.extra = Some(extra);
        self
    }

    pub fn extra_push(mut self, extra: RawText) -> Self {
        match self.extra {
            Some(ref mut extras) => extras.push(extra),
            None => self.extra = Some(vec![extra]),
        };
        self
    }
}

impl<T> From<T> for Text
where
    T: Into<TextValue>,
{
    fn from(value: T) -> Self {
        Text {
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

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RawText {
    String(String),
    Array(Vec<RawText>),
    Object(Text),
}

impl RawText {
    pub fn text(text: &str) -> Self {
        RawText::String(text.into())
    }

    // TODO: Shrink Text::Object and Text::Array when objects only contain TextValue and turn it into Text::String
    pub fn shrink(&self) -> Self {
        unimplemented!()
    }
}

impl<T> From<T> for RawText
where
    T: Into<Text>,
{
    fn from(object: T) -> Self {
        RawText::Object(object.into())
    }
}

impl<T> From<Vec<T>> for RawText
where
    T: Into<Self> + Copy,
{
    fn from(value: Vec<T>) -> Self {
        let value: Vec<RawText> = value.iter().map(|v| (*v).into()).collect();
        RawText::Array(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::text::{Color, RawText, Text};
    use std::error::Error;

    #[test]
    pub fn text_text_single() -> Result<(), Box<dyn Error>> {
        let text_orignal = RawText::from("hello");

        let text_json = serde_json::to_string(&text_orignal)?;

        assert_eq!(&text_json, r#"{"text":"hello"}"#);

        let text: RawText = serde_json::from_str(&text_json)?;
        assert_eq!(text_orignal, text);

        Ok(())
    }

    #[test]
    fn text_text_array() -> Result<(), Box<dyn Error>> {
        let text_orignal = RawText::from(vec!["hello", "world"]);

        let text_json = serde_json::to_string(&text_orignal)?;

        assert_eq!(&text_json, r#"[{"text":"hello"},{"text":"world"}]"#);

        let text: RawText = serde_json::from_str(&text_json)?;
        assert_eq!(text_orignal, text);

        Ok(())
    }

    #[test]
    fn text_text_color() -> Result<(), Box<dyn Error>> {
        let mut text_orignal: RawText = Text::from("hello world")
            .red()
            .into();

        let text_json = serde_json::to_string(&text_orignal)?;

        assert_eq!(&text_json, r#"{"text":"hello","color":"dark_red"}"#);

        let text: RawText = serde_json::from_str(&text_json)?;
        assert_eq!(text_orignal, text);

        Ok(())
    }

    #[test]
    fn text_translate() {}
}