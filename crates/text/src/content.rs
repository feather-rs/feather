use super::{Text, ExtraVec};
use serde::{Serialize, Deserialize};
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
pub enum Content<'a> {
    /// A string containing plain text to display directly. Can also be a number or boolean that is displayed directly.
    Text { text: Cow<'a, str> },
    //// Displays a translated piece of text from the currently selected language. 
    /// This uses the client's selected language, so if players with their games set to different languages are logged into the same server, each will see the component in their own language.
    /// Translations are defined in language files in resource packs, including the built-in resource pack.
    /// Translations can contain slots for text that is not known ahead of time, such as player names. When displaying the translated text, slots will be filled from a provided list of text components. 
    /// The slots are defined in the language file, and generally take the form %s (displays the next component in the list), or %3$s (displays the third component in the list; replace 3 with whichever index is desired).
    /// For example, the built-in English language file contains the translation "chat.type.advancement.task": "%s has made the advancement %s",. 
    Translation {
        translate: Cow<'a, str>,
        #[serde(with = "ExtraVec")]
        with: Vec<Text<'a>>,
    },
    /// Displays the name of the button that is currently bound to a certain configurable control. 
    /// This uses the client's own control scheme, so if players with different control schemes are logged into the same server, each will see their own keybind. 
    Keybind {
        keybind: Cow<'a, str>
    },
}

impl<'a> Default for Content<'a> {
    fn default() -> Self {
        Content::text("")
    }
}

impl<'a> Content<'a> {
    pub fn text<T: Into<Cow<'a, str>>>(text: T) -> Self {
        Content::Text { text: text.into() }
    }

    pub fn translation<T: Into<Cow<'a, str>>>(translate: T, with: Vec<Text<'a>>) -> Self {
        Content::Translation {
            translate: translate.into(),
            with,
        }
    }

}

macro_rules! is_content {
    {$($ident:ident => $is_ident:ident),*$(,)?} => {
        impl<'a> Content<'a> {
            $(
            pub fn $is_ident(&self) -> bool {
                match self {
                    Content::$ident { .. } => true,
                    _ => false,
                }
            }
            )*
        }
    };
}

is_content! {
    Text        => is_text,
    Translation => is_translation,
    Keybind     => is_keybind,
}
