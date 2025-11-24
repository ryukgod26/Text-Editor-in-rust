mod terminal;
mod view;
mod statusbar;
mod documentstatus;
mod fileinfo;
mod messagebar;
mod uicomponent;

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
use uicomponenet::UIComponent;
use self::{
    command::{
        Command::{Command,Edit,Move,System},
        System::{Quit,Resize,Save},
    },
    messagebar::MessageBar,terminal::Size};

pub const NAME: &str = env!("CARGO_PKG_NAME");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
const QUIT_TIMES: u8 = 3;

#[derive(Default)]
pub struct Editor{
should_quit: bool,
view:View,
status_bar: StatusBar,
title: String,
mesaage_bar: MessageBar,
terminal_size: Size,
quit_times: u8,
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
    let mut editor = Self::default();
    let size = Terminal::size().unwrap_or_default();
    editor.resize(size);
    editor.message_bar.update_message("HELP: Ctrl+S = Save | Ctrl+Q = Quit");
    let args: Vec<String> = env::args().collect();
    if let Some(filename) = args.get(1){
    editor.view.load(filename).is_err(){
        editor.message_bar.update_message(&format!("Error: Could not open file {filename}"))
    }
    }
    editor.refresh_status();
    Ok(editor)
    }

    pub fn refresh_status(&mut self){
    let status = self.view.get_status();
    let title = format!("{} - {NAME}",status.filename);
    self.status_bar.update_status(status);

    if title != self.title && matches!(Terminal::set_title(&title), Ok(())) {
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


fn evaluate_event(&mut self,event:Event)
{

    let should_process = match &event {
            Event::Key(KeyEvent { kind, .. }) => kind == &KeyEventKind::Press,
            Event::Resize(_, _) => true,
            _ => false,
        };
    if should_process {
    if let Ok(command) = EditorCommand::try_from(event) {
            self.process_command(command);
    }           
}

fn process_command(&mut self,command: Command){
    match command{
        System(Quit) => self.handle_quit(),
        Systen(Resize(size)) => self.resize(size),
        _ => self.reset_quit_times(),
    }

    match command{
        System(Quit | Resize(_)) => {},
        System(Save) => self.handle_save(),
        Edit(edit_command) => self.view.handle_edit_command(edit_command),
        Move(move_command) => self.view.handle_move_command(move_command),
    }
}

fn handle_save(&mut self){
    if self.view.save().is_ok(){
        self.message_bar.update_message("File saved successfully.");
    }else{
        self.message_bar.update_message("Error writing file!");
    }

}

#[allow(clippy::arithmetic_side_effects))]
fn handle_quit(&mut self){ 
    if !self.view.get_status().is_modified || self.quit_times + 1 == QUIT_TIMES{
        self.should_quit = true;
    }else if self.view.get_status().is_modified{
        self.message_bar.update_message(&format!("Warning!!! File has Some Unsaved Changes.Please Press Ctrl-Q {} more times to confirm quit",QUIT_TIMES - self.quit_times - 1 ));
        self.quit_times += 1;
    }
}

fn reset_quit_times(&mut self){
    if self.quit_times > 0{
        self.quit_times = 0;
        self.message_bar.update_message("");
    }
}

fn refresh_screen(&mut self)
{
if self.terminal_size.width == 0 || self.terminal_size.height == 0{
    return;
}

let _ = Terminal::hide_caret();
self.message_bar.render(self.terminal_size.height.saturating_sub(1));
if self.terminal_size.height > 1{
    self.status_bar.render(self.terminal_size.height.saturating_sub(2));
}
if self.terminal_size.height > 2{
    self.view.render(0);
}
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

fn resize(&mut self, size: Size){
    self.terminal_size = size;
    self.view.resize(Size{
        width: size.width,
        height: size.height.saturating_sub(2),
    });

    self.message_bar.resize(Size{
        height: 1,
        width: size.width,
    });

    self.status_bar.resize(Size{
        height: 1,
        width: size.width,
    });
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
