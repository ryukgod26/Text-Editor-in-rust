use super ::{
    Size,Terminal,
    uicomponent::UIComponent,
};
use std::time::{Instant,Duration};

const DEFAULT_DISAPPEAR_DURATION: Duration = Duration::new(5,0);

struct Message{
    text: String,
    time: Instant,
}

#[derive(Default)]
pub struct MessageBar{
    current_message: Message,
    needs_redraw: bool,
    cleared_after_expiry: bool,
}

impl MessageBar{
    pub fn update_message(&mut self, new_message: &str){
        self.current_message = Message{
            text: new_message.to_string(),
            time: Instant::now(),
        };
        self.cleared_after_expiry = false;
        self.mark_redraw(true);
    }
}

impl UIComponent for MessageBar{
    fn mark_redraw(&mut self,val: bool){
        self.needs_redraw = val;
    }

    fn needs_redraw(&self) -> bool{
        self.needs_redraw
    }

    fn set_size(&mut self, _: Size) {}

    fn draw(&mut self, origin: usize) -> Result<(), std::io::Error> {
        if self.current_message.is_expired() {
            self.cleared_after_expiry = true; // Upon expiration, we need to write out "" once to clear the message. To avoid clearing more than necessary, we  keep track of the fact that we've already cleared the expired message once.
        }
        let message = if self.current_message.is_expired() {
            ""
        } else {
            &self.current_message.text
        };

        Terminal::print_row(origin, message)
    }
}

impl Message{
    fn is_expired(&self) -> bool{
        Instant::now().duration_since(self.time) > DEFAULT_DISAPPEAR_DURATION
    }
}

impl Default for Message{
    fn default() -> Self{
        Self{
            text: String::new(),
            time: Instant::now(),
        }
    }
}
