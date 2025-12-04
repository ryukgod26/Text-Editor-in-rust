use crossterm::event::Event;
use std::convert::TryFrom;

mod movecommand;
mod edit;
mod system;

pub use movecommand::Move;
pub use edit::Edit;
pub use system::System;

use super::Size;

#[derive(Copy,Clone)]
pub enum Command{
    Move(Move),
    Edit(Edit),
    System(System),
}

#[allow(clippy::as_conversions)]
impl TryFrom<Event> for Command{
    type Error = String;
    fn try_from(event: Event) -> Result<Self,Self::Error>{
        match event{
            Event::Key(key_event) => Edit::try_from(key_event).map(Command::Edit)
                .or_else(|_| Move::try_from(key_event).map(Command::Move))
                .or_else(|_| System::try_from(key_event).map(Command::System))
                .map_err(|_err| format!("Event Not Supported {event:?}")),
            Event::Resize(width_u16,height_u16) => Ok(Self::System(System::Resize(
                        Size{
                            height: height_u16 as usize,
                            width: width_u16 as usize,
                        }))),
            _ => Err(format!("Event No5 Supported {event:?}")),

        }
    }
}