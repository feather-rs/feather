use std::fmt::{Result, Write};

use ansi_term::{Color, Style};

use crate::{Attributes, Content, Text};

impl<'a> Text<'a> {
    pub fn write_ansi<W>(&self, writer: &mut W) -> Result
    where
        W: Write,
    {
        // TODO: here we supply default values for `None` so should we maybe have an variant of Attributes that does not sue `Option`?
        for (content, attributes) in self.iter(&Attributes::default()) {
            let mut style = Style::default();

            style.is_bold = attributes.bold.unwrap_or_default();
            style.is_italic = attributes.italic.unwrap_or_default();
            style.is_strikethrough = attributes.strikethrough.unwrap_or_default();
            style.is_underline = attributes.underlined.unwrap_or_default();

            let color = {
                let (r, g, b) = attributes
                    .color
                    .unwrap_or(crate::Color::White)
                    .to_rgb()
                    .unwrap_or((0, 0, 0));
                Color::RGB(r, g, b)
            };

            let content = match content {
                Content::Text { text } => text,
                _ => unimplemented!(),
            };

            let content = style.fg(color).paint(content.as_ref());
            write!(writer, "{}", content)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use ansi_term::Color;

    use crate::Text;

    #[test]
    fn test() {
        let text = Text::of("red and bold").red().bold();
        let mut red_and_bold = String::new();
        text.write_ansi(&mut red_and_bold).unwrap();
        assert_eq!(
            Color::RGB(0xFF, 0x55, 0x55)
                .bold()
                .paint("red and bold")
                .to_string(),
            red_and_bold
        )
    }
}
