use std::fs::{File,read_to_string};
use super::Line;
use super::Location;
use std::io::Write;
use super::FileInfo;

#[derive(Default)]
pub struct Buffer{
    pub lines :Vec<Line>,
    pub fileinfo: FileInfo,
    pub dirty: bool,
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
Ok(Self{
    lines,
    fileinfo: FileInfo::from(filename),
    dirty: false,
    })
}

pub fn height(&self) -> usize {
self.lines.len()
}

pub fn insert_char(&mut self,character: char,at: Location){
    if at.line_index > self.height() {
            return;
        }
    else if at.line_index == self.height() {
        self.lines.push(Line::from(&character.to_string()));
        self.dirty = true;
        }
    else if let Some(line) = self.lines.get_mut(at.line_index){
            line.insert_char(character,at.grapheme_index);
            self.dirty = true;
        }
    }

pub fn delete(&mut self,at: Location){
    if let Some(line) = self.lines.get(at.line_index) {
            if at.grapheme_index >= line.grapheme_count() && self.height() > at.line_index.saturating_add(1){
                let next_line = self.lines.remove(at.line_index.saturating_add(1));

                #[allow(clippy::index_slicing)]
                self.lines[at.line_index].append(&next_line);
                self.dirty = true;
            } else if at.grapheme_index < line.grapheme_count() {
                #[allow(clippy::index_slicing)]
                self.lines[at.line_index].delete(at.grapheme_index);
                self.dirty = true;
            }
        }
    }

pub fn insert_newline(&mut self,at: Location){
    if at.line_index == self.height() {
        self.lines.push(Line::default());
        self.dirty = true;
        } else if let Some(line) = self.lines.get_mut(at.line_index){
        let new = line.split(at.grapheme_index);
        self.lines.insert(at.line_index.saturating_add(1),new);
        self.dirty = true;
        }

    }

    pub fn save(&mut self) -> Result<(),std::io::Error>{
        self.save_to_file(&self.fileinfo)?;
        self.dirty = false;
        Ok(())
    }
    
    fn save_to_file(&self, fileinfo: &FileInfo) -> Result<(),std::io::Error>{
        if let Some(filepath) = &fileinfo.get_path() {
            let mut file = File::create(filepath)?;

            for line in &self.lines{
                writeln!(file,"{line}")?;
            }
        }
        Ok(())
    }

    pub fn save_as(&mut self, filename: &str) -> Result<(),std::io::Error>{
        let fileinfo = FileInfo::from(filename);
        self.save_to_file(&fileinfo)?;
        self.fileinfo = fileinfo;
        self.dirty = false;
        Ok(())
    }

    pub const fn is_file_loaded(&self) -> bool{
        self.fileinfo.has_path()
    }

}
