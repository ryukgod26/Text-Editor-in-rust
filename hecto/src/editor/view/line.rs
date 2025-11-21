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
}

#[derive(Default)]
pub struct Line{
fragments: Vec<TextFragment>
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
        Self {fragments}
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
        String::new();
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

    fn str_to_fragnents(line_str: &str) -> Vec<TextFragment>{
        line_str.graphemes(true)
            .map(|grapheme| {
                let (replacement,rendered_width) = Self::replacement_character(grapheme) .map_or_else( || {
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
                }
            }).collect();
        }

    pub fn insert_char(&mut self,character: char,at: usize){
        let mut result = String::new();

        for (indux,fragment) in self.fragments.iter().enumerate(){
            if index == grapheme_index {
                result.push(character);
                }
            result.push_str(&fragment.grapheme);
            }
        if at >= self.fragments.len(){
                result.push(character);
            }
        self.fragments = Self::str_to_fragments(&result);
    }

    pub fn delete(&mut self,at: usize){
        let mut result = String::new();

        for (index,fragment) in self.fragments.iter().enumerate(){
            if index != at {
                result.push_str(&fragment.grapheme);
                }
            }
        self.fragments = Self::str_to_fragments(&result);
        }
    
    pub fn append(&mut self,other: &Self){
        let mut concat = self.to_string();
        concat.push_str(&other.to_string());
        self.fragments = Self::str_to_fragments(&concat);
        }

    pub fn split(&mut self,at: usize) -> Self{
        if at > self.fragments.len(){
            Self::deafult()
        }
        let remainder = self.fragments.split_off(at);
        Self{
            fragments: remainder,
        }
    }
}


impl fmt::Display for Line{
    fn fmt(&self,formatter: &mut fmt::Formatter) -> fmt::Result{
        let result: String = self.fragment.iter()
            .map(|fragment| fragment.grapheme.clone())
            .collect();
        write!(formatter,"{result}")
        }
}
