use crossterm::terminal::{size,disable_raw_mode,enable_raw_mode,Clear,ClearType};
use crossterm::cursor::MoveTo;
use std::io::{stdout,Write};
//use crossterm::execute;
use crossterm::cursor::{Hide,Show};
use crossterm::{queue,Command};
use core::fmt::Display;

pub struct Terminal{}

#[derive(Copy,Clone)]
pub struct Size{
width: usize,
height: usize,
}

#[derive(Copy,Clone)]
pub struct Position{
x: usize,
y: usize,
}

impl Terminal{
pub fn intialize() -> Result<(),std::io::Error>{
enable_raw_mode()?;
Self::clear_screen()?;
Self::move_cursor_to(Position{x: 0,y: 0 })?;
Self::execute()?;
Ok(())
}

pub fn terminate() -> Result<(),std::io::Error>{
Self::execute()?;
disable_raw_mode()?;
Ok(())
}

pub fn clear_screen() -> Result<(),std::io::Error>{
Self::queue_command(stdout(),Clear(ClearType::All))?;
Ok(())
}

pub fn clear_current_line() -> Result<(),std::io::Error>{
Self::queue_command(stdout(),Clear(ClearType::CurrentLine)?;
Ok(())
}


pub fn move_cursor_to(position: Position) -> Result<(),std::io::Error>{
Self::queue_command(stdout(),MoveTo(position.x as u16, position.y as u16))?;
Ok(())
}

pub fn size() -> Result<Size,std::io::Error>
{
let (width_16,height_16) = size()?;
#[allow(clippy::as_conversions)]
let height = height_16 as usize;

#[allow(clippy::as_conversions)]
let width = width_16 as usize;
Ok(Size{height, width})
}

pub fn hide_cursor() -> Result<(),std::io::Error>{
Self::queue_command(stdout(),Hide)?;
Ok(())
}

pub fn show_cursor() -> Result<(),std::io::Error>{
Self::queue_command(stdout(),Show)?;
Ok(())
}

pub fn print<T:Display>(string:T) -> Result<(),std::io::Error>{
Self::queue_command(Print(string))?;
Ok(())
}

pub fn execute() -> Result<(),std::io::Error>{
stdout().flush()?;
Ok(())
}

fn queue_command<T:Command>(command:T) -> Result<(),std::io::Error>{
queue!(stdout(),command)?;
Ok(())
}

}
