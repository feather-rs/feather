use crate::text::{Keybind, Text, Translate};
use std::borrow::Cow;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(untagged)]
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

impl TextValue {
    pub fn text<T: Into<Cow<'static, str>>>(text: T) -> Self {
        TextValue::Text { text: text.into() }
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

impl<T> From<T> for TextValue
where
    T: Into<Cow<'static, str>>,
{
    fn from(value: T) -> Self {
        Self::text(value)
    }
}
