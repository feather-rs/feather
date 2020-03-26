use serde::{Deserialize, Serialize};
use std::borrow::Cow;

mod constants;
pub use constants::{Color, Keybind, Translate};
mod component;
pub use component::{Click, Entity, Hover, Reset, Style, TextComponent, TextValue};
mod language;
mod simplification;
pub use simplification::Simplification;

impl<T> std::ops::Mul<T> for Translate
where
    T: IntoIterator,
    T::Item: Into<Text>,
{
    type Output = TextComponent;
    fn mul(self, rhs: T) -> TextComponent {
        TextComponent::translate_with(self, rhs)
    }
}

/// Text can either be a json String, Object, or an Array.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Text {
    String(Cow<'static, str>),
    Array(Vec<Text>),
    Component(Box<TextComponent>),
}

impl Text {
    pub fn empty() -> Self {
        Self::from("")
    }

    pub fn of<A: Into<Cow<'static, str>>>(text: A) -> Self {
        Text::from(text)
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
        Text::from(TextValue::score(name, objective, value))
    }

    pub fn translate_with<A, B>(translate: A, with: B) -> Self
    where
        A: Into<Translate>,
        B: IntoIterator,
        B::Item: Into<Text>,
    {
        Text::from(TextValue::translate_with(translate, with))
    }

    pub fn keybind<A: Into<Keybind>>(keybind: A) -> Self {
        Text::from(TextValue::keybind(keybind))
    }

    pub fn nbt<A: Into<nbt::Blob>>(nbt: A) -> Self {
        Text::from(TextValue::nbt(nbt))
    }

    pub fn from_json(json: &str) -> serde_json::Result<Text> {
        serde_json::from_str(json)
    }
}

impl From<Color> for Text {
    fn from(color: Color) -> Self {
        TextComponent::from(color).into()
    }
}

impl From<Style> for Text {
    fn from(style: Style) -> Self {
        TextComponent::from(style).into()
    }
}

impl From<Keybind> for Text {
    fn from(keybind: Keybind) -> Self {
        Text::keybind(keybind)
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

impl From<&Text> for serde_json::Value {
    fn from(text: &Text) -> Self {
        serde_json::to_value(text).unwrap()
    }
}

impl From<Text> for serde_json::Value {
    fn from(text: Text) -> Self {
        (&text).into()
    }
}

impl From<&Text> for String {
    fn from(text: &Text) -> Self {
        serde_json::Value::from(text).to_string()
    }
}

impl From<Text> for String {
    fn from(text: Text) -> Self {
        serde_json::Value::from(text).to_string()
    }
}

impl From<TextComponent> for serde_json::Value {
    fn from(text: TextComponent) -> Self {
        Text::from(text).into()
    }
}

impl From<TextComponent> for String {
    fn from(text: TextComponent) -> Self {
        serde_json::Value::from(text).to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::text::{Color, Entity, Style, Text, TextComponent, Translate};
    use serde_json::json;
    use std::error::Error;

    #[test]
    pub fn text_text_single() -> Result<(), Box<dyn Error>> {
        let text_orignal: Text = Text::from("hello").into();

        let text_json = serde_json::Value::from(&text_orignal);

        assert_eq!(&text_json, &json!("hello"));

        let text: Text = Text::from_json(&text_json.to_string().as_str())?;
        assert_eq!(text_orignal, text);

        Ok(())
    }

    #[test]
    fn text_text_array() -> Result<(), Box<dyn Error>> {
        let text_orignal = Text::from("hello") + Text::from(" ") + Text::from("world!");

        let text_json = serde_json::Value::from(&text_orignal);

        assert_eq!(&text_json, &json!(["hello", " ", "world!"]));

        let text: Text = Text::from_json(&text_json.to_string().as_str())?;
        assert_eq!(text_orignal, text);

        Ok(())
    }

    #[test]
    fn text_text_color() -> Result<(), Box<dyn Error>> {
        let text_orignal = "hello world" * Color::DarkRed;
        let text_orignal = Text::from(text_orignal);

        let text_json = serde_json::Value::from(&text_orignal);

        assert_eq!(
            &text_json,
            &json!({
                "text": "hello world",
                "color": "dark_red",
            })
        );

        let text = Text::from_json(text_json.to_string().as_str())?;
        assert_eq!(text_orignal, text);
        Ok(())
    }

    #[test]
    fn text_hello_space_world() {
        let hello = "hello" * Color::Red * Style::Italic * Style::Bold;
        let space = " ";
        let world = "world" * Color::Blue * Style::Bold;
        let hello_space_world = hello + space + world;

        let text_json = serde_json::Value::from(&hello_space_world);

        assert_eq!(
            &text_json,
            &json!([
                "",
                {
                    "text": "hello",
                    "color": "red",
                    "bold": true,
                    "italic": true,
                },
                " ",
                {
                    "text": "world",
                    "color": "blue",
                    "bold": true,
                }
            ])
        );
    }

    #[test]
    fn text_translate() {
        let join =
            Translate::from("multiplayer.player.joined") * vec!["The_Defman"] * Color::Yellow;

        let text_json = serde_json::Value::from(join);

        assert_eq!(
            &text_json,
            &json!({
                "translate": "multiplayer.player.joined",
                "with": ["The_Defman"],
                "color": "yellow"
            })
        );

        let join = Translate::MultiplayerPlayerJoined * vec!["The_Defman"] * Color::Yellow;

        let text_json = serde_json::Value::from(join);

        assert_eq!(
            &text_json,
            &json!({
                "translate": "multiplayer.player.joined",
                "with": ["The_Defman"],
                "color": "yellow"
            })
        );
    }

    #[test]
    fn text_entity() {
        let entity = TextComponent::from("hello").on_hover_show_entity(Entity::new(
            uuid::Uuid::nil(),
            "player",
            "The_Defman",
        ));
        let json = serde_json::Value::from(entity);
        assert_eq!(
            &json,
            &json!({
                "text": "hello",
                "hover": {
                    "action": "show_entity",
                    "value": {
                        "id": "00000000-0000-0000-0000-000000000000",
                        "type": "player",
                        "name": "The_Defman"
                    }
                }
            })
        );
    }
}
