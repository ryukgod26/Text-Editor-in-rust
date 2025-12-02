use super::AnnotationType;


#[derive(Copy,Clone,Debug)]
#[allow(clippy::struct_field_names)]
pub struct Annotation{
    pub annotation_type: AnnotationType,
    pub start: usize,
    pub end: usize,
}
