use unicode_segmentation::UnicodeSegmentation;
use std::{fmt,ops::Range};
use unicode_width::UnicodeWidthStr;

#[derive(Copy,Clone)]
enum GraphemeWidth{
Half,
Full
}

struct TextFragment{
grapheme: String,
rendered_width: GraphemeWidth,
replacement: Option<char>,
start_byte_idx: usize,
}

#[derive(Default)]
pub struct Line{
fragments: Vec<TextFragment>,
string: String,
}

impl GraphemeWidth{

    pub fn saturation_add(&self,other: usize) -> usize{
    match self{
            Self::Half=> other.saturating_add(1),
            Self::Full=> other.saturating_add(2),
        
        }

    }

}

impl Line{

    pub fn from(line_str: &str) -> Self{
        let fragments = Self::str_to_fragments(line_str);
        Self {fragments
            string: String::from(line_str),
        }
    }
    fn replacement_character(for_str: &str) -> Option<char> {
    let width = for_str.width();
    match for_str{
        " " => None,
        "\t"=>Some(' '),
        _ if width > 0 && for_str.trim().is_empty() => Some('_'),
        _ if width == 0 => {
                let mut chars = for_str.chars();
                if let Some(ch) = chars.next()  {
                    if ch.is_control() && chars.next().is_none(){
                        return Some('[');
                    }
                }
                Some('.')
            }
            _=> None,
        }

    }

    pub fn get_visible_graphemes(&self,range: Range<usize> )-> String{
    if range.start >= range.end{
        return String::new();
        }
        let mut result = String::new();
        let mut current_pos = 0;

        for fragment in &self.fragments{
            let fragment_end = fragment.rendered_width.saturation_add(current_pos);
            if current_pos >= range.end{
                break;
            }
            if fragment_end > range.start{
                if fragment_end > range.end || current_pos < range.start{
                result.push('⋯');
                
                } else if let Some(char) = fragment.replacement {
                    result.push(char);
                }else{
                    result.push_str(&fragment.grapheme);
                }
                
            }
            current_pos = fragment_end;
        }
        result
    }
/*
    pub fn get(&self,range: Range<usize>) -> String{
        let start = range.start;
        let end = cmp::min(range.end,self.string.len());
        self.string.get(start..end).unwrap_or_default().to_string()

    }*/

    pub fn grapheme_count(&self) -> usize{
    //self.string.len()
//    let graphemes = self.string.graphemes(true).collect::<Vec<&str>>();
    self.fragments.len()
    }
    
    pub fn width_until(&self,at: usize) -> usize{
        self.fragments.iter().take(at)
            .map(|fragment| match fragment.rendered_width{
                GraphemeWidth::Half => 1,
                GraphemeWidth::Full => 2,
            }).sum()
    }

    fn str_to_fragments(line_str: &str) -> Vec<TextFragment>{
        line_str.grapheme_indices(true)
            .map( |(byte_idx,grapheme)|
                let replacement, rendered_width = Self::get_replacement_character(grapheme)
                    .map_or_else( || {
                    let unicode_width = grapheme.width();
                    let rendered_width = match unicode_width{
                        0 |1 => GraphemeWidth::Half,
                        _ => GraphemeWidth::Full,
                    };
                    (None,rendered_width)
                },
                |replacement| (Some(replacement),GraphemeWidth::Half)
                );
                TextFragment{
                    grapheme: grapheme.to_string(),
                    rendered_width,
                    replacement,
                    stsrt_byte_idx: byte_idx
                }
            }).collect()
        }

    pub fn width(&self) -> usize{
        self.width_until(self.grapheme_count())
    }

    pub fn append_char(&mut self, character: char){
        self.insert_char(character, self.grapheme_count());
    }

    pub fn delete_last(&mut self){
        self.delete(self.grapheme_count().saturating_sub(1));
    }

    pub fn insert_char(&mut self,character: char,at: usize){
        if let Some(fragment) = self.fragments.get(at){
            self.string.insert(fragment.start_byte_idx,character);
        }else{
            self.string.push(character);
        }
        self.rebuild_fragments();
    }

    pub fn delete(&mut self,at: usize){
        if let Some(fragment) = self.fragments.get(at){
            let start = fragment.start_byte_idx;
            let end = fragment.start_byte_idx.saturating_add(fragment.grapheme.len())
            self.string.drain(start..end);
            self.rebuild_fragments();
            }
        }
    
    pub fn append(&mut self,other: &Self){
        self.string.push_str(&other.string);
        self.rebuild_fragments();
        }

    pub fn split(&mut self,at: usize) -> Self{
        if let Some(fragment) = self.fragments.get(at){
            let remainder = self.string.split_off(fragment.start_byte_idx);
            self.rebuild_fragments();
            Self::from(&remainder)
        }else{
            Self::default()
        }
    }

    fn rebuild_fragments(&mut self) {
        self.fragments = Self::str_to_fragments(&self.string);
    }    

}


impl fmt::Display for Line{
    fn fmt(&self,formatter: &mut fmt::Formatter) -> fmt::Result{
        write!(formatter,"{}",self.string)
        }
}
