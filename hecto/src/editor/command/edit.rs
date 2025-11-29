use crossterm::event::{KeyCode::{
    Backspace, Char, Delete,Enter, Tab
}
    , KeyEvent, KeyModifiers};

#[derive(Copy,Clone)]
pub enum Edit{
    Insert(char),
    Backspace,
    Delete,
    Enter,
}

impl TryFrom<KeyEvent> for Edit{
    type Error = String;
    fn try_from(event: KeyEvent) -> Result<Self,Self::Error>{
        match (event.code, event.modifiers) {
            (Char(character),KeyModifiers::NONE | KeyModifiers::SHIFT) =>{
                Ok(Self::Insert(character))
            },
            (Tab,KeyModifiers::NONE) => Ok(Self::Insert('\t')),
            (Enter,KeyModifiers::NONE) => Ok(Self::Enter),
            (Backspace,KeyModifiers::NONE) => Ok(Self::Backspace),
            (Delete,KeyModifiers::NONE) => Ok(Self::Delete),
        
            
            _ => Err(format!("Unsupported Key Code {:?} or modifier {:?}",event.code,event.modifiers)),
        }
    }
}

