use crate::prelude::*;
use std::collections::HashMap;
use super::{Annotation, AnnotationType, Line, SyntaxHighlighter};
use unicode_segmentation::UnicodeSegmentation;

#[derive(Default)]
pub struct RustSyntaxHighlighter{
    highlights: HashMap<LineIdx, Vec<Annotation>>,
}



impl SyntaxHighlighter for RustSyntaxHighlighter{
    fn highlight(&mut self, line_idx: LineIdx, line: &Line){
        let mut result = Vec::new();
        for(start_idx, word) in line.split_word_bound_indices(){
            if is_valid_digit(word) {
                result.push(Annotation{
                    annotation_type: AnnotationType::Digit,
                    start: start_idx,
                    end: start_idx.saturating_add(word.len()),
                });
            }
        }
        self.highlights.insert(line_idx, result);
    }

    fn is_valid_digit(word: &str)  -> bool{
        if word.is_empty() {
            return false;
        }

        if is_numerical_lateral(word){
            return true;
        }

        let mut chars = word.chars();
        
        if let Some(first_char) = chars.next() && !first_char.is_ascii_digit(){
            return false;
        }

        let mut dot_detected = false;
        let mut e_detected = false;
        let mut prev_digit = false;

        for char in chars{
            match char{
                '0'..='9' => prev_digit = true,
                '_' => if !prev_digit {false} else {prev_digit = false;},
                '.' => {
                    if dot_detected || e_detected || !prev_digit {
                        return false;
                    }
                    dot_detected = true;
                    prev_digit = false;
                }
                'e' | 'E' => {
                    if e_detected || !prev_digit{
                        return false;
                    }
                    e_detected = true;
                    prev_digit = false;
                }
                _ => {
                    return false;
                }

            }
        }
        prev_digit    
    }

    fn is_numeric_literal(word: &str) -> bool{
        if word.len() <= 2 {
            return false;
        }
        let base = chars.next(){
            Some('b' | 'B') => 2,
            Some('o' | 'O') => 8,
            Some('x' | 'X') => 16,
            _ => return false;
        };
        chars.all(|char| char.is_digit(base))

    }

    fn get_annotations(&self, line_idx: LineIdx) -> Option<&Vex<Annotation>>{
        self.highlights.get(&line_idx)
    }
}
