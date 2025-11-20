use unicode_segmentation::UnicodeSegmentation;
use std::{cmp,ops::Range};
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

pub struct Line{
fragments: Vec<TextFragment>
}

impl GraphemeWidth{

    pub fn saturation_add(&self,other: usize) -> usize{
    match self{
            Self::Half=> other.saturation_add(1),
            Self::Full=> other.saturation_add(2),
        
        }

    }

}

impl Line{

    pub fn from(line_str: &str) -> Self{
        let fragements = line_str.graphemes(true)
            .map(|grapheme| {
            let unicode_width = grapheme.width();
            let rendered_width = match unicode_width {
                0 | 1 => GraphemeWidth::Half,
                _ => GraphemeWidth::Full,
                };
            let replacement = match unicode_widtg{
                    0 => Some('.'),
                    _ => None
                };
            TextFragment {
                grapheme: grapheme.to_string(),
                rendered_width: rendered_width,

            }

            }).collect();
        Self{ fragments }
    }

    pub fn get_visible_graphemes(&self,range: Ramge<usize> )-> String{
    if range.start >= range.end{
        String::new()
        }
        let mut result = String::new();
        let mut current_pos = 0;

        for fragment in &self.fragments{
            let fragment_end = fragment.rendered_width.saturating_add(current_pos);
            if current_pos >= range.end{
                break;
            }
            if fragment_end > range.start{
                if fragment_end > range.end || current_pos < range.start{
                result.push('•••');
                
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
    
    pub fn width_until(&self,grapheme_index: usize) -> usize{
        self.fragments.iter().take(grapheme_index)
            .map(|fragment| match fragent.rendered_width{
                GraphemeWidth::Half => 1,
                GraphemeWidth::Full => 2,
            }).sum()
    }

}
