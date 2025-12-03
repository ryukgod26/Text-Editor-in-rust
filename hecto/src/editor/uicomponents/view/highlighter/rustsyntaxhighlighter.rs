use crate::prelude::*;
use std::collections::HashMap;
use super::{Annotation, AnnotationType, Line, SyntaxHighlighter};
use unicode_segmentation::UnicodeSegmentation;

#[derive(Default)]
pub struct RustSyntaxHighlighter{
    highlights: HashMap<LineIdx, Vec<Annotation>>,
}

const KEYWORDS: [&str; 56 ]= [
    "abstract",
    "as",
    "async",
    "await",
    "become",
    "box",
    "break",
    "const",
    "continue",
    "crate",
    "do",
    "dyn",
    "else",
    "enum",
    "extern",
    "false",
    "final",
    "fn",
    "for",
    "gen",
    "if",
    "impl",
    "in",
    "let",
    "loop",
    "macro",
    "macro_rulws",
    "match",
    "mod",
    "move",
    "mut",
    "override",
    "priv",
    "pub",
    "raw",
    "ref",
    "return",
    "safe",
    "self",
    "Self",
    "static",
    "struct",
    "super",
    "trait",
    "true",
    "try",
    "type",
    "typeof",
    "unsafe",
    "union",
    "unsized",
    "use",
    "virtual",
    "where",
    "while",
    "yield",
];

const TYPES: [&str; 21] = [
    "i8",
    "i16",
    "i32",
    "i64",
    "i128",
    "isize",
    "u8",
    "u16",
    "u32",
    "u64",
    "u128",
    "usize",
    "f32",
    "f64",
    "bool",
    "char",
    "Option",
    "Result",
    "String",
    "str",
    "Vec",
    "HashMap",
];

const KNOWN_VALUES: [&str; 6] = [
    "Some",
    "None",
    "true",
    "false",
    "Ok",
    "Err",
];

impl SyntaxHighlighter for RustSyntaxHighlighter{
    fn highlight(&mut self, line_idx: LineIdx, line: &Line){
        let mut result = Vec::new();
        for(start_idx, word) in line.split_word_bound_indices(){
            let mut annotation_type = None;

            if is_valid_digit(word) {
                annotation_type = AnnotationType::Digit;
            }
            else if is_keyword(word) {
                annotation_type = AnnotationType::Keyword;
            }
            else if is_type(word) {
                annotation_type = AnnotationType::Type;
            }
            else if is_known_value(word) {
                annotation_type = AnnotationType::KnownValue;
            }
            if let Some(annotation_type) = annotation_type{
                result.push(Annotation{
                    annotation_type,
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

        if chars.next() != Some('0'){
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

    fn is_keyword(word: &str) -> bool{
        KEYWORDS.contain(&word);
    }

    fn is_type(word: &str) -> bool{
        TYPES.contain(&word);
    }

    fn is_special_value(word: &str) -> bool{
        SPECIAL_VALUES.contain(&word);
    }
}
