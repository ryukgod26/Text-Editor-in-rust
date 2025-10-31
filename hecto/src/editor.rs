use crossterm::event::{read,Event::Key,KeyEvent, KeyCode::Char,KeyModifiers};
use crossterm::terminal::{disable_raw_mode,enable_raw_mode,Clear,ClearType};
use std::io::out;

pub struct Editor{
should_quit : bool,
}

impl Editor {
    pub fn default() -> Self {
        Editor { should_quit: false }
    }
    pub fn run(&mut self) {
        Self.intialize().unwrap();
        let result = self.repl();
        Self::terminate.unwrap();
        result.unwrap();
        /*
        if let Err(err) = self.repl() {
            panic!("{err:#?}");
        }
        print!("Thanks For Using.\r\n");*/
    }
    
    fn intialize() -> Result<(),std::io::Error>
    {
    enable_raw_mode.unwrap()?;
    Self::clear_screen()
    }

    fn terminate() -> Result<(),std::io::Error>
    {
    disable_raw_mode()
    }

    fn clear_screen() -> Result<(),std::io::Error>
    {
    let mut stdout = stdout();
    execute!(stdout,Clear(ClearType::All))
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
            }*/
    
     let event = read()?;
     self.evaluate_event(&event);
     self.refresh_screen()?;


            if self.should_quit {
                break;
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

fn evaluate_event(&mut self,event:&Event)
{
if let Key(KeyEvent {
    code,
    modifiers,
    ..
}) = event;

match code 
    {
    Char('q') if *modifiers == KeyModifiers::CONTROL =>{
    self.should_quit = true;
    },
    _=>(),
    }

}

fn refresh_screen(&self) -> Result<(),std::io::Error>
{
if self.should_quit{
Self::clear_screen()?;
println!("Thanks For Using.\r\n");

}

Ok(())
}



}
