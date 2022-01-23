use std::cmp::Ordering;

use serde::{Serialize, Deserialize};

use crate::Effect;

/// Storing effects info.
#[derive(Copy, Clone, Debug, Hash, Serialize, PartialEq, Eq, Deserialize)]
pub struct EffectApplication {
    /// Effect kind
    pub kind: Effect,
    /// Effect intensity, up to 255.
    pub amplifier: u8,
    /// Duration of the effect in ticks.
    pub duration: u32,
    /// Effect flags
    pub flags: EffectFlags,

    /// Store when effect was added, if start_tick == 0 effect not yet sent to client
    pub start_tick: u64,
}
/// Flags that define how an effect is presented.
#[derive(Copy, Clone, Debug, Hash, Serialize, PartialEq, Eq, Deserialize)]
pub struct EffectFlags {
    /// `true` if there should be visible particles.
    pub particle: bool,
    /// `true` when caused by a beacon.
    pub ambient: bool,
    /// `true` if an icon should be shown to the player.
    pub icon: bool,
}
impl Ord for EffectApplication {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.amplifier > other.amplifier || self.duration > other.duration {
            Ordering::Greater
        } else if self.amplifier == other.amplifier || self.duration == other.duration {
            Ordering::Equal
        } else {
            Ordering::Less
        }
    }
}
impl PartialOrd for EffectApplication {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}