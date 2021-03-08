use super::Text;
use std::convert::TryFrom;

macro_rules! styles {
    {$($ident:ident => $name:ident),* $(,)?} => {
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum Style {
            $($ident,)*
        }

        impl TryFrom<&str> for Style {
            type Error = ();

            fn try_from(name: &str) -> Result<Self, Self::Error> {
                match name {
                    $(stringify!($name) => Ok(Style::$ident),)*
                    _ => Err(()),
                }
            }
        }

        impl Into<&'static str> for Style {
            fn into(self) -> &'static str {
                match self {
                    $(Style::$ident => stringify!($name),)*
                }
            }
        }

        impl<'a> Text<'a> {
            $(pub fn $name(self) -> Self {
                self.style(Style::$ident)
            })*
        }
    };
}

styles! {
    Obfuscated      => obfuscated,
    Bold            => bold,
    Strikethrough   => strikethrough,
    Underlined      => underlined,
    Italic          => italic,
}

macro_rules! not_style {
    {$($ident:ident => $name:ident),* $(,)?} => {
        impl<'a> Text<'a> {
            $(pub fn $name(self) -> Self {
                self.not_style(Style::$ident)
            })*
        }
    };
}

not_style! {
    Obfuscated      => not_obfuscated,
    Bold            => not_bold,
    Strikethrough   => not_strikethrough,
    Underlined      => not_underlined,
    Italic          => not_italic,
}

macro_rules! clear_style {
    {$($ident:ident => $name:ident),* $(,)?} => {
        impl<'a> Text<'a> {
            $(pub fn $name(self) -> Self {
                self.clear_style(Style::$ident)
            })*
        }
    };
}

clear_style! {
    Obfuscated      => clear_obfuscated,
    Bold            => clear_bold,
    Strikethrough   => clear_strikethrough,
    Underlined      => clear_underlined,
    Italic          => clear_italic,
}
