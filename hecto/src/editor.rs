mod terminal;
mod view;
use crossterm::event::{Event, KeyCode::{self, Char}, KeyEvent, KeyEventKind, KeyModifiers, read};
use terminal::{Terminal,Position,Size};
use core::cmp::min;
use view::View;
use std::env,panic::{set_hook,take_hook};

#[derive(Copy,Clone,Default)]
struct Location{
x: usize,
y: usize,
}


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

    pub fn new() -> Result<(),std::io::Error>{
    let current = take_hook();
    set_hook(Box::new(move|panic_info|{
    let _ = Terminal::terminate();
    current_hook(panic_info);
    }));
    Terminal::intialize()?;
    let mut view = View::default();
    let args: Vec<String> = env::args().collect();
    if let Some(filename) = args.get(1){
    view.load(filename)?;    
    }
    Ok(Self{
        should_quit: false,
        location: Location::default(),
        view
    })
    }


   pub fn run(&mut self){

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
            match event{
            Ok(event)=>
            self.evaluate_event(event)?;,
            Err(err)=>{
        #[cfg(debug_assertions)]
                {
                panic!("Could not read event {err:?}");
                }
            }
            }
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

fn move_point(&mut self, key_code: KeyCode)
{
let Location { mut x, mut y} = self.location;
let Size{ height,  width} = Terminal::size().unwrap_or_default();
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

#[allow(clippy::needless_pass_by_value)]
fn evaluate_event(&mut self,event:&Event)
{

    match event{
        Event::Key(KeyEvent{
    code,
    kind: KeyEventKind::Press,
    mofifiers,
    ..
        }) => match (code,modifiers) 
    {
        (KeyCode::Char('q'),KeyModifiers::CONTROL) =>{
    self.should_quit = true;
    }
        (
    KeyCode::Up
    | KeyCode::Down
    | KeyCode::Right
    | KeyCode::Left
    | KeyCode::PageDown
    | KeyCode::PageUp
    | KeyCode::Home
    | KeyCode::End, _,) =>{
    self.move_point(code);
    }
    _=>{}
    },
    Event::Resize(width_u16,height_u16) => {
    #[allow(clippy::as_conversions)]
    let height = height_u16 as usize;

    #[allow(clippy::as_conversions)]
    let width = width_u16 as usize;
    self.view.resize(Size{height,width});

    }
    _ => {}
}
Ok(())
}

fn refresh_screen(&self)
{
let _ = Terminal::hide_caret();
self.view.render();
/*
if self.should_quit{
Terminal::clear_screen()?;
//println!("Thanks For Using.\r\n");
Terminal::print("Thanks For Using>\r\n")?;
}else {}*/
let _ = Terminal::move_caret_to(Position{
col: self.location.x,
row: self.location.y,
});
let _ = Terminal::show_caret();
let _ = Terminal::execute();
Ok(())
}





// fn move_point(&mut self), key_code: KeyCode -> Result<(),std::io::Error> {
// let location {mut x, mut y} = self.location;
// let Size {}
// }

}


impl Drop for Editor{
fn drop(&mut self){
    let _ = Terminal::terminate();
    if self.should_quit{
    let _ = Terminal::print("Thanks For Using.\r\n");
    }
}
}
