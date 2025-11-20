mod terminal;
mod view;
use crossterm::event::{Event, KeyEvent, KeyEventKind, read};
use terminal::Terminal;
mod editorcommand;
use view::View;
use std::
{
    env,panic::{set_hook,take_hook}
};
use editorcommand::EditorCommand;

pub struct Editor{
should_quit: bool,
view:View,
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
    let mut view = View::default();
    let args: Vec<String> = env::args().collect();
    if let Some(filename) = args.get(1){
    view.load(filename);    
    }
    Ok(Self{
        should_quit: false,
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
            match EditorCommand::try_from(event) {
                Ok(command) => {
                    if matches!(command, EditorCommand::Quit) {
                        self.should_quit = true;
                    } else {
                        self.view.handle_command(command);
                    }
                }
                Err(err) => {
                    #[cfg(debug_assertions)]
                    {
                        panic!("Could not handle command: {err}");
                    }
                }
            }
        } 
}

fn refresh_screen(&mut self)
{
let _ = Terminal::hide_caret();
self.view.render();
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
