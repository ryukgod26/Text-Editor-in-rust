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

const TYPES: [&str; 22] = [
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
        
        let mut iterator = line.split_word_bound_indices().peekable();
        while let Some((start_idx, _)) = iterator.next(){
            let remainder = &line[start_idx..];
            if let Some(mut annotation) = annotate_comment(remainder)
                .or_else(|| annotate_char(remainder))
                .or_else(|| annotate_number(remainder))
                .or_else(|| annotate_keyword(remainder))
                .or_else(|| annotate_type(remainder))
                .or_else(|| annotate_special_value(remainder))
                .or_else(|| annotate_lifetime_specefier(remainder))
            {
                annotation.shift(start_idx);
                result.push(annotation);
                while let Some(&(next_idx, _) )= iterator.peek() {
                    if next_idx >= annotation.end {
                        break;
                    }
                    iterator.next();
                }
            }
            
        }
        self.highlights.insert(line_idx, result);
    }

    fn get_annotations(&self, line_idx: LineIdx) -> Option<&Vec<Annotation>>{
        self.highlights.get(&line_idx)
    }
}

    fn is_valid_digit(word: &str)  -> bool{
        if word.is_empty() {
            return false;
        }

        if is_numeric_literal(word){
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
                '_' => if !prev_digit {return false;} else {prev_digit = false;},
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
        let mut chars = word.chars();
        if chars.next() != Some('0'){
            return false;
        }
        let base = match chars.next(){
            Some('b' | 'B') => 2,
            Some('o' | 'O') => 8,
            Some('x' | 'X') => 16,
            _ => return false,
        };
        chars.all(|char| char.is_digit(base))

    }

    

    fn is_keyword(word: &str) -> bool{
        KEYWORDS.contains(&word)
    }

    fn is_type(word: &str) -> bool{
        TYPES.contains(&word)
    }

    fn is_known_value(word: &str) -> bool{
        KNOWN_VALUES.contains(&word)
    }

    fn annotate_next_word<F>(string: &str, annotation_type: AnnotationType, validator: F) -> Option<Annotation>  where F: Fn(&str) -> bool {
       
            if let Some(word) = string.split_word_bounds().next(){
                if validator(word) {
                    return Some(Annotation{
                        annotation_type,
                        start: 0,
                        end: word.len(),
                    });
                }
            }
        
        None
    }

    fn annotate_number(string: &str)  -> Option<Annotation>{
        annotate_next_word(string, AnnotationType::Digit, is_numeric_literal)
    }

    fn annotate_type(string: &str) -> Option<Annotation> {
        annotate_next_word(string, AnnotationType::Type, is_type)
    }

    fn annotate_special_value(string: &str) -> Option<Annotation>{
        annotate_next_word(string, AnnotationType::KnownValue, is_known_value)
    }

    fn annotate_keyword(string: &str) -> Option<Annotation>{
        annotate_next_word(string, AnnotationType::Keyword, is_keyword)
    }


    fn annotate_char(string: &str) -> Option<Annotation>{
        let mut iter = string.split_word_bound_indices().peekable();

        if let Some((_, "\'")) = iter.next(){
            if let Some((_, "\\")) = iter.peek() {
                iter.next();
            }
            iter.next();
            if let Some((idx, "\'")) = iter.next(){
                return Some(Annotation{
                    annotation_type: AnnotationType::Char,
                    start: 0,
                    end: idx.saturating_add(1),
                });
            }
        }
        None
    }

    fn annotate_lifetime_specefier(string: &str) -> Option<Annotation>{
        let mut iter = string.split_word_bound_indices();
        if let Some((_,"\'")) = iter.next() {
            if let Some((idx,next_word)) = iter.next(){
                return Some(Annotation{
                    annotation_type: AnnotationType::LifetimeSpecefier,
                    start: 0,
                    end: idx.saturating_add(next_word.len()),
                });
            }
        }
        None
    }

    fn annotate_comment(string: &str) -> Option<Annotation>{
        if string.starts_with("//"){
            return Some(Annotation{
                annotation_type: AnnotationType::Comment,
                start: 0,
                end: string.len(),
            })
         }
         None
    }

