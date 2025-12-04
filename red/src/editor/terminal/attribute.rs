use crossterm::style::Color;

use super::super::AnnotationType;

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
            AnnotationType::Keyword => Self{
                foreground: Some(Color::Rgb{
                    r: 150,
                    g: 100,
                    b: 237,
                }),
                background: None,
            },
            AnnotationType::Type => Self{
                foreground: Some(Color::Rgb{
                    r: 150,
                    g: 250,
                    b: 139,
                }),
                background: None,
            },
            AnnotationType::KnownValue => Self{
                foreground: Some(Color::Rgb{
                    r: 185,
                    g: 160,
                    b: 150,
                }),
                background: None,
            },
            AnnotationType::Char => Self{
                foreground: Some(Color::Rgb{
                    r: 250,
                    g: 180,
                    b: 0,
                }),
                background: None,
            },
            AnnotationType::LifetimeSpecefier => Self{
                foreground: Some(Color::Rgb{
                    r: 102,
                    g: 205,
                    b: 170,
                }),
                background: None,
            },
            AnnotationType::Comment => Self{
                foreground: Some(Color::Rgb{
                    r: 35,
                    g: 200,
                    b: 60,
                }),
                background: None,
            },
            AnnotationType::String => Self{
                foreground: Some(Color::Rgb{
                    r: 250,
                    g: 180,
                    b: 102,
                }),
                background: None,
            }
        }
    }
}
