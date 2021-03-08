use serde::{Deserialize, Serialize};
use std::{convert::TryFrom, borrow::Cow};
use super::Text;

macro_rules! colors {
    {$($ident:ident => $name:ident => $rgb:expr),* $(,)?} => {
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        #[serde(try_from = "String", into = "Cow<'static, str>")]
        pub enum Color {
            $($ident,)*
            Rgb(u8, u8, u8)
        }

        impl Color {
            pub fn to_rgb(&self) -> Option<(u8, u8, u8)> {
                match self {
                    $(Color::$ident => $rgb,)*
                    Color::Rgb(r, g, b) => Some((*r, *g, *b))
                }
            }
        }

        impl TryFrom<String> for Color {
            type Error = &'static str;

            fn try_from(name: String) -> Result<Self, Self::Error> {
                match name.as_ref() {
                    $(stringify!($name) => Ok(Color::$ident),)*
                    "#000000" => Ok(Color::Rgb(0x00, 0x00, 0x00)),
                    _ => Err("not a valid color"),
                }
            }
        }

        impl From<(u8, u8, u8)> for Color {
            fn from((r, g, b): (u8, u8, u8)) -> Self {
                Color::Rgb(r, g, b)
            }
        }

        impl Into<Cow<'static, str>> for Color {
            fn into(self) -> Cow<'static, str> {
                match self {
                    $(Color::$ident => Cow::from(stringify!($name)),)*
                    Color::Rgb(r, g, b) => Cow::from(format!("#{:02X}{:02X}{:02X}", r, g, b)),
                }
            }
        }

        impl<'a> Text<'a> {
            $(
            pub fn $name(self) -> Self {
                self.color(Color::$ident)
            }
            )*
        }
    };
}

colors! {
    Black       => black => Some((0x00, 0x00, 0x00)),
    DarkBlue    => dark_blue => Some((0x00, 0x00, 0xAA)),
    DarkGreen   => dark_green => Some((0x00, 0xAA, 0x00)),
    DarkAqua    => dark_aqua => Some((0x00, 0xAA, 0xAA)),
    DarkRed     => dark_red => Some((0xAA, 0x00, 0x00)),
    DarkPurple  => dark_purple => Some((0xAA, 0x00, 0xAA)),
    Gold        => gold => Some((0xFF, 0xAA, 0x00)),
    Gray        => gray => Some((0xAA, 0xAA, 0xAA)),
    DarkGray    => dark_gray => Some((0x55, 0x55, 0x55)),
    Blue        => blue => Some((0x55, 0x55, 0xFF)),
    Green       => green => Some((0x55, 0xFF, 0x55)),
    Aqua        => aqua => Some((0x55, 0xFF, 0xFF)),
    Red         => red => Some((0xFF, 0x55, 0x55)),
    LightPurple => light_purple => Some((0xFF, 0x55, 0xFF)),
    Yellow      => yellow => Some((0xFF, 0xFF, 0x55)),
    White       => white => Some((0xFF, 0xFF, 0xFF)),
    Reset       => reset => None,
}