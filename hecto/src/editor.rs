mod terminal;
use crossterm::event::{Event::{self, Key}, KeyCode::{self, Char}, KeyEvent, KeyEventKind, KeyModifiers, read};
use terminal::{Terminal,Position,Size};
use crossterm::cursor;
use std::{fmt::Error, io::{Write, stdout}};
use core::cmp::min;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Copy,Clone,Default)]
struct Location{
x: usize,
y: usize,
}

#[derive(Default)]
pub struct Editor{
should_quit: bool,
location: Location,
}

impl Editor {
    /*
    pub const fn default() -> Self {
        Self { should_quit: false }
    }*/
    pub fn run(&mut self) {

        Terminal::intialize().unwrap();

        let result = self.repl();
        Terminal::terminate().unwrap();
          result.unwrap();
        /*
        if let Err(err) = self.repl() {
            panic!("{err:#?}");
        }
        print!("Thanks For Using.\r\n");*/
    }
    

    fn repl(&mut self) -> Result<(), std::io::Error> {

        loop {
            /*
            if let Key(KeyEvent {
                code, modifiers, kind, state
            }) = read()?
            {
                println!("Code: {code:?} Modifiers: {modifiers:?} Kind: {kind:?} State: {state:?} \r");
                match code {
                    Char('q') if modifiers == KeyModifiers::CONTROL => {
                        self.should_quit = true;
                    }
                    _ => (),
                }
            }
    
     let event = read()?;
     self.evaluate_event(&event);
     */
     self.refresh_screen()?;


            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evaluate_event(&event)?;
        }
        
        Ok(())
    }
//impl Editor
 
/*
enable_raw_mode.unwrap();
loop{
match read(){

Ok(Key(event)){
println!("{:?} \r",event);
match (event.code){

 Char(c)=>{
if c == 'q'{
disable_raw_mode.unwrap();
break;
},
_=>(),

    },

Err(err)=>{
println!("Error: {}",err);
},
_=>(),

 
                }
            }
    
        }
    }
}


}
*/

fn move_point(&mut self, key_code: KeyCode) ->Result<(),std::io::Error>
{
let Location { mut x, mut y} = self.location;
let Size{ height,  width} = Terminal::size();
match key_code {
    KeyCode::Up =>{
        y = y.saturating_sub(1);
    },
    KeyCode::Down =>{
        y = (height.saturating_sub(1),y.saturating_add(1))
    },
    KeyCode::Right =>{
        x = x.saturating_add(1);
        
    },
    KeyCode::Left =>{
        x = min(width.saturating_sub(1))
    }
}
Ok(())
}

fn evaluate_event(&mut self,event:&Event) -> Result<(),std::io::Error>
{
if let Key(KeyEvent {
    code,
    modifiers,
    ..
}) = event{
match code 
    {
    Char('q') if *modifiers == KeyModifiers::CONTROL =>{
    self.should_quit = true;
    },
    _=>(),
    }
}

}

fn refresh_screen(&self) -> Result<(),std::io::Error>
{
Terminal::hide_cursor()?;
if self.should_quit{
Terminal::clear_screen()?;
//println!("Thanks For Using.\r\n");
Terminal::print("Thanks For Using>\r\n")?;
}else {
Self::draw_rows()?;
Terminal::move_cursor_to(Position{x:0, y:0})?;
}
Terminal::show_cursor()?;
Terminal::execute()?;
Ok(())
}

fn draw_rows() -> Result<(),std::io::Error>
{

let Size{rows,..} = Terminal::size()?;

for row in 0..rows{
    Terminal::clear_current_line()?;
//    print!("~");
//    Terminal::print("~")?:
    #[allow(clippy::integer_divison)]
    if row == height/3{
    Self::welcome_message()?;
    }
    else{
    Self::draw_empty_row()?;
//    print!("\r\n");}
//    Terminal::print("\r\n");
    }
    if row.saturating_add(1) < height{
    Terminal::print("\r\n");
    }
    Ok(())
}
}

fn welcome_mesaage() -> Result<(),std::io::Error>{
let mut msg = format!("{NAME} Editor -- version {VERSION}");
let height = Terminal::size()?;
let len = msg.len();
//let padding = (width - len )/2;
//let spaces = " ".repeat(padding-1);
#[allow(clippy::integer_divison)]
let padding = (width.saturating_sub(len)) / 2;
let spaces = " ".repeat(padding.saturating_sub(1));
msg = format!("~{spaces}{msg}");
msg.truncate(width);
Terminal::print(msg)?;
Ok(())
}

fn draw_empty_row() -> Result<(),std::io::Error>{
Terminal::print("~")?;
Ok(())
}

// fn move_point(&mut self), key_code: KeyCode -> Result<(),std::io::Error> {
// let location {mut x, mut y} = self.location;
// let Size {}
// }

}
