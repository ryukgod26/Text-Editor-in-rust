use crate:: prelude::*;
use super::{Annotation, AnnotationType, Line, SyntaxHighlighter};
use unicode_segmentation::UnicodeSegmentation;

#[derive(Default)]
pub struct CppSyntaxHighlighter{
    highlights: Vec<Vec<Annotation>>,
    ml_comments_num: usize,
    in_ml_string: bool,
}

const KEYWORDS: [&str;67] = [
"reflexpr",
"register",
"reinterpret_cast",
"requires",
"return",
"short",
"signed",
"sizeof",
"static",
"static_assert",
"static_cast",
"struct",
"switch",
"synchronized",
"template",
"this",
"thread_local",
"throw",
"true",
"try",
"typedef",
"typeid",
"typename",
"union",
"unsigned",
"using",
"virtual",
"void",
"volatile",
"wchar_t",
"while",
"xor",
"xor_eq",
"final",
"override",
"transaction_safe",
"transaction_safe_dynamic",
"import",
"module",
"pre",
"post",
"trivially_relocatable_if_eligible",
"replaceable_if_eligible",
"if",
"elif",
"else",
"endif",
"ifdef",
"ifndef",
"elifdef",
"elifndef",
"define",
"undef",
"include",
"embed",
"line",
"error",
"warning",
"pragma",
"defined",
"__has_include",
"__has_cpp_attribute",
"__has_embed",
"export",
"import",
"module",
"_Pragma"
];

const TYPES: [&str;28]  = [
    "bool",
    "char",
    "char16_t",
    "char32_t",
    "wchar_t",
    "short",
    "int",
    "long",
    "long long",
    "signed",
    "unsigned",
    "float",
    "double",
    "void",
    "size_t",
    "ptrdiff_t",
    "intptr_t",
    "uintptr_t",
    "std::string",
    "string",
    "std::vector",
    "vector",
    "std::map",
    "map",
    "std::set",
    "set",
    "std::optional",
    "optional",
];

const KNOWN_VALUES: [&str;14] = [
    "NULL",
    "nullptr",
    "false",
    "true",
    "EOF",
    "EXIT_SUCCESS",
    "EXIT_FAILURE",
    "M_PI",
    "M_E",
    "__FILE__",
    "__LINE__",
    "__func__",
    "std::nullopt",
    "std::string::npos",
];

impl CppSyntaxHighlighter{
    fn annotate_ml_comments(&mut self, string: &str) -> Option<Annotation>{
        let mut chars = string.char_indices().peekable();

        while let Some((_, char)) = chars.next() {
            if char == '/' {
                if let Some((_,'*')) = chars.peek(){
                    self.ml_comments_num = self.ml_comments_num.saturating_add(1);
                    chars.next();
                }
            } else if self.ml_comments_num == 0{
                return None;
            } else if char == '*'  && let Some((idx,'/')) = chars.peek() {
                    self.ml_comments_num = self.ml_comments_num.saturating_sub(1);
                    if self.ml_comments_num == 0{
                        return Some(Annotation{
                            annotation_type: AnnotationType::Comment,
                            start: 0,
                            end: idx.saturating_add(1),
                        });
                    }
                    chars.next();        
            }

        }
        (self.ml_comments_num > 0).then_some(Annotation{
            annotation_type: AnnotationType::Comment,
            start: 0,
            end: string.len(),
        })
    }

    fn annotate_string(&mut self, string: &str) -> Option<Annotation>{
        let mut chars = string.char_indices();
        while let Some((idx,char)) = chars.next(){
            if char == '\\' && self.in_ml_string{
                chars.next();
                continue;
            }
            if char == '"'{
                if self.in_ml_string{
                    self.in_ml_string = false;
                    return Some(Annotation{
                        annotation_type: AnnotationType::String,
                        start: 0,
                        end: idx.saturating_add(1),
                    });
                }
                self.in_ml_string = true;
            }
            if !self.in_ml_string{
                return None;
            }
        }
        self.in_ml_string.then_some(Annotation{
            annotation_type: AnnotationType::String,
            start: 0,
            end: string.len(),
        })
    }

    fn intial_annotation(&mut self, line: &Line) -> Option<Annotation>{
        if self.in_ml_string{
            self.annotate_string(line)
        } else if self.ml_comments_num > 0{
            self.annotate_ml_comments(line)
        }else{
            None
        }

    }
}

impl SyntaxHighlighter for CppSyntaxHighlighter{
    fn highlight(&mut self, line_idx: LineIdx, line: &Line) {
        debug_assert_eq!(line_idx,self.highlights.len());
        let mut result = Vec::new();
        let mut iterator = line.split_word_bound_indices().peekable();

        if let Some(annotation) = self.intial_annotation(line){
            result.push(annotation);

            while let Some(&(next_idx,_)) = iterator.peek(){
                if next_idx >= annotation.end{
                    break;
                }
                iterator.next();
            }
        }
        while let Some((start_idx, _)) = iterator.next(){
            let remainder = &line[start_idx..];
            if let Some(mut annotation) = self.annotate_ml_comments(remainder)
                .or_else(|| annotate_comment(remainder))
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
        self.highlights.push(result);
    }

    fn get_annotations(&self, line_idx: LineIdx) -> Option<&Vec<Annotation>> {
        self.highlights.get(line_idx)
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
                '_' => {
                if !prev_digit 
                    {return false;}  
                prev_digit = false;
                }
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

    pub fn is_keyword(word: &str) -> bool{
        KEYWORDS.contains(&word)
    }

    fn is_type(word: &str) -> bool{
        TYPES.contains(&word)
    }

    fn is_known_value(word: &str) -> bool{
        KNOWN_VALUES.contains(&word)
    }

    fn annotate_next_word<F>(string: &str, annotation_type: AnnotationType, validator: F) -> Option<Annotation>  where F: Fn(&str) -> bool {
       
            if let Some(word) = string.split_word_bounds().next() && validator(word) {
                    return Some(Annotation{
                        annotation_type,
                        start: 0,
                        end: word.len(),
                    });
            }
        
        None
    }

    fn annotate_number(string: &str)  -> Option<Annotation>{
        annotate_next_word(string, AnnotationType::Digit, is_valid_digit)
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
        if let Some((_,"\'")) = iter.next() && let Some((idx,next_word)) = iter.next(){
                return Some(Annotation{
                    annotation_type: AnnotationType::LifetimeSpecefier,
                    start: 0,
                    end: idx.saturating_add(next_word.len()),
                });
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