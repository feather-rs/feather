use crate::Text;
use serde::{Deserialize, Serialize};

// Based on https://wiki.vg/index.php?title=Protocol&oldid=16459#Title
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Title {
    pub title: Option<Text>,
    pub sub_title: Option<Text>,
    pub fade_in: u32,
    pub stay: u32,
    pub fade_out: u32,
}

impl Title {
    pub const HIDE: Title = Title {
        title: None,
        sub_title: None,
        fade_in: 0,
        stay: 0,
        fade_out: 0,
    };

    pub const RESET: Title = Title {
        title: None,
        sub_title: None,
        fade_in: 0,
        stay: 0,
        fade_out: 0,
    };
}
