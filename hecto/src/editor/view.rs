mod buffer;
mod line;

use buffer::Buffer;
use std::cmp::min;
use self::line::Line;
use super::{
    editorcommand::{Direction,EditorCommand},
    terminal::{Position,Size,Terminal},
    DocumentStatus,  NAME, VERSION
};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");


pub struct View{
    buffer: Buffer,
    needs_redraw: bool,
    size: Size,
    text_location: Location,
    scroll_offset: Position,
    margin_bottom: usize,
}

#[derive(Default,Copy,Clone)]
pub struct Location{
    pub grapheme_index: usize,
    pub line_index: usize,
}

impl View{

    pub fn new(margin_bottom: usize) -> Self{
        let terminal_size = Terminal::size().unwrap_or_default();
        Self{
            buffer: Buffer::default(),
            needs_redraw: true,
            size: Size{
                width: terminal_size.width,
                height: terminal_size.height.saturating_sub(margin_bottom),
            },
            text_location: Location::default(),
            scroll_offset: Position::default(),
        }
    }

    fn render_line(at: usize,line_text: &str){
    let result = Terminal::print_row(at,line_text);
    debug_assert!(result.is_ok(),"Failed to Remder Line");
    }

    pub fn render(&mut self) {
        if !self.needs_redraw || self.size.height == 0{
            return;
        }
        let Size { height, width } = self.size;
        if height == 0 || width == 0 {
            return;
        }

        #[allow(clippy::integer_division)]
        let vertical_center = height / 3;
        let top = self.scroll_offset.row;
        for current_row in 0..height {
            if let Some(line) = self.buffer.lines.get(current_row.saturating_add(top)) {
                let left = self.scroll_offset.col;
                let right = self.scroll_offset.col.saturating_add(width);
                Self::render_line(current_row, &line.get_visible_graphemes(left..right));
            } else if current_row == vertical_center && self.buffer.is_empty() {
                Self::render_line(current_row, &Self::build_welcome_message(width));
            } else {
                Self::render_line(current_row, "~");
            }
        }
        self.needs_redraw = false;
    }


    pub fn load(&mut self,filename: &str){
    if let Ok(buffer) = Buffer::load(filename) {
            self.buffer = buffer;
            self.needs_redraw = true;
        }
    }

    fn build_welcome_message(width: usize) -> String {
        if width == 0 {
            String::new()
        }
        let welcome_message = format!("{NAME} editor -- version {VERSION}");
        let len = welcome_message.len();
        let remain_width = width.saturating_sub(1);
        if remain_width < len {
            return "~".to_string();
        }
        format!("{:<1}{:^remain_width$}","~",welcome_message)
    }

