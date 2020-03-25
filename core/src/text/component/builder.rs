use crate::text::{Click, Color, Entity, Hover, Reset, Style, Text, TextComponent};
use std::borrow::Cow;

impl TextComponent {
    pub fn set_style(mut self, style: Style, value: Option<bool>) -> Self {
        match style {
            Style::Bold => self.bold = value,
            Style::Italic => self.italic = value,
            Style::Obfuscated => self.obfuscated = value,
            Style::Strikethrough => self.strikethrough = value,
            Style::Underlined => self.underlined = value,
        };
        self
    }

    pub fn style(self, style: Style) -> Self {
        self.set_style(style, Some(true))
    }

    pub fn bold(self) -> Self {
        self.style(Style::Bold)
    }

    pub fn italic(self) -> Self {
        self.style(Style::Italic)
    }

    pub fn obfuscated(self) -> Self {
        self.style(Style::Obfuscated)
    }

    pub fn strikethrough(self) -> Self {
        self.style(Style::Strikethrough)
    }

    pub fn underlined(self) -> Self {
        self.style(Style::Underlined)
    }

    pub fn not_style(self, style: Style) -> Self {
        self.set_style(style, None)
    }

    pub fn not_bold(self) -> Self {
        self.style(Style::Bold)
    }

    pub fn not_italic(self) -> Self {
        self.style(Style::Italic)
    }

    pub fn not_obfuscated(self) -> Self {
        self.style(Style::Obfuscated)
    }

    pub fn not_strikethrough(self) -> Self {
        self.style(Style::Strikethrough)
    }

    pub fn not_underlined(self) -> Self {
        self.style(Style::Underlined)
    }

    /// Resets the given style; the given style will be inherited from parent's.
    pub fn reset_style(self, style: Style) -> Self {
        self.set_style(style, None)
    }

    pub fn reset_bold(self) -> Self {
        self.style(Style::Bold)
    }

    pub fn reset_italic(self) -> Self {
        self.style(Style::Italic)
    }

    pub fn reset_obfuscated(self) -> Self {
        self.style(Style::Obfuscated)
    }

    pub fn reset_strikethrough(self) -> Self {
        self.style(Style::Strikethrough)
    }

    pub fn reset_underlined(self) -> Self {
        self.style(Style::Underlined)
    }

    pub fn reset_style_all(mut self) -> Self {
        self.bold = None;
        self.italic = None;
        self.obfuscated = None;
        self.strikethrough = None;
        self.underlined = None;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    pub fn dark_red(self) -> Self {
        self.color(Color::DarkRed)
    }

    pub fn red(self) -> Self {
        self.color(Color::Red)
    }

    pub fn gold(self) -> Self {
        self.color(Color::Gold)
    }

    pub fn yellow(self) -> Self {
        self.color(Color::Yellow)
    }

    pub fn dark_green(self) -> Self {
        self.color(Color::DarkGreen)
    }

    pub fn green(self) -> Self {
        self.color(Color::Green)
    }

    pub fn aqua(self) -> Self {
        self.color(Color::Aqua)
    }

    pub fn dark_aqua(self) -> Self {
        self.color(Color::DarkAqua)
    }

    pub fn dark_blue(self) -> Self {
        self.color(Color::DarkBlue)
    }

    pub fn blue(self) -> Self {
        self.color(Color::Blue)
    }

    pub fn light_purple(self) -> Self {
        self.color(Color::LightPurple)
    }

    pub fn dark_purple(self) -> Self {
        self.color(Color::DarkPurple)
    }

    pub fn white(self) -> Self {
        self.color(Color::White)
    }

    pub fn gray(self) -> Self {
        self.color(Color::Gray)
    }

    pub fn dark_gray(self) -> Self {
        self.color(Color::DarkGray)
    }

    pub fn black(self) -> Self {
        self.color(Color::Black)
    }

    pub fn reset_color(mut self) -> Self {
        self.color = None;
        self
    }

    pub fn insertion<A: Into<Cow<'static, str>>>(mut self, insertion: A) -> Self {
        self.insertion = Some(insertion.into());
        self
    }

    pub fn reset_insertion(mut self) -> Self {
        self.insertion = None;
        self
    }

    pub fn on_click(mut self, click: Click) -> Self {
        self.click = Some(click);
        self
    }

    pub fn on_click_change_page(self, page: i32) -> Self {
        self.on_click(Click::ChangePage(page))
    }

    pub fn on_click_copy_to_clipboard<A: Into<Cow<'static, str>>>(self, to_copy: A) -> Self {
        self.on_click(Click::CopyToClipboard(to_copy.into()))
    }
    /// Can only be used on the client.
    pub fn on_click_open_file<A: Into<Cow<'static, str>>>(self, path: A) -> Self {
        self.on_click(Click::OpenFile(path.into()))
    }

    pub fn on_click_open_url<A: Into<Cow<'static, str>>>(self, url: A) -> Self {
        self.on_click(Click::OpenUrl(url.into()))
    }

    pub fn on_click_run_command<A: Into<Cow<'static, str>>>(self, command: A) -> Self {
        self.on_click(Click::RunCommand(command.into()))
    }
    /// Only useable for messages in chat.
    pub fn on_click_suggest_command<A: Into<Cow<'static, str>>>(self, command: A) -> Self {
        self.on_click(Click::SuggestCommand(command.into()))
    }

    pub fn reset_on_click(mut self) -> Self {
        self.click = None;
        self
    }

    pub fn on_hover(mut self, hover: Hover) -> Self {
        self.hover = Some(hover);
        self
    }

    pub fn on_hover_show_entity<A: Into<Entity>>(self, entity: A) -> Self {
        self.on_hover(Hover::ShowEntity(entity.into()))
    }

    pub fn on_hover_show_item(self, item: String) -> Self {
        self.on_hover(Hover::ShowItem(item))
    }

    pub fn on_hover_show_text<A: Into<Text>>(self, text: A) -> Self {
        self.on_hover(Hover::ShowText(Box::new(text.into())))
    }

    pub fn reset_on_hover(mut self) -> Self {
        self.hover = None;
        self
    }

    pub fn extra<A>(mut self, extra: A) -> Self
    where
        A: IntoIterator,
        A::Item: Into<Text>,
    {
        self.extra = Some(extra.into_iter().map(|e| e.into()).collect());
        self
    }

    pub fn push_extra<A: Into<Text>>(mut self, extra: A) -> Self {
        match self.extra {
            Some(ref mut extras) => extras.push(extra.into()),
            None => self.extra = Some(vec![extra.into()]),
        };
        self
    }

    pub fn reset_extra(mut self) -> Self {
        self.extra = None;
        self
    }

    pub fn reset_all(mut self) -> Self {
        self.color = None;
        self.bold = None;
        self.italic = None;
        self.underlined = None;
        self.strikethrough = None;
        self.obfuscated = None;
        self.insertion = None;
        self.click = None;
        self.hover = None;
        self.extra = None;
        self
    }

    pub fn reset(self, reset: Reset) -> Self {
        match reset {
            Reset::Color => self.reset_color(),
            Reset::Insertion => self.reset_insertion(),
            Reset::OnClick => self.reset_on_click(),
            Reset::OnHover => self.reset_on_hover(),
            Reset::Style => self.reset_style_all(),
        }
    }
}
