use std::{
    fmt::{self,Display},
    ops::{Range,Deref}
};
use unicode_width::UnicodeWidthStr;
use unicode_segmentation::UnicodeSegmentation;
use super::{AnnotatedString, AnnotationType};

mod textfragment;
mod graphemewidth;

use textfragment::TextFragment;
use graphemewidth::GraphemeWidth;

type GraphemeIdx = usize;
type ByteIdx = usize;
type ColIdx = usize;

#[derive(Default)]
pub struct Line{
fragments: Vec<TextFragment>,
string: String,
}

impl Line{

    pub fn from(line_str: &str) -> Self{
        debug_assert!(line_str.is_empty() || line_str.lines().count() == 1);
        let fragments = Self::str_to_fragments(line_str);
        Self {fragments,
            string: String::from(line_str),
        }
    }
    fn get_replacement_character(for_str: &str) -> Option<char> {
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

    pub fn get_visible_graphemes(&self,range: Range<ColIdx> )-> String{
        self.get_annotated_visible_substr(range,None,None).to_string()
    // if range.start >= range.end{
    //     return String::new();
    //     }
    //     let mut result = String::new();
    //     let mut current_pos = 0;

    //     for fragment in &self.fragments{
    //         let fragment_end = fragment.rendered_width.saturation_add(current_pos);
    //         if current_pos >= range.end{
    //             break;
    //         }
    //         if fragment_end > range.start{
    //             if fragment_end > range.end || current_pos < range.start{
    //             result.push('⋯');
                
    //             } else if let Some(char) = fragment.replacement {
    //                 result.push(char);
    //             }else{
    //                 result.push_str(&fragment.grapheme);
    //             }
                
    //         }
    //         current_pos = fragment_end;
    //     }
    //     result
    }
/*
    pub fn get(&self,range: Range<usize>) -> String{
        let start = range.start;
        let end = cmp::min(range.end,self.string.len());
        self.string.get(start..end).unwrap_or_default().to_string()

    }*/

    pub fn get_annotated_visible_substr(&self, range: Range<ColIdx>, query: Option<&str>, selected_match: Option<GraphemeIdx>) -> AnnotatedString{
        if range.start >= range.end{
            return AnnotatedString::default();
        }
        
        let mut result = AnnotatedString::from(&self.string);

        if let Some(query)= query{
            if !query.is_empty(){
                self.find_all(query,0..self.string.len()).iter().for_each(
                    |(start_byte_idx,grapheme_idx)|{
                        if let Some(selected_match) = selected_match{
                            if *grapheme_idx == selected_match{
                                result.add_annotation(
                                    AnnotationType::SelectedMatch,
                                    *start_byte_idx,
                                    start_byte_idx.saturating_add(query.len()),
                                );
                                return;
                            }
                        }
                        result.add_annotation(
                            AnnotationType::Match,
                            *start_byte_idx,
                            start_byte_idx.saturating_add(query.len()),
                        );
                    }
                );
            }
        }
        let mut fragment_start = self.width();
        for fragment in self.fragments.iter().rev(){
            let fragment_end = fragment_start;
            fragment_start = fragment_start.saturating_add(fragment.rendered_width.into());
            if fragment_start > range.end{
                continue;
            }
            if fragment_start < range.end && fragment_end > range.end{
                result.replace(fragment.start_byte_idx, self.string.len(), "@@@");
                continue;
            } else if fragment_start == range.end{
                result.replace(fragment.start_byte_idx, self.string.len(), "");
                continue;
            }
            if fragment_end <= range.start{
                result.replace(0,fragment.start_byte_idx.saturating_add(fragment.grapheme.len()),"");
                break;
            }else if fragment_start < range.start && fragment_end > range.start{
                result.replace(0,fragment.start_byte_idx.saturating_add(fragment.grapheme.len()),"@@@");
                break;
            }

            if fragment_start >= range.start && fragment_end <= range.end{
                if let Some(replacement) = fragment.replacement{
                    let start_byte_idx = fragment.start_byte_idx;
                    let end_byte_idx = start_byte_idx.saturating_add(fragment.grapheme.len());
                    result.replace(start_byte_idx,end_byte_idx,&replacement.to_string());
                }
            }
        }
        result
    }

    pub fn grapheme_count(&self) -> GraphemeIdx{
    //self.string.len()
//    let graphemes = self.string.graphemes(true).collect::<Vec<&str>>();
    self.fragments.len()
    }
    
    pub fn width_until(&self,at: GraphemeIdx) -> GraphemeIdx{
        self.fragments.iter().take(at)
            .map(|fragment| match fragment.rendered_width{
                GraphemeWidth::Half => 1,
                GraphemeWidth::Full => 2,
            }).sum()
    }

    fn str_to_fragments(line_str: &str) -> Vec<TextFragment> {
        line_str
            .grapheme_indices(true)
            .map(|(byte_idx, grapheme)| {
                let (replacement, rendered_width) = Self::get_replacement_character(grapheme)
                    .map_or_else(
                        || {
                            let unicode_width = grapheme.width();
                            let rendered_width = match unicode_width {
                                0 | 1 => GraphemeWidth::Half,
                                _ => GraphemeWidth::Full,
                            };
                            (None, rendered_width)
                        },
                        |replacement| (Some(replacement), GraphemeWidth::Half),
                    );

                TextFragment {
                    grapheme: grapheme.to_string(),
                    rendered_width,
                    replacement,
                    start_byte_idx: byte_idx,
                }
            })
            .collect()
    }

    pub fn width(&self) -> GraphemeIdx{
        self.width_until(self.grapheme_count())
    }

    pub fn append_char(&mut self, character: char){
        self.insert_char(character, self.grapheme_count());
    }

    pub fn delete_last(&mut self){
        self.delete(self.grapheme_count().saturating_sub(1));
    }

    pub fn insert_char(&mut self,character: char,at: GraphemeIdx){
        debug_assert!(at.saturating_sub(1) <= self.grapheme_count());
        if let Some(fragment) = self.fragments.get(at){
            self.string.insert(fragment.start_byte_idx,character);
        }else{
            self.string.push(character);
        }
        self.rebuild_fragments();
    }

    pub fn delete(&mut self,at: GraphemeIdx){
        if let Some(fragment) = self.fragments.get(at){
            let start = fragment.start_byte_idx;
            let end = fragment.start_byte_idx.saturating_add(fragment.grapheme.len());
            self.string.drain(start..end);
            self.rebuild_fragments();
            }
        }
    
    pub fn append(&mut self,other: &Self){
        self.string.push_str(&other.string);
        self.rebuild_fragments();
        }

    pub fn split(&mut self,at: GraphemeIdx) -> Self{
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

    fn byte_idx_to_grapheme_idx(&self, byte_idx: ByteIdx) -> Option<GraphemeIdx>{
        debug_assert!(byte_idx <= self.string.len());
        if byte_idx > self.string.len(){
            return None;
        }
        
        self.fragments
            .iter()
            .position(|fragment| fragment.start_byte_idx >= byte_idx)
    }

    fn grapheme_idx_to_byte_idx(&self, grapheme_idx: GraphemeIdx) -> ByteIdx{
        debug_assert!(grapheme_idx <= self.grapheme_count());
        if grapheme_idx == 0 || self.grapheme_count() == 0{
            return 0;
        }
        self.fragments.get(grapheme_idx)
            .map_or_else(|| {
                #[cfg(debug_assertions)]
                {
                    panic!("Fragment not found for grapheme index {grapheme_idx:?}");
                }
                #[cfg(not(debug_assertions))]
                {
                    0
                }
            },
            |fragment| fragment.start_byte_idx,
            )
    }

    // pub fn search(&self, query: &str, from_grapheme_idx: GraphemeIdx) -> Option<GraphemeIdx>{

    //     let start_byte_idx = self.grapheme_idx_to_byte_idx(from_grapheme_idx);

    //      self.string.get(start_byte_idx..)
    //       .and_then(|substr| {
    //             substr.find(query)
    //                 .and_then(|byte_idx| self.byte_idx_to_grapheme_idx(byte_idx.saturating_add(start_byte_idx)))
    //         })
    // }

    pub fn search_forward(&self, query: &str, from_grapheme_idx: GraphemeIdx) -> Option<GraphemeIdx>{
        debug_assert!(from_grapheme_idx <= self.grapheme_count());
        if from_grapheme_idx == self.grapheme_count(){
            return None;
        }

        let start_byte_idx = self.grapheme_idx_to_byte_idx(from_grapheme_idx);
      self.find_all(query, start_byte_idx..self.string.len())
            .first()
            .map(|(_, grapheme_idx)| *grapheme_idx)
    }

    pub fn search_backward(&self, query: &str, from_grapheme_idx: GraphemeIdx) -> Option<GraphemeIdx>{
        debug_assert!(from_grapheme_idx <= self.grapheme_count());
        if from_grapheme_idx == 0{
            return None;
        }

        let end_byte_idx = if from_grapheme_idx == self.grapheme_count(){
            self.string.len()
        } else{
            self.grapheme_idx_to_byte_idx(from_grapheme_idx)
        };

        self.find_all(query, 0..end_byte_idx)
            .last()
            .map(|(_, grapheme_idx)| *grapheme_idx)
    }

    fn find_all(&self, query: &str,range: Range<ByteIdx>) -> Vec<(ByteIdx,GraphemeIdx)>{
        let start_byte_idx = range.start;
        let end_byte_idx = range.end;

        self.string.get(start_byte_idx..end_byte_idx).map_or_else(Vec::new,
            |substr| {
                substr.match_indices(query)
                .filter_map(|(relative_start_idx,_)|{
                    let absolute_start_idx = relative_start_idx.saturating_add(start_byte_idx);
                    
                    self.byte_idx_to_grapheme_idx(absolute_start_idx).map(|grapheme_idx| (absolute_start_idx,grapheme_idx))
                }).collect()
            }
        )
    }

}


impl Display for Line{
    fn fmt(&self,formatter: &mut fmt::Formatter) -> fmt::Result{
        write!(formatter,"{}",self.string)
        }
}

impl Deref for Line{
    type Target = str;
    fn deref(&self) -> &Self::Target{
        &self.string
    }
}