    pub fn handle_command(&mut self,command: EditorCommand){
    match command {
            EditorCommand::Resize(size) => 
                self.resize(size),
            EditorCommand::Move(direction) =>
                self.move_text_location(&direction),
            EditorCommand::Quit => {},
            EditorCommand::Insert(Char) => self.insert_char(Char),
            EditorCommand::Backspace => self.backspace(),
            EditorCommand::Delete => self.delete(),
            EditorCommand::Enter => self.insert_newline(),
            EditorCommand::Save => self.save_file_to_disk(),
        }
    }

// pub fn get_position(&self) -> Position{
// self.location.subtract(&self.scroll_offset).into()
// }

fn insert_char(&mut self,character: char){
let old_len = self.buffer.lines.get(self.text_location.line_index).map_or(0,Line::grapheme_count);
self.buffer.insert_char(character,self.text_location);
let new_len = self.buffer.lines.get(self.text_location.line_index).map_or(0,Line::grapheme_count);
let grapheme_sub = new_len.saturating_sub(old_len);
if grapheme_sub > 0{
self.move_text_location(Direction::Right);
}
self.needs_redraw = true;

}

fn resize(&mut self,to: Size){
self.size = Size{
    width: to.width,
    height: to.height.saturating_sub(self.margin_bottom),
};
self.scroll_text_location_into_view();
self.needs_redraw = true;
}

fn scroll_vertically(&mut self,to: usize){
    let Size {height,..} = self.size;
    let offset_changed =  if to < self.scroll_offset.row{
            self.scroll_offset.row = to;
            true
        } else if to >= self.scroll_offset.row.saturating_add(height){
            self.scroll_offset.row = to.saturating_sub(height).saturating_add(1);
            true
        }
        else{
            false
        };
        self.needs_redraw = self.needs_redraw || offset_changed;

    }

fn scroll_horizontally(&mut self,to: usize){
    let Size { width,.. } = self.size;
    let offset_changed = if to < self.scroll_offset.col {
            self.scroll_offset.col = to;
            true
        } else if to >= self.scroll_offset.col.saturating_add(width) {
        self.scroll_offset.col = to.saturating_sub(width).saturating_add(1);
        true
        }else{
        false
        };
    self.needs_redraw = self.needs_redraw || offset_changed;
    }

fn scroll_text_location_into_view(&mut self){
    let Position { row, col} = self.text_location_to_position();
    self.scroll_vertically(row);
    self.scroll_horizontally(col);
    }



pub fn caret_position(&self) -> Position{
    self.text_location_to_position().saturating_sub(self.scroll_offset)
    }

fn text_location_to_position(&self) -> Position{
    let row = self.text_location.line_index;
    let col = self.buffer.lines.get(row).map_or(0,|line| {
    line.width_until(self.text_location.grapheme_index)
    });
    Position {col,row}
    }   

#[allow(clippy::arithnetic_side_effects)]
fn move_text_location(&mut self,direction: Direction){
let Size{ height,.. } = self.size;

//The Boundary Checking happens after this match
match direction{
    Direction::Up => self.move_up(1),
    Direction::Down => self.move_down(1),
    Direction::Left => self.move_left(),
    Direction::Right => self.move_right(),
    Direction::PageUp => self.move_up(height.saturating_sub(1)),
    Direction::PageDown => self.move_down(height.saturating_sub(1)),
    Direction::Home => self.move_to_start_of_line(),
    Direction::End => self.move_to_end_of_line(),
    }
self.scroll_text_location_into_view();
}

fn move_up(&mut self,step: usize){
    self.text_location.line_index = self.text_location.line_index.saturating_sub(step);
    self.snap_to_valid_grapheme();
    }

fn move_down(&mut self,step: usize){
    self.text_location.line_index = self.text_location.line_index.saturating_add(step);
    self.snap_to_valid_grapheme();
    self.snap_to_valid_line();
    }

fn snap_to_valid_line(&mut self){
    self.text_location.line_index = min(self.text_location.line_index,self.buffer.height());
    }

fn snap_to_valid_grapheme(&mut self){
    self.text_location.grapheme_index = self.buffer.lines
        .get(self.text_location.line_index).map_or(0,|line| {
    min(self.text_location.grapheme_index,line.grapheme_count())
        });
    }

fn move_to_start_of_line(&mut self) {
    self.text_location.grapheme_index = 0;
    }

fn move_to_end_of_line(&mut self){
self.text_location.grapheme_index = self.buffer.lines.get(self.text_location.line_index).map_or(0,Line::grapheme_count);
}

#[allow(clippy::arithmetic_side_effects)]
fn move_left(&mut self) {
    if self.text_location.grapheme_index > 0{
        self.text_location.grapheme_index -= 1;
        } else if self.text_location.line_index > 0{
        self.move_up(1);
        self.move_to_end_of_line();
        }
    }

#[allow(clippy::arithmetic_side_effects)]
fn move_right(&mut self){
let line_width = self.buffer.lines.get(self.text_location.line_index)
    .map_or(0,Line::grapheme_count);
if self.text_location.grapheme_index < line_width{
    self.text_location.grapheme_index += 1;
   }
else{
    self.move_to_start_of_line();
    self.move_down(1);
    }
}

fn delete(&mut self){
    self.buffer.delete(self.text_location);
    self.needs_redraw = true;
    }

fn backspace(&mut self){
    if self.text_location.line_index != 0 || self.text_location.grapheme_index != 0 {
//    self.move_left();
        self.move_text_location(Direction::Left);
        self.delete();
        }
    }

fn insert_newline(&mut self){
    self.buffer.insert_newline(self.text_location);
    self.move_text_location(Direction::Right);
    self.needs_redraw = true;
    } 

fn save_file_to_disk(&mut self){
let _ = self.buffer.save();
}

pub fn get_status(&self) -> DocumentStatus{
    DocumentStatus{
        total_lines: self.buffer.height(),
        current_line_index: self.text_location.line_index,
        filename: format!("{}",self.buffer.fileinfo),
        is_modified: self.buffer.dirty,
    }
    }

}

