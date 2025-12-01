pub mod annotationtype;
mod annotation;
mod terminal;
mod documentstatus;
mod uicomponents;
mod position;
mod size;
mod annotatedstring;
mod command;
mod line;

use crossterm::event::{Event, KeyEvent, KeyEventKind, read};
use terminal::Terminal;

use std::
{
    env,panic::{set_hook,take_hook}
};

use documentstatus::DocumentStatus;
use uicomponents::{
    UIComponent,CommandBar,MessageBar,View,StatusBar
};
use self::command::{
        Command::{self,Edit,Move,System},
        Edit::Enter,
        Move::{Left,Right,Up,Down},
        System::{Dismiss,Quit,Resize,Save,Find},
};
use annotatedstring::{AnnotatedString};
use position::{Position};
use size::Size;
pub use line::Line;
pub use annotationtype::AnnotationType;
use annotation::Annotation;

pub const NAME: &str = env!("CARGO_PKG_NAME");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
const QUIT_TIMES: u8 = 3;

#[derive(Default)]
pub struct Editor{
should_quit: bool,
view:View,
status_bar: StatusBar,
title: String,
message_bar: MessageBar,
terminal_size: Size,
quit_times: u8,
command_bar: CommandBar,
prompt_type: PromptType,
}

#[derive(Eq,PartialEq,Default)]
enum PromptType{
    Find,
    Save,
    #[default]
    None,
}

impl PromptType{
    fn is_none(&self) -> bool{
        *self == Self::None
    }
}

impl Editor {
    /*
    pub const fn default() -> Self {
        Self { should_quit: false }
    }*/

