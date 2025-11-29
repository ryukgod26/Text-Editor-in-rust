use crossterm::event::{ KeyCode::{
    self,Char
}, KeyEvent, KeyModifiers};


use super::super::Size;

#[derive(Copy,Clone)]
pub enum System{
    Save,
    Resize(Size),
    Quit,
    Dismiss,
    Find,
}


impl TryFrom<KeyEvent> for System{
    type Error = String;
    fn try_from(event: KeyEvent) -> Result<Self,Self::Error>{
        let KeyEvent{
            code,modifiers,..
        } = event;

        if modifiers == KeyModifiers::CONTROL{
            match code{
                Char('q') => Ok(Self::Quit),
                Char('s') => Ok(Self::Save),
                Char('f') => Ok(Self::Find),
                KeyCode::Esc => Ok(Self::Dismiss),
                _ => Err(format!("Unsupported Control + {code:?}")),
            }
        }else if modifiers == KeyModifiers::NONE && matches!(code,KeyCode::Esc){
            Ok(Self::Dismiss)
        }else{

            Err(format!("Unsupported Key Code {code:?} or modifier {modifiers:?}"))
        }
    }
}
