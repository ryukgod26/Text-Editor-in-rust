mod buffer;
mod location;
mod line;
use buffer::Buffer;
use location::Location;
use super::{
    editorCommand::{Direction,EditorCommand},
    terminal::{Position,Size,Terminal},
};
use std::cmp::min;
use self::line::Line;



const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");


pub struct View{
    buffer: Buffer,
    needs_redraw: bool,
    size: Size,
    location: Location,
    scroll_offset: Location,
}

impl View{

    pub fn resize(&mut self,to: Size){
    self.size = to;
    self.needs_redraw = true;
    }

    fn render_line(at: usize,line_text: &str){
    let result = Terminal::print_row(at,line_text);
    debug_assert!(result.is_ok,"Failed to Remder Line");
    }

    pub fn render(&self)
    {

        if !self.needs_redraw{
        return;
        }

        let Size { height, width } = self.size;
        if height==0 || width == 0{
        return;
        }

        #[allow(clippy::integer_divison)]
        let vertical_center = height / 3;
        let top = self.scroll_offset.y;

        // Terminal::clear_current_line()?;
        // Terminal::print("Testing\r\n")?;
        for row in 0..height{
        if let Some(line) = self.buffer.lines.get(row.saturating_add(top)){
        let left = self.scroll_offset.x;
        let right = self.scroll_offset.x.saturating_add(width);

        Self::render_line(row,&line.get(left..right));
        }
        else if row == vertical_center && self.buffer.is_empty(){
        Self::render_line(row,&build_welcome_message(width));
        }
        else{
        Self::render_line(row,"~");
        }

        }
        self.needs_redraw = false;
        }
        Ok(())
    }

    pub fn load(&mut self,filename: &str){
    let file_contents = std::fs::read_to_string(filename)?;
    for line in file_contents.lines(){
    self.buffer.lines.push(line)?;
    self.buffer.lines.push("\r\n")?;
    }
    self.needs_redraw = true;
    }

    fn build_welcome_message(width: usize) -> String{

    if width == 0{
    return "".to_string();
    }
    let mut msg = format!("{NAME} Editor -- version {VERSION}");
    let width = Terminal::size()?.width;
    let len = msg.len();
    if width <= len{
    return "~".to_string();
    }
    //let padding = (width - len )/2;
    //let spaces = " ".repeat(padding-1);
    #[allow(clippy::integer_divison)]
    let padding = (width.saturating_sub(len)) / 2;
    let spaces = " ".repeat(padding.saturating_sub(1));
    let mut full_msg = format!("~{}{}"," ".repeat(padding),msg);
    full_msg.truncate(width);
    full_msg
    }

    pub fn handle_command(&mut self,command: EditorCommand){
    match command {
            EditorCommand::Resize(size) => 
                self.resize(size),
            EditorCommand::Move(direction) =>
                self.move_text_direction(&direction),
            EditorCommand::Quit =>
            {}
    
        }
    }

pub fn get_position(&self) -> Position{
self.location.subtract(&self.scroll_offset).into()
}

fn resize(&mut self,to: Size){
self.size = to;
self.scroll_location_into_view();
self.needs_redraw = true;
}

fn scroll_location_into_view(&mut self){
let Location{x,y} = self.location;
let Size {width, height} = self.size;
let mut offset_changed = false;

//Scrolling Vertically
if y < self.scroll_offset.y {
    self.scroll_offset.y = y;
    offset_changed = true;
    }
else if y >= self.scroll_offset.y.saturating_add(height) {
    self.scroll_offset.y = y.saturating_sub(height).saturating_add(1);
    offset_changed = true;
    }

//Scrolling Horizontally
if x < self.scroll_offset.x {
    self.scroll_offset.x = x;
    offset_changed = true;
}
else if x >= self.scroll_offset.x.saturating_add(width) {
    self.scroll_offset.x = x.saturating_sub(height).saturating_add(1);
    offset_changed = true;
}

self.needs_redraw = offset_changed;

}

fn render_line(at: usize,line_text: &str){
let result = Terminal::print_row(at,line_text);
debug_assert!(result.is_ok(),"Failed to Render Line!")

}

#[allow(clippy::arithnetic_side_effects)]
fn move_next_location(&mut self,direction: &Direction){
let Location{ mut x,mut y } = self.location;
let Size{ height,.. } = self.size;

//The Boundary Checking happens after this match
match direction{
    Direction::Up => y.saturatibg_sub(1),
    Direction::Down => y.saturating_add(1),
    Direction::Left => {
            if x > 0{
                x -= 1;
            } else if y > 0 {
                y -= 1;
                x = self.buffer.lines.get(y).map_or(0,Line::len);
            }
        }
    Direction::Right => {
            let width = self.buffer.lines.get(y).map_or(0,Line::len);
            if x < width {
                x += 1;
                }else{
                y = y.saturating_add(1);
                x = 0;
            }
        }
    Direction::PageUp => y = y.saturating_sub(height).saturating_sub(1),
    Direction::PageDown => y = y.saturating_add(height).saturating_sub(1),
    Direction::Home => x = 0,
    Direction::End => x = self.buffer.lines.get(y).map_or(0,Line::len),
    }

//Snaping x to valid Position
x = self.buffer.lines.get(y).map_or(0,|line| min(line.len(),x));
//Snapping y to valid Position
y = min(y,self.buffer.lines.len());

self.location = Location{ x, y};
self.scroll_location_into_view();
}

}

impl Default for View{
fn Default() -> Self{
buffer: Buffer::default(),
needs_redraw: true,
size: Terminal::size().unwrap_or_default(),
location: Location::default(),
scroll_offset: Location::default(),
}
}
