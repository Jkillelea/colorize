extern crate colorize;
use colorize::*;

// a simple example
fn main() {
    use StyleCode::*;
    for sty in [Normal, Bold, Pale, Italic, Underline, Invert, Invisible, StrikeThru].iter() {
        println!("{:?}", sty);

        use ForegroundCode::*;
        for fg in [White, Gray, Red, Green, Yellow, Blue, Purple, Teal].iter() {

            use BackgroundCode::*;
            for bg in [Regular, Black, Red, Green, Yellow, Blue, Purple, Teal, White].iter() {
                print!("{}", "potato".style(*sty)
                                     .foreground(*fg)
                                     .background(*bg));
            }
            println!("");
        }
    }
}
