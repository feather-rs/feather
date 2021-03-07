use crate::text::Text;
use serde::{Serialize, Deserialize};

// Based on https://wiki.vg/index.php?title=Protocol&oldid=16459#Title
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Title {
    pub title: Option<Text>,
    pub sub_title: Option<Text>,
    pub fade_in: u32,
    pub stay: u32,
    pub fade_out: u32
}

impl Default for Title {

    fn default () -> Self {
        Title {
            title: None,
            sub_title: None,
            fade_in: 20,
            stay: 20,
            fade_out: 20
        }
    }
}