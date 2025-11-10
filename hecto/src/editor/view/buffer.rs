//#[derive(Default)]
pub struct Buffer{
    pub lines :Vec<String>
}

impl Default for Buffer{
    fn default() -> Self {
        Self {
            lines:vec!["Testing".to_string()]
        }
    }
}
