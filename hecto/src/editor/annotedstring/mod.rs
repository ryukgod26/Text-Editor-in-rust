use std::{
    cmp::{max,min},
    fmt::{self,Display},
};

pub mod annotationtype;
mod annotation;
mod annotatedstringpart;
mod annotatedstringiterator;

pub use annotationtype::AnnotationType;
use annotation::Annotation;
use annotatedstringpart::AnnotatedStringPart;
use annotatedstringiterator::AnnotatedStringIterator;

#[derive(Default,Debug)]
pub struct AnnotatedString{
    string: String,
    annotations: Vec<Annotation>,
}

impl AnnotatedString{
    pub fn from(string: &str) -> Self{
        Self{
            string: String::from(string),
            annotations: Vec::new(),
        }
    }

    pub fn add_annotation(&mut self, annotation_type: AnnotationType,
        start_byte_idx: usize, end_byte_idx: usize) {
        debug_assert!(start_byte_idx <= end_byte_idx);
        self.annotations.push(Annotation{
            annotation_type,
            start_byte_idx,
            end_byte_idx,
        });
    }

    pub fn replace(&mut self, start_byte_idx: usize, end_byte_idx: usize, new_string: &str){
        debug_assert!(start_byte_idx <= end_byte_idx);

        let end_byte_idx = min(end_byte_idx,self.string.len());
        if start_byte_idx > end_byte_idx{
            return;
        }

        self.string.replace_range(start_byte_idx..end_byte_idx, new_string);
        let replace_range_len = end_byte_idx.saturating_sub(start_byte_idx);
        let shortend = new_string.len() < replace_range_len;
        let len_diff = new_string.len().abs_diff(replace_range_len);

        if len_diff == 0{
            return;
        }

        self.annotation.iter_mut().for_each(|annotation| {
            annotation.start_byte_idx = if annotation.start_byte_idx >= annotation.end_byte_idx{
                if shortend{
                    annotation.start_byte_idx.saturating_sub(len_diff)
                }else{
                    annotation.start_byte_idx.saturating_add(len_diff)
                }
            } else if annotation.start_byte_idx >= start_byte_idx{
                if shortend{
                    max(start_byte_idx, annotation.start_byte_idx.saturating_sub(line_diff))
                }else{
                    min(end_byte_idx, annotation.start_byte_idx.saturating_add(line_diff))
                }
            }
            else{
                annotation.start_byte_idx
            };
            annotation.end_byte_idx = if annotation.end_byte_idx >= end_byte_idx{
                if shortend{
                    annotation.end_byte_idx.saturating_sub(line_diff)
                }
                else{
                    annotation.end_byte_idx.saturating_add(line_diff)
                }
            }else if annotation.end_byte_idx >= start_byte_idx {
                if shortend{
                    max(start_byte_idx,
                        annotation.end_byte_idx.saturating_sub(line_diff)
                    )
                } else{
                    min(end_byte_idx,
                        annotation.end_byte_idx.saturating_sub(line_diff)
                    )
                }
            } else{
                annotation.end_byte_idx
            }
        });

        self.annotations.retain(|annotation|{
        annotation.start_byte_idx < annotation.end_byte_idx  &&
            annotation.start_byte_idx < self.string.len()
        });
    }
}

impl Display for AnnoatedString{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result{
        write!(formatter,"{}",self.string)
    }
}

impl <'a> IntoIterator for &'a AnnotatedString{
    type Item = AnnotatedStringPart<'a>;
    type IntoIter = AnnotatatedStringIterator<'a>;
    fn into_iter(self) -> Self::IntoIter{
        AnnotatedStringIterator{
            annotated_string: self,
            current_idx: 0,
        }
    }
}
