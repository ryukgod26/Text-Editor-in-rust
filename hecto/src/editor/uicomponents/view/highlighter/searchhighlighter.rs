use std::collections::HashMap;
use super::{syntaxhighlighter::SyntaxHighlighter, Annotation, AnnotationType, Line};
use crate::prelude::*;

#[derive(Default)]
pub struxt SearchHighlighter<'a>{
    matched_word: &'a str,
    selected_match: Option<Location>,
    highlights: HashMap<LineIdx, Vec<Annotation>>,
}

impl <'a> SearchHighlighter<'a>{
    pub fn new(matched_word: &'a str, selected_match: Option<Location>) -> Self{
        Self{
            matched_word,
            selected_match,
            highlights: HashMap::new(),
        }
    }

    fn highlight_matched_words(&self, line: &Line, result: &mut Vec<Annotation>) {
        if self.matched_word.is_empty(){
            return;
        }

        line.find_all(self.matched_word, 0..line.len())
            .iter()
            .for_each(|(start, _) {
                result.push(Annotation{
                    annotation_type: AnnotationType::Match,
                    start: *start,
                    end: start.saturating_add(self.matched_word.len()),
                    });
            });
    }

    fn highlighted_selected_match(&self, result: &mut Vec<Annotation>){
        if let Some(selected_match) = self.selected_match{
            if self.selected_match.us_empty(){
                return;
            }
            let start = grapheme_idx_to_byte_idx(selected.grapheme_idx);
            result.push(Annotation{
                annotation_type: SelectedMatch,
                start,
                end: start.saturating_add(self.matched_word.len()),
            });
        }
    }

}

impl <'a> SyntaxHighlighter for SearchHighlighter<'a>{
    fn highlight(&mut self, line_idx: LineIdx, line: &Line) {
        let result = Vec:new();
        self.highlight_matched_words(line_idx, &mut result);
        if let Some(selected_match) = self.selected_match{
            if selected_match.line_idx == line_idx {
                self.highlight_selected_match(&mut result);
            }
        }
        self.highlights.insert(line_idx, result);
    }

    fn get_annotations(&self, line_idx: LineIdx) -> Option<&Vec<Annotation>>{
        self.highlights.get(&line_idx)
    }
}
