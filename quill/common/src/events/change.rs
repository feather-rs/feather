/*
All events in this file are triggerd when there is a change in a certain value.
*/

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreativeFlyingEvent {
    pub is_flying: bool,
}

impl CreativeFlyingEvent {
    pub fn new(changed_to: bool) -> Self {
        Self {
            is_flying: changed_to,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SneakEvent {
    pub is_sneaking: bool,
}

impl SneakEvent {
    pub fn new(changed_to: bool) -> Self {
        Self {
            is_sneaking: changed_to,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SprintEvent {
    pub is_sprinting: bool,
}

impl SprintEvent {
    pub fn new(changed_to: bool) -> Self {
        Self {
            is_sprinting: changed_to,
        }
    }
}
