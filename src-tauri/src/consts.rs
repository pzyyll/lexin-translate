//! Copyright: 2024 Lizc. All rights reserved.
//! License: GNU GPL v3 or later
//! You may obtain a copy of the License at https://www.gnu.org/licenses/gpl-3.0.html
//!
//! Author: Lizc
//! Created Data: 2024-05-27
//!
//! Description: The consts module for the application.

#![allow(unused)]

pub const WIN_LABEL_HOME: &str = "home";
pub const WIN_LABEL_TRANSLATE: &str = "translate";
pub const APP_NAME: &str = "Lexin";
pub const APP_DATA_CONFIG: &str = "settings.json";
pub const SERVER_API_KEY: &str = "lexinsvr";
pub const DOUBLE_CLICK_INTERVAL: u32 = 350;

pub enum WindowEvent {
    CPCP,
}

impl WindowEvent {
    pub fn to_string(&self) -> &str {
        match self {
            WindowEvent::CPCP => "cpcp",
        }
    }
}

#[derive(Eq, Hash, PartialEq)]
pub enum Shortcut {
    DoubleCopy,
    DoubleAlt,
    AltShift1,
}

#[derive(Eq, Hash, PartialEq)]
pub enum MouseEvent {
    LeftDown,
    LeftUp,
    Move,
}
