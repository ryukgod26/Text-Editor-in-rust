use crossterm::terminal::{size,disable_raw_mode,enable_raw_mode,Clear,ClearType};
use crossterm::cursor::MoveTo;
use std::io::{stdout,Write};
//use crossterm::execute;
use crossterm::cursor::{Hide,Show};

pub struct Terminal{}

impl Terminal{
pub fn intialize() -> Result<(),std::io::Error>{
enable_raw_mode()?;
Self::clear_screen()?;
Self::move_cursor_to(0,0)?;
Ok(())
}

pub fn terminate() -> Result<(),std::io::Error>{
disable_raw_mode()?;
Ok(())
}

pub fn clear_screen() -> Result<(),std::io::Error>{
queue!(stdout(),Clear(ClearType::All))?;
Ok(())
}

pub fn clear_current_line() -> Result<(),std::io::Error>{
queue!(stdout(),Clear(ClearType::CurrentLine)?;
Ok(())
}


pub fn move_cursor_to(x: u16,y: u16) -> Result<(),std::io::Error>{
queue!(stdout(),MoveTo(x,y))?;
Ok(())
}

pub fn size() -> Result<(u16, u16),std::io::Error>
{
size()
}

pub fn hide_cursor() -> Result<(),std::io::Error>{
queue!(stdout(),Hide)?;
Ok(())
}

pub fn show_cursor() -> Result<(),std::io::Error>{
queue!(stdout(),Show)?;
Ok(())
}


}
