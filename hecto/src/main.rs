#![warn(clippy::all,
clippy::pedantic,
clippy::print_stdout,
clippy::arithmetic_side_effects,
clippy::as_conversions,
clippy::integer_divison
)]

mod editor;
use editor::Editor;


fn main() {
    Editor::new().unwrap().run();
    /*
    enable_raw_mode().unwrap();
    for b in io::stdin().bytes(){
        // println!("{:?}",b);
        let b = b.unwrap();
        let c = b.unwrap() as char;
        if c.is_control() {
            println!("Binary {0:0b} ASCII {0:#03} \r",b);
        } else{
            println!("Binary {0:0b} ASCII {0:#03} Character {1:#?}\r");
         }
        if c == 'q'{
            disable_raw_mode().unwrap();
            break;
        }
    }*/
}
