use super::super::super::{Annotation, AnnotationType, FileType, Line};
use crate::prelude::*;
mod syntaxhighlighter;
mod searchhighlighter;
mod rustsyntaxhighlighter;

use searchhighlighter::SearchHighlighter;
use syntaxhighlighter::SyntaxHighlighter;
use rustsyntaxhighlighter::RustSyntaxHighlighter;

fn create_syntax_highlighter(file_type: FileType) -> Option<Box<dyn SyntaxHighlighter>> {
    match file_type{
        FileType::Rust => Some(Box::<RustSyntaxHighlighter>::default()),
        FileType::Text => None,
    }
}

#[derive(Default)]
pub struct Highlighter<'a>{
    syntax_highlighter: Option<Box<dyn SyntaxHighlighter>>,
    search_result_highlighter: Option<SearchHighlighter<'a>>,
}

impl<'a> Highlighter<'a>{
    
    pub fn new(matched_word: Option<&'a str>, selected_match: Option<Location>, file_type: FileType) -> Self{
        let search_result_highlighter = matched_word
            .map(|matched_word| SearchHighlighter::new(matched_word, selected_match));
        Self{
            syntax_highlighter: create_syntax_highlighter(file_type),
            search_result_highlighter,
        }
    }


    pub fn get_annotations(&self, line_idx: LineIdx) -> Vec<Annotation>{
        let mut result = Vec::new();

        if let Some(syntax_highlighter) = &self.syntax_highlighter &&
         let Some(annotations) = syntax_highlighter.get_annotations(line_idx){         
                result.extend(annotations.iter().copied());         
        }

        if let Some(search_result_highlighter) = &self.search_result_highlighter &&
        let Some(annotations) = search_result_highlighter.get_annotations(line_idx){    
                result.extend(annotations.iter().copied());            
        }
        result
    }

    pub fn highlight(&mut self, line_idx: LineIdx, line: &Line){
        if let Some(syntax_highlighter) = &mut self.syntax_highlighter{
            syntax_highlighter.highlight(line_idx, line);
        }
        if let Some(search_result_highlighter) = &mut self.search_result_highlighter{
            search_result_highlighter.highlight(line_idx, line);
        }
    }

}
