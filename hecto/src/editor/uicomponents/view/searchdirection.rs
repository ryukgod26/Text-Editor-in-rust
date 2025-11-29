#[derive(Eq,PartialEq,Default,Copy,Clone)]
pub enum SearchDirection{
    #[default]
    Forward,
    Backward,
}