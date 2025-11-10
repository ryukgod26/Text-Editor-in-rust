mod buffer;
use buffer::Buffer;

use super::terminal::{Size,Terminal};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
pub struct View{
    buffer:Buffer
}

impl View{

    pub fn render(&self) -> Result<(),std::io::Error>
    {
        let Size { height, .. } = Terminal::size()?;
        // Terminal::clear_current_line()?;
        // Terminal::print("Testing\r\n")?;

        for row in 0..height{

            Terminal::clear_current_line()?;
        if let Some(line) = self.buffer.lines.get(row){
            Terminal::print(line)?;
            Terminal::print("\r\n")?;
            continue;
        }
            // #[allow(clippy::integer_division)]
            // if row == height/3{
            //     Self::welcome_message()?;
            // }else{
            //     Self::draw_empty_row()?;
            // }
            // if row.saturating_add(1) < height{
            //     Terminal::print("\r\n")?;
            // }
        }
        Ok(())
    }

    fn welcome_message() -> Result<(),std::io::Error>{

    let mut msg = format!("{NAME} Editor -- version {VERSION}");
    let width = Terminal::size()?.width;
    let len = msg.len();
    //let padding = (width - len )/2;
    //let spaces = " ".repeat(padding-1);
    #[allow(clippy::integer_divison)]
    let padding = (width.saturating_sub(len)) / 2;
    let spaces = " ".repeat(padding.saturating_sub(1));
    msg = format!("~{spaces}{msg}");
    msg.truncate(width);
    Terminal::print(&msg)?;
    Ok(())
    }

    fn draw_empty_row() -> Result<(),std::io::Error>{
    Terminal::print("~")?;
    Ok(())
    }
}