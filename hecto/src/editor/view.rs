mod buffer;
use buffer::Buffer;

use super::terminal::{Size,Terminal};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View{
    buffer: Buffer,
    needs_redraw: bool,
    size: Size
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


        // Terminal::clear_current_line()?;
        // Terminal::print("Testing\r\n")?;
        if !(self.buffer.is_empty()){
        for row in 0..height{
        if let Some(line) = self.buffer.lines.get(row){
        let truncated_line = if line.len() >= width {
        &line[0..width]
        }else{
        line
        };
        Self::render_line(row,truncated_line);
        }
        else if row == vertical_center && self.buffer.is_empty(){
        Self::render_line(row,&build_welcome_message(width));
        }//
        else{
        Self::render_line(row,"~");
        }

        }
        }
        else{
            #[allow(clippy::integer_division)]
            if row == height/3{
                Self::welcome_message()?;
             }else{
                 Self::draw_empty_row()?;
             }
             if row.saturating_add(1) < height{
                 Terminal::print("\r\n")?;
             }
        }
        Ok(())
    }

    pub fn load(&mut self,filename: &str) -> Result<(),std::io::Error>{
    let file_contents = std::fs::read_to_string(filename)?;
    for line in file_contents.lines(){
    self.buffer.lines.push(line)?;
    self.buffer.lines.push("\r\n")?;
    }
    self.needs_redraw = true;
    Ok(())
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

}

impl Default for View{
fn Default() -> Self{
buffer: Buffer::default(),
needs_redraw: true,
size: Terminal::size().unwrap_or_default(),

}
}
