mod terminal;
mod view;
use crossterm::event::{Event::{self, Key}, KeyCode::{self, Char}, KeyEvent, KeyEventKind, KeyModifiers, read};
use terminal::{Terminal,Position,Size};
use core::cmp::min;
use view::View;


#[derive(Copy,Clone,Default)]
struct Location{
x: usize,
y: usize,
}


#[derive(Default)]
pub struct Editor{
should_quit: bool,
location: Location,
view:View,
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
let Size{ height,  width} = Terminal::size()?;
 match key_code {
            KeyCode::Up => {
                y = y.saturating_sub(1);
            }
            KeyCode::Down => {
                y = min(height.saturating_sub(1), y.saturating_add(1));
            }
            KeyCode::Left => {
                x = x.saturating_sub(1);
            }
            KeyCode::Right => {
                x = min(width.saturating_sub(1), x.saturating_add(1));
            }
            KeyCode::PageUp => {
                y = 0;
            }
            KeyCode::PageDown => {
                y = height.saturating_sub(1);
            }
            KeyCode::Home => {
                x = 0;
            }
            KeyCode::End => {
                x = width.saturating_sub(1);
            }
            _ => (),
        }
        self.location = Location{x,y};
        Ok(())
}

fn evaluate_event(&mut self,event:&Event) -> Result<(),std::io::Error>
{
if let Key(KeyEvent {
    code,
    modifiers,
     kind:KeyEventKind::Press,
    ..
}) = event{
match code 
    {
    KeyCode::Char('q') if *modifiers == KeyModifiers::CONTROL =>{
    self.should_quit = true;
    }
    KeyCode::Up
    | KeyCode::Down
    | KeyCode::Right
    | KeyCode::Left
    | KeyCode::PageDown
    | KeyCode::PageUp
    | KeyCode::Home
    | KeyCode::End =>{
    self.move_point(*code)?;
    }
    _=>(),
    }
}
Ok(())
}

fn refresh_screen(&self) -> Result<(),std::io::Error>
{
Terminal::hide_caret()?;
Terminal::move_caret_to(Position::default())?;
if self.should_quit{
Terminal::clear_screen()?;
//println!("Thanks For Using.\r\n");
Terminal::print("Thanks For Using>\r\n")?;
}else {
self.view.render()?;
Terminal::move_caret_to(Position{
col: self.location.x,
row: self.location.y,
})?;
}
Terminal::show_caret()?;
Terminal::execute()?;
Ok(())
}





// fn move_point(&mut self), key_code: KeyCode -> Result<(),std::io::Error> {
// let location {mut x, mut y} = self.location;
// let Size {}
// }

}
