macro_rules! c_enum {
    (
        $(#[$outer:meta])*
        pub enum $ident:ident {
            $($variant:ident = $x:literal),* $(,)?
        }
    ) => {
        $(#[$outer])*
        #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, ::serde::Serialize, ::serde::Deserialize)]
        #[repr(u32)]
        pub enum $ident {
            $(
                $variant = $x,
            )*
        }

        impl $ident {
            pub fn from_u32(x: u32) -> Option<Self> {
                match x {
                    $(
                        $x => Some(Self::$variant),
                    )*
                    _ => None,
                }
            }
        }
    }
}
