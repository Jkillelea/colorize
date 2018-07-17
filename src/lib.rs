// Crate Colorize
// file src/lib.rs
// This file is Beerware. Copyright 2018 Jacob Killelea <jkillelea@protonmail.ch>

use std::fmt;

// possible foreground options in my bash session
#[derive(Clone, Copy, Debug)]
pub enum ForegroundCode {
    White  = 0,
    Gray   = 30,
    Red    = 31,
    Green  = 32,
    Yellow = 33,
    Blue   = 34,
    Purple = 35,
    Teal   = 36,
}

// possible background options in my bash session
#[derive(Clone, Copy, Debug)]
pub enum BackgroundCode {
    Regular = 39,
    Black   = 40,
    Red     = 41,
    Green   = 42,
    Yellow  = 43,
    Blue    = 44,
    Purple  = 45,
    Teal    = 46,
    White   = 47,
}

// possible style options in my bash session
#[derive(Clone, Copy, Debug)]
pub enum StyleCode {
    Normal     = 0,
    Bold       = 1,
    Pale       = 2,
    Italic     = 3,
    Underline  = 4,
    Invert     = 7,
    Invisible  = 8, // TODO: ??
    StrikeThru = 9,

}

// basic trait
pub trait Colorize {
    fn foreground(&self, colorcode: ForegroundCode) -> String;
    fn background(&self, colorcode: BackgroundCode) -> String;
    fn style(&self, colorcode: StyleCode) -> String;
}


impl Colorize for String {
    fn foreground(&self, colorcode: ForegroundCode) -> String {
        colorstring(&self, colorcode as i32)
    }

    fn background(&self, colorcode: BackgroundCode) -> String {
        colorstring(&self, colorcode as i32)
    }

    fn style(&self, stylecode: StyleCode) -> String {
        colorstring(&self, stylecode as i32)
    }
}

impl Colorize for str {
    fn foreground(&self, colorcode: ForegroundCode) -> String {
        colorstring(&self, colorcode as i32)
    }

    fn background(&self, colorcode: BackgroundCode) -> String {
        colorstring(&self, colorcode as i32)
    }

    fn style(&self, stylecode: StyleCode) -> String {
        colorstring(&self, stylecode as i32)
    }
}

// helper method
fn colorstring(string: &fmt::Display, colorcode: i32) -> String {
    format!("\x1B[1;{}m{}\x1B[0m", colorcode, string)
}

