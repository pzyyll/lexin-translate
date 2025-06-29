#![allow(unused)]

use enigo::{
    Direction::{Click, Press, Release},
    Enigo, InputResult, Key, Keyboard, Settings,
};
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref SIMULATOR: Mutex<Enigo> =
        Mutex::new(Enigo::new(&Settings::default()).unwrap());
}

pub fn text(text: &str) -> InputResult<()> {
    SIMULATOR.lock().unwrap().text(text)
}

pub fn trigger_copy() -> InputResult<()> {
    SIMULATOR.lock().unwrap().key(Key::Control, Press)?;
    SIMULATOR.lock().unwrap().key(Key::Unicode('c'), Click)?;
    SIMULATOR.lock().unwrap().key(Key::Control, Release)
}
