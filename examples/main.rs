// Crate Colorize
// file examples/main.rs
// This file is Beerware. Copyright 2018 Jacob Killelea <jkillelea@protonmail.ch>

extern crate colorize;
use colorize::*;

// A simple example. Iterate through each style/color
fn main() {
    use StyleCode::*;
    for sty in [Normal, Bold, Pale, Italic, Underline, Invert, Invisible, StrikeThru].iter() {
        println!("{:?}", sty);

        use ForegroundCode::*;
        for fg in [White, Gray, Red, Green, Yellow, Blue, Purple, Teal].iter() {

            use BackgroundCode::*;
            for bg in [Regular, Black, Red, Green, Yellow, Blue, Purple, Teal, White].iter() {
                print!("{}", "colorize".style(*sty)
                                       .foreground(*fg)
                                       .background(*bg));
            }
            println!("");
        }
    }
}
