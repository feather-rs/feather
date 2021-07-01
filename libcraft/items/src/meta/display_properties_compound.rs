use serde::{Deserialize, Serialize};

/// Contains data related to `ItemStack` display.
/// * Display info: title, lore (list) and hex armor color codes.
/// * Bit flags to indicate which parts of the tooltip should be hidden.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DisplayPropertiesCompound {
    /// Display info compound containing title, lore and
    /// hex armor colors.
    display: DisplayInfoCompound,

    /// Bit flags that indicate which tooltips should be hidden.
    #[serde(rename = "HideFlags")]
    hide_flags: Option<u8>,
}

bitflags! {
    struct HideToolTipFlags: u8 {
        const ENCHANTMENTS        = 0b00000001;
        const ATTRIBUTE_MODIFIERS = 0b00000010;
        const UNBREAKABLE         = 0b00000100;
        const CAN_DESTROY         = 0b00001000;
        const CAN_PLACE_ON        = 0b00010000;
        const OTHER               = 0b00100000;
        const DYE                 = 0b01000000;
    }
}

/// Contains info data to be displayed about the `ItemStack`:
/// * The title of the `ItemStack`.
/// * The lore of the `ItemStack`.
/// * The hex color code of the armor to be displayed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DisplayInfoCompound {
    /// The title of the `ItemStack`.
    name: String,

    /// The lore of the `ItemStack`.
    lore: Vec<String>,

    /// The hex color of the armor.
    /// Red << 16 + Green << 8 + Blue
    #[serde(rename = "color")]
    color: Option<u32>,
}
