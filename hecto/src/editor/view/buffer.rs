use std::fs::read_to_string;
use super::line::Line;


#[derive(Default)]
pub struct Buffer{
    pub lines :Vec<Line>
}

impl Buffer{
pub fn is_empty(&self) -> bool{
self.lines.is_empty()
}

pub fn load(filename: &str) -> Result<Self,std::io::Error>{
let contents = read_to_string(filename)?;
let mut lines = Vec::new();
for line in contents.lines(){
    lines.push(Line::from(line));
}
Ok(Self{lines})
}

pub fn height(&self) -> usize {
self.lines.len()
}

}
