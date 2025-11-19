use crossterm::terminal::{size,disable_raw_mode,enable_raw_mode,Clear,ClearType,EnterAlternateScreen,LeaveAlternateScreen};
use crossterm::cursor::MoveTo;
use std::io::{stdout,Write};
//use crossterm::execute;
use crossterm::cursor::{Hide,Show};
use crossterm::{queue,Command};
use crossterm::style::Print;

pub struct Terminal{}

#[derive(Default,Copy,Clone)]
pub struct Size{
pub height: usize,
pub width: usize,
}

#[derive(Copy,Clone,Default)]
pub struct Position{
pub col: usize,
pub row: usize,
}

impl Terminal{
pub fn intialize() -> Result<(),std::io::Error>{
enable_raw_mode()?;
Self::enter_alternate_screen()?;
Self::clear_screen()?;
// Self::move_cursor_to(Position{x: 0,y: 0 })?;
Self::execute()?;
Ok(())
}

pub fn terminate() -> Result<(),std::io::Error>{
Self::leave_alternate_screen()?;
Self::show_caret()?;
Self::execute()?;
disable_raw_mode()?;
Ok(())
}

pub fn clear_screen() -> Result<(),std::io::Error>{
Self::queue_command(Clear(ClearType::All))?;
Ok(())
}

pub fn clear_current_line() -> Result<(),std::io::Error>{
Self::queue_command(Clear(ClearType::CurrentLine))?;
Ok(())
}


pub fn move_caret_to(position: Position) -> Result<(),std::io::Error>{
#[allow(clippy::as_conversions, clippy::cast_possible_truncation)]
Self::queue_command(MoveTo(position.col as u16, position.row as u16))?;
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

pub fn hide_caret() -> Result<(),std::io::Error>{
Self::queue_command(Hide)?;
Ok(())
}

pub fn show_caret() -> Result<(),std::io::Error>{
Self::queue_command(Show)?;
Ok(())
}

pub fn print(string: &str) -> Result<(),std::io::Error>{
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

pub fn enter_alternate_screen() -> Result<(),std::io::Error>{
Self::queue_command(EnterAlternateScreen)?;
Ok(())
}

pub fn leave_alternate_screen() -> Result<(),std::io::Error>
{
Self::queue_command(LeaveAlternateScreen)?;
Ok(())
}

pub fn print_row(row: usize,line_text: &str) -> Result<(),std::io::Error>{
Self::move_caret_to(Position{row,col:0})?;
Self::clear_current_line()?;
Self::print(line_text)?;
Ok(())

}

}
