use crossterm::event::{read,Event::Key,KeyEvent, KeyCode::Char,KeyModifiers};
mod terminal;
use terminal::Terminal;


pub struct Editor{
should_quit : bool,
}

impl Editor {
    pub const fn default() -> Self {
        Self { should_quit: false }
    }
    pub fn run(&mut self) {
    
        Terminal::intialize().unwrap();

        let result = self.repl();
        Terminal::terminate.unwrap();
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
     self.evaluate_event(&event);*/
     self.refresh_screen()?;


            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evaluate_event(&event);
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
Terminal::clear_screen()?;
println!("Thanks For Using.\r\n");
}else {
Self::draw_rows()?;
Terminal::move_to_cursor(0,0)?;
}

Ok(())
}

fn draw_rows(){

let rows = Terminal::size()?.1;

for row in 0..rows{
 print!("~");
 if row+1 < height{
    print("\r\n");}
    }
}

}

