use std::{
    cmp::{max,min},
    fmt::{self,Display},
};

mod annotatedstringpart;
mod annotatedstringiterator;

use annotatedstringpart::AnnotatedStringPart;
use annotatedstringiterator::AnnotatedStringIterator;

use super::{Annotation, AnnotationType};

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
        start: usize, end: usize) {
        debug_assert!(start <= end);
        self.annotations.push(Annotation{
            annotation_type,
            start,
            end,
        });
    }

    pub fn replace(&mut self, start: usize, end: usize, new_string: &str){
        debug_assert!(start <= end);

        let end = min(end,self.string.len());
        if start > end{
            return;
        }

        self.string.replace_range(start..end, new_string);
        let replace_range_len = end.saturating_sub(start);
        let shortend = new_string.len() < replace_range_len;
        let line_diff = new_string.len().abs_diff(replace_range_len);

        if line_diff == 0{
            return;
        }

        self.annotations.iter_mut().for_each(|annotation| {
            annotation.start = if annotation.start >= annotation.end{
                if shortend{
                    annotation.start.saturating_sub(line_diff)
                }else{
                    annotation.start.saturating_add(line_diff)
                }
            } else if annotation.start >= start{
                if shortend{
                    max(start, annotation.start.saturating_sub(line_diff))
                }else{
                    min(end, annotation.start.saturating_add(line_diff))
                }
            }
            else{
                annotation.start
            };
            annotation.end = if annotation.end >= end{
                if shortend{
                    annotation.end.saturating_sub(line_diff)
                }
                else{
                    annotation.end.saturating_add(line_diff)
                }
            }else if annotation.end >= start {
                if shortend{
                    max(start,
                        annotation.end.saturating_sub(line_diff)
                    )
                } else{
                    min(end,
                        annotation.end.saturating_sub(line_diff)
                    )
                }
            } else{
                annotation.end
            }
        });

        self.annotations.retain(|annotation|{
        annotation.start < annotation.end  &&
            annotation.start < self.string.len()
        });
    }
}

impl Display for AnnotatedString{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result{
        write!(formatter,"{}",self.string)
    }
}

impl<'a> IntoIterator for &'a AnnotatedString{
    type Item = AnnotatedStringPart<'a>;
    type IntoIter = AnnotatedStringIterator<'a>;

    fn into_iter(self) -> Self::IntoIter{
        AnnotatedStringIterator{
            annotated_string: self,
            current_idx: 0,
        }
    }
}
