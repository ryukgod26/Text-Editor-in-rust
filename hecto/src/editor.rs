mod terminal;
mod view;
mod statusbar;
mod documentstatus;
mod fileinfo;

use crossterm::event::{Event, KeyEvent, KeyEventKind, read};
use terminal::Terminal;
mod editorcommand;
use view::View;
use std::
{
    env,panic::{set_hook,take_hook}
};
use editorcommand::EditorCommand;
use statusbar::StatusBar;
use documentstatus::DocumentStatus;

pub const NAME: &str = env!("CARGO_PKG_NAME");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Editor{
should_quit: bool,
view:View,
status_bar: StatusBar,
title: String,
}

impl Editor {
    /*
    pub const fn default() -> Self {
        Self { should_quit: false }
    }*/

    pub fn new() -> Result<Self,std::io::Error>{
    let current_hook = take_hook();
    set_hook(Box::new(move|panic_info|{
    let _ = Terminal::terminate();
    current_hook(panic_info);
    }));
    Terminal::intialize()?;
    let mut editor = Self{
        should_quit: false,
        view: View::new(2),
        status_bar: StatusBar::new(1),
        title: String::new(),
    };
    let args: Vec<String> = env::args().collect();
    if let Some(filename) = args.get(1){
    editor.view.load(filename);    
    }
    editor.refresh_status();
    Ok(editor)
    }

    pub fn refresh_status(&mut self){
    let status = self.view.get_status();
    let title = format!("{} - {NAME}",status.filename);
    self.status_bar.update_status(status);

    if title != self.title && matches!(Terminal::set_title(&title),Ok(())){
        self.title = title;
        }
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
     self.refresh_screen();


            if self.should_quit {
                break;
            }
            let event = read();
            match event{
            Ok(event)=>
            self.evaluate_event(event),
            Err(err)=>{
        #[cfg(debug_assertions)]
                {
                panic!("Could not read event {err:?}");
                }
            }
            }
            let status = self.view.get_status();
            self.status_bar.update_status(status);
        }
        
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



#[allow(clippy::needless_pass_by_value)]
fn evaluate_event(&mut self,event:Event)
{

    let should_process = match &event {
            Event::Key(KeyEvent { kind, .. }) => kind == &KeyEventKind::Press,
            Event::Resize(_, _) => true,
            _ => false,
        };
    if should_process {
    if let Ok(command) = EditorCommand::try_from(event) {
            if matches!(command, EditorCommand::Quit){
                self.should_quit = true;
            } else {
                self.view.handle_command(command);
                if let EditorCommand::Resize(size) = command{
                    self.status_bar.resize(size);
                }
            }
        }
    } 
}

fn refresh_screen(&mut self)
{
let _ = Terminal::hide_caret();
self.view.render();
self.status_bar.render();
/*
if self.should_quit{
Terminal::clear_screen()?;
//println!("Thanks For Using.\r\n");
Terminal::print("Thanks For Using>\r\n")?;
}else {}*/
let _ = Terminal::move_caret_to(self.view.caret_position());
let _ = Terminal::show_caret();
let _ = Terminal::execute();
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
