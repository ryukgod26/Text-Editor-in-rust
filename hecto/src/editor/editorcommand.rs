use crossterm::event::{Event, KeyCode::{
    self,Backspace, Char, Delete, Dowm, Up, Right, Left, Enter, PageUp, PageDown, Home, End, Tab
}
    , KeyEvent, KeyModifiers};
use std::convert::TryFrom;

use super::Size;



#[derive(Copy,Clone)]
pub enum Move{
    PageUp,
    PageDown,
    Home,
    End,
    Up,
    Down,
    Right,
    Left,
}


#[derive((Copy,Clone)]
pub enum Edit{
    Insert(Char),
    Backspace,
    Delete,
    Enter,

}


#[derive((Copy,Clone)]
pub enum System{
    Save,
    Resize(Size),
    Quit,
    Dismiss,
}

#[derive(Copy,Clone)]
pub enum Command{
    Move(Move),
    Edit(Edit),
    System(System),
}

#[allow(clippy::as_conversions)]
impl TryFrom<Event> for Move {
    type Error = String;
    fn try_from(event: Event) -> Result<Self, Self::Error> {
        let KeyEvent {
            code, modifiers,..
        } = event;
        if modifiers == KeyModifiers::NONE {
            match code {
                Up => Ok(Self::Up),
                Down => Ok(Self::Down),
                Right => Ok(Self::Right),
                Left => Ok(Self::Left),
                PageUp => Ok(Self::PageUp),
                PageDown => Ok(Self::PageDown),
                Home => Ok(Self::Home),
                End => Ok(Self::End),
                _ => Err(format!("UnSupported code: {code:?}")),
            }
            } else {
                Err(format!("UnSupported Key Code {code:?} or Modifier {modifier:?}"))
        }
            Event::Resize(width_u16, height_u16) => {
 
                #[allow(clippy::as_conversions)]
                let height = height_u16 as usize;

                #[allow(clippy::as_conversions)]
                let width = width_u16 as usize;
                Ok(Self::Resize(Size { height, width }))
            }
            _ => Err(format!("Event not supported: {event:?}")),
        }
    }

impl TryFrom<Event> for Edit{
    type Error = String;
    fn try_from(event: Event) -> Result<Self,Self::Error>{
        match (event.code, event.modifiers) {
            (Char(character),KeyModifiers::NONE | KeyModifiers::SHIFT) =>{
                Ok(Self::Insert(character))
            },
            (Tab,KeyModifiers::NONE) => Ok(Self::Insert('\t')),
            (Enter,KeyModifiers::NONE) => Ok(Self::Enter),
            (Backspace,KeyModifiers::NONE) => Ok(Self::Backspace),
            (Delete,KeyModifiers::NONE) => Ok(Self::Delete),
        
            
            _ => Err(format!("Unsupported Key Code {event.code:?} or modifier {event.modifier:?}")),
        }
    }
}

impl TryFrom<Event> for System{
    type Error = String;
    fn try_from(event: Event) -> Result<Self,Self::Error>{
        let KeyEvent{
            code,modifiers,..
        } = event;

        if modifiers == KeyModifiers::CONTROL{
            match code{
                Char('q') => Ok(Self::Quit),
                Char('s') => Ok(Self::Save),
                _ => Err(format!("Unsupported Control + {code:?}")),
            }
        }else if modifiers == KeyModifiers::NONE && matches(code,KeyCode::Esc){
            Ok(Self::Dismiss)
        }else{

            Err(format!("Unsupported Key Code {event.code:?} or modifier {event    .modifier:?}"))
        }
    }
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
