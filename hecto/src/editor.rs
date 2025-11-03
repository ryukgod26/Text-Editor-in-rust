mod terminal;
use crossterm::event::{read,Event,KeyModifiers,KeyCode::Char,Event::Key,KeyEvent};
use terminal::{Terminal,Position,Size};
use crossterm::cursor;
use crossterm::io::{stdout,Write};

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
    Terminal::print("~")?:
 if row+1 < rows{
//    print!("\r\n");}
    Terminal::print("\r\n");
    }
    Ok(())
}

}
