mod editor;
use editor::Editor;


fn main() {
    Editor::default().run();

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
