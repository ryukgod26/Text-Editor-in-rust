use std::fs::read_to_string;
use super::line::Line;
use super::Location;

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

pub fn insert_char(&mut self,character: char,at: Location){
    if at.line_index > self.height() {
            return;
        }
    else if at.line_index == self.lines.len() {
        self.lines.push(Line::from(&character.to_string()));
        }
    else if let Some(line) = self.lines.get_mut(at.line_index){
            line.insert_char(character,at.grapheme_index);
        }
    }

pub fn delete(&mut self,at: Location){
    if let Some(line) = self.lines.get(at.line_index) {
            if at.grapheme_index >= line.grapheme_count && self.height() > at.line_index.saturating_add(1){
                let next_line = self.lines.remove(at.line_index.saturating_add(1));

                #[allow(clippy::index_slicing)]
                self.lines[at.line_index].append(&next_line);
            } else if at.grapheme_index < line.grapheme_count() {
                #[allow(clippy::index_slicing)]
                self.lines[at.line_index].remove(at.grapheme_index);
            }
        }
    }

pub fn insert_newline(&mut self,at: Location){
    if at.line_index == self.heifht() {
        self.push(Line::default());
        } else if let Some(line) = self.lines.get_mut(at.line_index){
        let new = line.split(at.grapheme_index);
        self.lines.insert(at.line_index.saturating_add(1),new);
        }

    }

}
