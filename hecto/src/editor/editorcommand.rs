use crossterm::event::{Event,KeyCode,KeyEvent,KeyModifiers};
use std::convert::TryFrom;
use super::terminal::Size;

pub enum Direction{
Up,
Down,
Left,
Right,
PageUp,
PageDown,
Home,
End
}

pub enum EditorCommand{
Move(Direction),
Resize(Size),
Quit
}

impl TryFrom<event> for EditorCommand{
type Error = String;

fn try_from(event: Event) -> Result<Self,Self::Error>
    {
    match evemt{
    Event::Key(KeyEvent{
            code,
            modifiers,
            ..
            }) => match(code,modifiers) {
                (KeyCode::Char('q'),KeyModifiers::CONTROL) =>
                    Ok(Self::Quit),
                (KeyCode::Up,_) =>
                    Ok(Self::Move(Direction::Up)),
                (KeyCode::Down,_) =>
                    Ok(Self::Move(Direction::Down)),
                (KeyCode::Left,_) =>
                    Ok(Self::Move(Direction::Left)),
                (KeyCode::Right,_)=>
                    Ok(Self::Move(Direction::Right)),
                (KeyCode::PageUp,_) =>
                    Ok(Self::Move(Direction::PageUp)),
                (KeyCode::PageDown,_) =>
                    Ok(Self::Move(Direction::PageDown)),
                (KeyCode::Home,_) =>
                    Ok(Self::Move(Direction::Home)),
                (KeyCode::End,_) =>
                    Ok(Self::Move(Direction::End)),
                _ =>
                    Err(format!("Key Code not Supported {code:?}")),
                },
                Event::Resize(width_u16,height_u16) =>{
                    #[allow(clippy::as_conversions)]
                    let height = height_u16 as usize;
                    #[allow(clippy::as_conversions)]
                    let width = width_u16 as usize;
                    Ok(Self::Resize(Size{height,width}))
                }
                _ => Err(format!("Event not Supported {event:?}")),
        }
    }
}
