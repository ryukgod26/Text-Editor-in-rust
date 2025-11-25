use super::Size;

pub trait UIComponent{
    fn mark_redraw(&mut self,val: bool);
    fn needs_redraw(&self) -> bool;
    fn resize(&mut self, size: Size){
        self.set_size(size);
        self.mark_redraw(true);
    }
    fn set_size(&mut self, size: Size);
    fn render(&mut self, origin_row: usize){
        if self.needs_redraw(){
            match self.draw(origin_row) {
                Ok(()) => self.mark_redraw(false),
                Err(err) => {
                    #[cfg(debug_assertions)]
                    {
                        panic("Could Not Render Component: {err:?}");
                    }
                }
            }
        }
    }
    fn draw(&mut self, origin_row: usize) -> Result<(),std::io::Error>;
}