     pub fn new() -> Result<Self, std::io::Error> {
        let current_hook = take_hook();
        set_hook(Box::new(move |panic_info| {
            let _ = Terminal::terminate();
            current_hook(panic_info);
        }));
        Terminal::initialize()?;

        let mut editor = Self::default();
        let size = Terminal::size().unwrap_or_default();
        editor.handle_resize_command(size);
        editor
            .update_message("HELP: Ctrl-S = save and Ctrl-Q = quit and Ctrl-f = find");

        let args: Vec<String> = env::args().collect();
        if let Some(file_name) = args.get(1) {
            if editor.view.load(file_name).is_err() {
                editor
                    .update_message(&format!("Error: Could not open file: {file_name}"));
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
            #[cfg(not(debug_assertions))]
                {
                    let _ = err;
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
        if let Ok(command) = Command::try_from(event) {
            self.process_command(command);
        }           
    }
}

fn process_command(&mut self,command: Command){
    
    if let System(Resize(size)) = command {
        self.handle_resize_command(size);
        return;
    }

    match self.prompt_type{
        PromptType::Find => self.process_command_during_search(command),
        PromptType::Save => self.process_command_during_save(command),
        PromptType::None => self.process_command_no_prompt(command),
    }
}

fn process_command_no_prompt(&mut self, command: Command){
    if matches!(command, System(Quit)){
        self.handle_quit_command();
        return;
    }
    self.reset_quit_times();

    match command{
        System(Quit | Resize(_) | Dismiss) => {}
        System(Find) => self.set_prompt(PromptType::Find),
        System(Save) => self.handle_save_command(),
        Edit(edit_command) => self.view.handle_edit_command(edit_command),
        Move(move_command) => self.view.handle_move_command(move_command),
    }
}

// fn dismiss_prompt(&mut self){
//     self.command_bar = None;
//     self.message_bar.mark_redraw(true);
// }

// fn show_prompt(&mut self) {
//     let mut command_bar = CommandBar::default();
//     command_bar.set_prompt("Save as");
//     command_bar.resize(Size{
//         height: 1,
//         width: self.terminal_size.width,
//     });
//     command_bar.mark_redraw(true);
//     self.command_bar = Some(command_bar);
// }

fn handle_save_command(&mut self){
    if self.view.is_file_loaded(){
        self.save(None);
    }else{
        self.set_prompt(PromptType::Save);
    }

}

fn save(&mut self, filename: Option<&str>) {
    let result = if let Some(name) = filename{
        self.view.save_as(name)
    } else{
        self.view.save_file_to_disk()
    };
    
    if result.is_ok() {
        self.update_message("File Saved Successfully.");
    } else{
        self.update_message("Error Occured While Saving File.");
    }
}

fn process_command_during_save(&mut self, command: Command){
    match command{
        System(Quit | Find | Resize(_) | Save) | Move(_)=> {}
        System(Dismiss) => {
            self.set_prompt(PromptType::None);
            self.update_message("Save Aborted.");
        }
        Edit(Enter) => {
            let filename = self.command_bar.value();
            self.save(Some(&filename));
            self.set_prompt(PromptType::None);
        }
        Edit(edit_command) => self.command_bar.handle_edit_command(edit_command),
    }
}

fn process_command_during_search(&mut self, command:Command) {
    match command{
        System(Dismiss) => {
            self.set_prompt(PromptType::None);
            self.view.dismiss_search();
        }
        Edit(Enter) => {
            self.set_prompt(PromptType::None);
            self.view.exit_search();
        }
        Edit(edit_command) => {
            self.command_bar.handle_edit_command(edit_command);
            let query = self.command_bar.value();
            self.view.search(&query);
        }
    
        Move(Right | Down) => self.view.search_next(),
        Move(Left | Up ) => self.view.search_prev(),
        System(Find | Save | Resize(_) | Quit) | Move(_) => {}
    }
}

#[allow(clippy::arithmetic_side_effects)]
fn handle_quit_command(&mut self){ 
    if !self.view.get_status().is_modified || self.quit_times + 1 == QUIT_TIMES{
        self.should_quit = true;
    }else if self.view.get_status().is_modified{
        self.update_message(&format!("Warning!!! File has Some Unsaved Changes.Please Press Ctrl-Q {} more times to confirm quit",QUIT_TIMES - self.quit_times - 1 ));
        self.quit_times += 1;
    }
}

fn reset_quit_times(&mut self){
    if self.quit_times > 0{
        self.quit_times = 0;
        self.update_message("");
    }
}

fn refresh_screen(&mut self)
{
if self.terminal_size.width == 0 || self.terminal_size.height == 0{
    return;
}
let bottom_bar_row = self.terminal_size.height.saturating_sub(1);
let _ = Terminal::hide_caret();
//self.message_bar.render(self.terminal_size.height.saturating_sub(1));

if self.in_prompt(){
    self.command_bar.render(bottom_bar_row);
    
}else{
    self.message_bar.render(bottom_bar_row);
}

if self.terminal_size.height > 1{
    self.status_bar.render(self.terminal_size.height.saturating_sub(2));
}
if self.terminal_size.height > 2{
    self.view.render(0);
}

let new_caret_pos = if self.in_prompt(){
    Position{
        row: bottom_bar_row,
        col: self.command_bar.caret_position_col(),
    }
}else{
    self.view.caret_position()
};

debug_assert!(new_caret_pos.row <= self.terminal_size.height);
debug_assert!(new_caret_pos.col <= self.terminal_size.width);

/*
if self.should_quit{
Terminal::clear_screen()?;
//println!("Thanks For Using.\r\n");
Terminal::print("Thanks For Using>\r\n")?;
}else {}*/

let _ = Terminal::move_caret_to(new_caret_pos);
let _ = Terminal::show_caret();
let _ = Terminal::execute();
}

fn handle_resize_command(&mut self, size: Size){
    self.terminal_size = size;
    self.view.resize(Size{
        width: size.width,
        height: size.height.saturating_sub(2),
    });

    let bar_size = Size{
        height: 1,
        width: size.width,
    };
    self.message_bar.resize(bar_size);
    self.status_bar.resize(bar_size);
    self.command_bar.resize(bar_size);
}



// fn move_point(&mut self), key_code: KeyCode -> Result<(),std::io::Error> {
// let location {mut x, mut y} = self.location;
// let Size {}
// }

fn update_message(&mut self,new_msg: &str) {
    self.message_bar.update_message(new_msg);
}

fn in_prompt(&self) -> bool{
    !self.prompt_type.is_none()
}

fn set_prompt(&mut self, prompt_type: PromptType) {
    match prompt_type{
        PromptType::None => self.message_bar.mark_redraw(true),
        PromptType::Save => self.command_bar.set_prompt("Save as: "),
        PromptType::Find => {
            self.view.enter_search();
            self.command_bar.set_prompt("Find[Esc to exit and Use Arrows to Move]: ")
        }
    }
    self.command_bar.clear_value();
    self.prompt_type = prompt_type;
}

}

impl Drop for Editor{
fn drop(&mut self){
    let _ = Terminal::terminate();
    if self.should_quit{
    let _ = Terminal::print("Thanks For Using.\r\n");
    }
}
}
