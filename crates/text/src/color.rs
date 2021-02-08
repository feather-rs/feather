use serde::{Deserialize, Serialize};
use std::{convert::TryFrom, borrow::Cow};
use super::Text;

macro_rules! colors {
    {$($ident:ident => $name:ident),* $(,)?} => {
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        #[serde(try_from = "&str", into = "Cow<'static, str>")]
        pub enum Color {
            $($ident,)*
            Rgb(u8, u8, u8)
        }

        impl TryFrom<&str> for Color {
            type Error = &'static str;

            fn try_from(name: &str) -> Result<Self, Self::Error> {
                match name {
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
    Black       => black,
    DarkBlue    => dark_blue,
    DarkGreen   => dark_green,
    DarkAqua    => dark_aqua,
    DarkRed     => dark_red,
    DarkPurple  => dark_purple,
    Gold        => gold,
    Gray        => gray,
    DarkGray    => dark_gray,
    Blue        => blue,
    Green       => green,
    Aqua        => aqua,
    Red         => red,
    LightPurple => light_purple,
    Yellow      => yellow,
    White       => white,
    Reset       => reset,
}