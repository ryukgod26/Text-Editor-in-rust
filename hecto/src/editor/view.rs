mod buffer;
mod fileinfo;

use buffer::Buffer;
use std::{cmp::min, fmt::Error};
use super::{
    editorcommand::{Edit,Move},
    Position,Size,Terminal,
    DocumentStatus,  NAME, VERSION,
    UIComponent,Line
};
use fileinfo::FileInfo;

// const NAME: &str = env!("CARGO_PKG_NAME");
// const VERSION: &str = env!("CARGO_PKG_VERSION");

struct SearchInfo{
    prev_location: Location,
}

#[derive(Default)]
pub struct View{
    buffer: Buffer,
    needs_redraw: bool,
    size: Size,
    text_location: Location,
    scroll_offset: Position,
    search_info: Option<SearchInfo>,
}

#[derive(Default,Copy,Clone)]
pub struct Location{
    pub grapheme_index: usize,
    pub line_index: usize,
}

impl View{

    

    fn render_line(at: usize,line_text: &str) -> Result<(),std::io::Error>{
    Terminal::print_row(at,line_text)
    }



    pub fn load(&mut self,filename: &str) -> Result<(),std::io::Error>{
            let buffer = Buffer::load(filename)?;
            self.buffer = buffer;
            self.mark_redraw(true);
            Ok(())
    }

    pub const fn is_file_loaded(&self) -> bool{
        self.buffer.is_file_loaded()
    }

    fn build_welcome_message(width: usize) -> String {
        if width == 0 {
            String::new();
        }
        let welcome_message = format!("{NAME} editor -- version {VERSION}");
        let len = welcome_message.len();
        let remain_width = width.saturating_sub(1);
        if remain_width < len {
            return "~".to_string();
        }
        format!("{:<1}{:^remain_width$}","~",welcome_message)
    }

    pub fn handle_edit_command(&mut self,command: Edit){
    match command {
            Edit::Insert(Char) => self.insert_char(Char),
            Edit::Backspace => self.backspace(),
            Edit::Delete => self.delete(),
            Edit::Enter => self.insert_newline(),
        }
    }

// pub fn get_position(&self) -> Position{
// self.location.subtract(&self.scroll_offset).into()
// }

    pub fn handle_move_command(&mut self, command: Move){
        let Size{height,..} = self.size;
        match command{
            Move::Up => self.move_up(1),
            Move::Down => self.move_down(1),
            Move::Left => self.move_left(),
            Move::Right => self.move_right(),
            Move::PageUp => self.move_up(height.saturating_sub(1)),
            Move::PageDown => self.move_down(height.saturating_sub(1)),
            Move::Home => self.move_to_start_of_line(),
            Move::End => self.move_to_end_of_line(),
        }
        self.scroll_text_location_into_view();
    }
fn insert_char(&mut self,character: char){
let old_len = self.buffer.lines.get(self.text_location.line_index).map_or(0,Line::grapheme_count);
self.buffer.insert_char(character,self.text_location);
let new_len = self.buffer.lines.get(self.text_location.line_index).map_or(0,Line::grapheme_count);
let grapheme_sub = new_len.saturating_sub(old_len);
if grapheme_sub > 0{
self.handle_move_command(Move::Right);
}
self.mark_redraw(true);

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
        if offset_changed{
            self.mark_redraw(true);
        }

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
    if offset_changed{
        self.mark_redraw(true);
    }
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
fn move_text_location(&mut self,direction: Move){
let Size{ height,.. } = self.size;

//The Boundary Checking happens after this match
match direction{
    Move::Up => self.move_up(1),
    Move::Down => self.move_down(1),
    Move::Left => self.move_left(),
    Move::Right => self.move_right(),
    Move::PageUp => self.move_up(height.saturating_sub(1)),
    Move::PageDown => self.move_down(height.saturating_sub(1)),
    Move::Home => self.move_to_start_of_line(),
    Move::End => self.move_to_end_of_line(),
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
    self.mark_redraw(true);
    }

fn backspace(&mut self){
    if self.text_location.line_index != 0 || self.text_location.grapheme_index != 0 {
//    self.move_left();
        self.handle_move_command(Move::Left);
        self.delete();
        }
    }

fn insert_newline(&mut self){
    self.buffer.insert_newline(self.text_location);
    self.handle_move_command(Move::Right);
    self.mark_redraw(true);
    } 

pub fn save_file_to_disk(&mut self) -> Result<(),std::io::Error>{
    self.buffer.save()
}

pub fn save_as(&mut self, filename: &str) -> Result<(),std::io::Error>{
    self.buffer.save_as(filename)
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


impl UIComponent for View{
    fn mark_redraw(&mut self,val: bool){
        self.needs_redraw = val;
    }

    fn needs_redraw(&self) -> bool{
        self.needs_redraw
    }

    fn set_size(&mut self, size: Size){
        self.size = size;
        self.scroll_text_location_into_view();
    }

    fn draw(&mut self,origin_row: usize) -> Result<(),std::io::Error>{
    
        let Size { height, width } = self.size;
        let end_y = origin_row.saturating_add(height);
        
        #[allow(clippy::integer_divison)]
        let top_third = height / 3;
        let scroll_top = self.scroll_offset.row;
        
        for current_row in origin_row..end_y{
            let line_index = current_row.saturating_sub(origin_row).saturating_add(scroll_top);
            if let Some(line) = self.buffer.lines.get(line_index) {
                let left = self.scroll_offset.col;
                let right = self.scroll_offset.col.saturating_add(width);
                Self::render_line(current_row,&line.get_visible_graphemes(left..right))?;
            } else if current_row == top_third && self.buffer.is_empty(){
                Self::render_line(current_row,&Self::build_welcome_message(width))?;
            }else{
                Self::render_line(current_row,"~")?;
            }
        }
    Ok(())
    }

    pub fn enter_search(&mut self){
        self.search_info = Some(SearchInfo{
            prev_location: self.text_location
        });
    }

    pub fn exit_search(&mut self){
        self.search_info = None;
    }

    pub fn dismiss_search(&mut self){
        if let Some(searcg_info) = &self.search_info{
            self.text_location = search_info.prev_location
        }
        self.search_info = None;
        self.scroll_text_location_into_view();
    }

    pub fn search(&mut self, query; &str){
        if query.is_empty(){
            return;
        }

        if let Some(location) = self.buffer.search(query){
            self.text_location = location;
            self.scroll_text_location_into_view();
        }
    }
}
