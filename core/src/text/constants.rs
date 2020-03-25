use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::borrow::Cow;

macro_rules! constants {
    ($typ:ident, $(($name:ident, $value:literal)),*) => {
        #[derive(Debug, PartialEq)]
        pub enum $typ {
            $(
                $name,
            )*
            Custom(Cow<'static, str>),
        }

        impl<T> From<T> for $typ
        where
            T: Into<Cow<'static, str>>
        {
            fn from(value: T) -> Self {
                let value = value.into();
                match value.as_ref() {
                    $(
                        $value => Self::$name,
                    )*
                    _ => Self::Custom(value)
                }
            }
        }

        impl<'a> From<&'a $typ> for &'a str {
            fn from(value: &'a $typ) -> Self {
                match value {
                    $(
                        $typ::$name => $value,
                    )*
                    $typ::Custom(ref value) => value,
                }
            }
        }

        impl Serialize for $typ {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                serializer.serialize_str(self.into())
            }
        }

        impl<'de> Deserialize<'de> for $typ {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                Ok(s.into())
            }
        }
    };
}

constants!(
    Color,
    (DarkRed, "dark_red"),
    (Red, "red"),
    (Gold, "gold"),
    (Yellow, "yellow"),
    (DarkGreen, "dark_green"),
    (Green, "green"),
    (Aqua, "aqua"),
    (DarkAqua, "dark_aqua"),
    (DarkBlue, "dark_blue"),
    (Blue, "blue"),
    (LightPurple, "light_purple"),
    (DarkPurple, "dark_purple"),
    (White, "white"),
    (Gray, "gray"),
    (DarkGray, "dark_gray"),
    (Black, "black")
);

constants!(
    Translate,
    (ChatTypeText, "chat.type.text"),
    (MultiplayerPlayerJoined, "multiplayer.player.joined")
);

constants!(
    Keybind,
    (Attack, "key_key.attack"),
    (UseItem, "key_key.use"),
    (Forward, "key_key.forward"),
    (Left, "key_key.left"),
    (Back, "key_key.back"),
    (Right, "key_key.right"),
    (Jump, "key_key.jump"),
    (Sneak, "key_key.sneak"),
    (Sprint, "key_key.sprint"),
    (Drop, "key_key.drop"),
    (Iventory, "key_key.inventory"),
    (Chat, "key_key.chat"),
    (ListPlayers, "key_key.playerlist"),
    (PickBlock, "key_key.pickItem"),
    (Command, "key_key.command"),
    (Screenshot, "key_key.screenshot"),
    (Perspective, "key_key.togglePerspective"),
    (MouseSmoothing, "key_key.smoothCamera"),
    (Fullscreen, "key_key.fullscreen"),
    (SpectatorOutlines, "key_key.spectatorOutlines"),
    (SwapHands, "key_key.swapHands"),
    (SaveToolbar, "key_key.saveToolbarActivator"),
    (LoadToolbar, "key_key.loadToolbarActivator"),
    (Advancements, "key_key.advancements"),
    (Hotbar1, "key_key.hotbar.1"),
    (Hotbar2, "key_key.hotbar.2"),
    (Hotbar3, "key_key.hotbar.3"),
    (Hotbar4, "key_key.hotbar.4"),
    (Hotbar5, "key_key.hotbar.5"),
    (Hotbar6, "key_key.hotbar.6"),
    (Hotbar7, "key_key.hotbar.7"),
    (Hotbar8, "key_key.hotbar.8"),
    (Hotbar9, "key_key.hotbar.9")
);
