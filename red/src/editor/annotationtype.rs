#[derive(Copy,Clone,Eq,PartialEq,Debug)]
pub enum AnnotationType{
    Match,
    SelectedMatch,
    Digit,
    Keyword,
    KnownValue,
    Type,
    Char,
    LifetimeSpecefier,
    Comment,
    String,
}
