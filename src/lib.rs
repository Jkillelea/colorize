// Crate Colorize
// file src/lib.rs
// This file is Beerware. Copyright 2018 Jacob Killelea <jkillelea@protonmail.ch>

use std::fmt;

// possible foreground options in my bash session
#[derive(Clone, Copy, Debug)]
pub enum ForegroundCode {
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
    Reset      = 0,
    Bold       = 1,
    Pale       = 2,
    Italic     = 3,
    Underline  = 4,
    Invert     = 7,
    Invisible  = 8,
    StrikeThru = 9,

}

// basic trait
pub trait Colorize {
    // taking these enums as references might be a performance hit
    // however, this library is mostly about ergonomics anyways
    fn foreground(&self, colorcode: &ForegroundCode) -> String;
    fn background(&self, colorcode: &BackgroundCode) -> String;
    fn style(&self, colorcode: &StyleCode)           -> String;
    fn reset(&self) -> String;
}

impl Colorize for String {
    fn foreground(&self, colorcode: &ForegroundCode) -> String {
        color_string(self, *colorcode as i32)
    }

    fn background(&self, colorcode: &BackgroundCode) -> String {
        color_string(&self, *colorcode as i32)
    }

    fn style(&self, stylecode: &StyleCode) -> String {
        color_string(&self, *stylecode as i32)
    }

    fn reset(&self) -> String {
        reset_string(&self)
    }
}

impl Colorize for str {
    fn foreground(&self, colorcode: &ForegroundCode) -> String {
        color_string(&self, *colorcode as i32)
    }

    fn background(&self, colorcode: &BackgroundCode) -> String {
        color_string(&self, *colorcode as i32)
    }

    fn style(&self, stylecode: &StyleCode) -> String {
        color_string(&self, *stylecode as i32)
    }

    fn reset(&self) -> String {
        reset_string(&self)
    }
}

impl<'a> Colorize for Box<fmt::Display> {
    fn foreground(&self, colorcode: &ForegroundCode) -> String {
        let mut buf = String::new();
        color_buffer(&mut buf, &self, *colorcode as i32);
        buf
    }

    fn background(&self, colorcode: &BackgroundCode) -> String {
        let mut buf = String::new();
        color_buffer(&mut buf, &self, *colorcode as i32);
        buf
    }

    fn style(&self, stylecode: &StyleCode) -> String {
        let mut buf = String::new();
        color_buffer(&mut buf, &self, *stylecode as i32);
        buf
    }

    fn reset(&self) -> String {
        let mut buf = String::new();
        reset_buffer(&mut buf, &self);
        buf
    }
}

// helper methods
fn color_string(string: &fmt::Display, colorcode: i32) -> String {
    let mut buffer = String::with_capacity(128); // pre reserve for small strings
    color_buffer(&mut buffer, &string, colorcode);
    buffer
}

fn reset_string(string: &fmt::Display) -> String {
    let mut buffer = String::with_capacity(128); // pre reserve for small strings
    reset_buffer(&mut buffer, &string);
    buffer
}

fn color_buffer(buffer: &mut fmt::Write, string: &fmt::Display, colorcode: i32) {
    write!(buffer, "\x1B[1;{}m{}", colorcode, string).unwrap();
}

fn reset_buffer(buffer: &mut fmt::Write, string: &fmt::Display) {
    write!(buffer, "{}\x1B[1;0m", string).unwrap();
}
