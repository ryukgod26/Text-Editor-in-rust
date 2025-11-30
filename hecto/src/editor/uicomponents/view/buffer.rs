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
     debug_assert!(at.line_idx <= self.height());
        if at.line_idx == self.height() {
            self.lines.push(Line::from(&character.to_string()));
            self.dirty = true;
        } else if let Some(line) = self.lines.get_mut(at.line_idx) {
            line.insert_char(character, at.grapheme_idx);
            self.dirty = true;
        }
    }

pub fn delete(&mut self,at: Location){
    if let Some(line) = self.lines.get(at.line_idx) {
            if at.grapheme_idx >= line.grapheme_count() && self.height() > at.line_idx.saturating_add(1){
                let next_line = self.lines.remove(at.line_idx.saturating_add(1));

                #[allow(clippy::indexing_slicing)]
                self.lines[at.line_idx].append(&next_line);
                self.dirty = true;
            } else if at.grapheme_idx < line.grapheme_count() {
                #[allow(clippy::indexing_slicing)]
                self.lines[at.line_idx].delete(at.grapheme_idx);
                self.dirty = true;
            }
        }
    }

pub fn insert_newline(&mut self,at: Location){
    if at.line_idx == self.height() {
        self.lines.push(Line::default());
        self.dirty = true;
        } else if let Some(line) = self.lines.get_mut(at.line_idx){
        let new = line.split(at.grapheme_idx);
        self.lines.insert(at.line_idx.saturating_add(1),new);
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
        }else{
            #[cfg(debug_assertions)]
            {
                panic!("Cannot Save File Without a path.");
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

    // pub fn search(self, query: &str, from: Location) -> Option<Location>{
    //     if query.is_empty(){
    //         return None;
    //     }
    //     let mut is_first = true;
    //     for (line_idx,line) in self.lines.iter().enumerate()
    //         .cycle().skip(from.line_idx).take(self.lines.len().saturating_add(1)){
    //             let from_grapheme_idx = if is_first{
    //                 is_first = false;
    //                 from.grapheme_idx
    //             }else{
    //                 if let Some(grapheme_idx) = line.search_forward(query,from_grapheme_idx){
    //                     return Some(Location{
    //                         grapheme_idx,
    //                         line_idx,
    //                     });
    //                 }
    //             };
    //     }
    //     None
    // }

    //Probable Buggy Function During Crash (First Dev Note )
    pub fn search_backward(&self, query: &str, from: Location) -> Option<Location>{
        if query.is_empty(){
            return None;
        }
        let mut is_first = true;

        for(line_idx, line) in self.lines.iter().enumerate()
            .rev().cycle().skip(self.lines.len().saturating_sub(from.line_idx).saturating_sub(1))
            .take(self.lines.len().saturating_add(1)){
                let from_grapheme_idx = if  is_first{
                    is_first = false;
                    from.grapheme_idx
                }else{
                    line.grapheme_count()
                };

                if let Some(grapheme_idx) = line.search_backward(query, from_grapheme_idx)
                {
               return Some(Location{
                   grapheme_idx,
                   line_idx,
               });
                }
        }
        None
    }

    pub fn search_forward(&self, query: &str, from: Location) -> Option<Location>{
        if query.is_empty(){
            return None;
        }
        let mut is_first = true;
        for (line_idx, line) in self.lines.iter()
            .enumerate().cycle().skip(from.line_idx).take(self.lines.len().saturating_add(1))
            {
                let from_grapheme_idx = if is_first{
                    is_first =  false;
                    from.grapheme_idx
                } else {
                    0
                };
                if let Some(grapheme_idx) = line.search_forward(query, from_grapheme_idx){
                    return Some(Location { grapheme_idx, line_idx });
                }
            }
            None
    }
    
}
