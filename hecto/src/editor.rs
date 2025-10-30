use crossterm::event::{read,Event::key, KeyCode::char};
use crossterm::terminal::{disable_raw_mode,enable_raw_mode};

pub struct Editor{
should_quit : bool,
}

impl Editor{

pub fn default() ->self{
Editor(should_quit=false)
}

pub fn repl(&mut self) -> Result<(),std::io::Error>
{
emable_row_mode()?;

loop{
if let Key(KeyEvent{
code,modifiers,kind,state
}) = read()?{
println!("Code: {code:?}, Modifiers: {modifiers:?},Kind: {kind:?}, State: {state:?}");
match code {
    Char('q') if modifiers == KeyModifiers::CONTROL =>{
    self.should_quit = true;
    },
    _ =>(),
    }
}

if self.should_quit{
break;
}

}

disable_row_mode()?;
Ok(());
}


pub fn run(&mut self){

if let Err = self.repl(){
panic!("{err:#?}");
}
println!("Thanks For Using\r\n");

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
    }*/
}


}

