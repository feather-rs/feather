use std::borrow::Cow;

macro_rules! keybinds {
    {$($ident:ident => $name:literal),* $(,)?} => {
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum Keybind<'a> {
            $($ident,)*
            Custom(Cow<'a, str>)
        }

        impl<'a> From<&'a str> for Keybind<'a> {

            fn from(bind: &'a str) -> Self {
                match bind {
                    $($name => Keybind::$ident,)*
                    _ => Keybind::Custom(Cow::<'a>::from(bind)),
                }
            }
        }

        impl<'a> Into<Cow<'a, str>> for Keybind<'a> {
            fn into(self) -> Cow<'a, str> {
                match self {
                    $(Keybind::$ident => Cow::from($name),)*
                    Keybind::Custom(bind) => bind,
                }
            }
        }
    };
}

keybinds! {
    Attack              => "key_key.attack",
    UseItem             => "key_key.use",
    Forward             => "key_key.forward",
    Left                => "key_key.left",
    Back                => "key_key.back",
    Right               => "key_key.right",
    Jump                => "key_key.jump",
    Sneak               => "key_key.sneak",
    Sprint              => "key_key.sprint",
    Drop                => "key_key.drop",
    Inventory           => "key_key.inventory",
    Chat                => "key_key.chat",
    ListPlayers         => "key_key.playerlist",
    PickBlock           => "key_key.pickItem",
    Command             => "key_key.command",
    Screenshot          => "key_key.screenshot",
    Perspective         => "key_key.togglePerspective",
    MouseSmoothing      => "key_key.smoothCamera",
    Fullscreen          => "key_key.fullscreen",
    SpectatorOutlines   => "key_key.spectatorOutlines",
    SwapHands           => "key_key.swapHands",
    SaveToolbar         => "key_key.saveToolbarActivator",
    LoadToolbar         => "key_key.loadToolbarActivator",
    Advancements        => "key_key.advancements",
    Hotbar1             => "key_key.hotbar.1",
    Hotbar2             => "key_key.hotbar.2",
    Hotbar3             => "key_key.hotbar.3",
    Hotbar4             => "key_key.hotbar.4",
    Hotbar5             => "key_key.hotbar.5",
    Hotbar6             => "key_key.hotbar.6",
    Hotbar7             => "key_key.hotbar.7",
    Hotbar8             => "key_key.hotbar.8",
    Hotbar9             => "key_key.hotbar.9",
}
