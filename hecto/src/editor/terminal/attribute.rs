use crossterm::style::Color;

use crate::editor::annotatedstring::AnnotationType;

pub struct Attribute{
    pub foreground: Option<Color>,
    pub background: Option<Color>,
}

impl From<AnnotationType> for Attribute{
    fn from(annotation_type: AnnotationType) -> Self{
        match annotation_type{
            AnnotationType::Match => Self{
                foreground: Some(Color::Rgb{
                    r: 255,
                    g: 255,
                    b: 255,
                }),
                background: Some(Color::Rgb{
                    r: 110,
                    g: 110,
                    b: 110,
                })
            },
            AnnotationType::SelectedMatch => Self{
                foreground: Some(Color::Rgb{
                    r: 255,
                    g: 255,
                    b: 255,
                }),
                background: Some(Color::Rgb{
                    r: 245,
                    g: 246,
                    b: 0,
                }),
            },
            AnnotationType::Digit => Self{
                foreground: Some(Color::Rgb{
                    r: 240,
                    g: 0,
                    b: 0,
                }),
                background: Some(Color::Rgb{
                    r: 110,
                    g: 110,
                    b: 110,
                }),
            },
        }
    }
}
