use crate::prelude::*;

#[derive(Component, Clone)]
#[allow(dead_code)]
pub enum StoryEvent {
    Clear,
    Append(Paragraph),
}

#[derive(Clone)]
pub struct Paragraph(Vec<Snippet>);

impl<T> From<Vec<(T, Character, FontStyle)>> for Paragraph
where
    T: ToString,
{
    fn from(value: Vec<(T, Character, FontStyle)>) -> Self {
        Self(
            value
                .into_iter()
                .map(|snip| snip.into())
                .collect::<Vec<Snippet>>(),
        )
    }
}
impl Paragraph {
    pub fn into_text_sections(self, fonts: &Res<FontAssets>) -> Vec<TextSection> {
        self.0
            .into_iter()
            .map(|snip| TextSection {
                value: snip.0,
                style: TextStyle {
                    font: snip.2.into_font(fonts),
                    font_size: 28.0,
                    color: snip.1.into(),
                },
            })
            .chain([TextSection {
                value: "\n\n".to_string(),
                style: TextStyle {
                    font: fonts.regular.clone(),
                    font_size: 18.0,
                    color: Color::WHITE,
                },
            }])
            .collect::<Vec<TextSection>>()
    }
}

#[derive(Clone)]
struct Snippet(String, Character, FontStyle);

impl<T> From<(T, Character, FontStyle)> for Snippet
where
    T: ToString,
{
    fn from(value: (T, Character, FontStyle)) -> Self {
        Self(value.0.to_string(), value.1, value.2)
    }
}
