use crate::text::{Text, TextComponent, TextValue};
use std::borrow::Cow;

pub trait Simplification {
    fn simplify(&mut self);
}

impl Simplification for Text {
    fn simplify(&mut self) {
        match self {
            Text::String(_) => {},
            Text::Array(ref mut texts) => {
                texts.simplify();
                if let (Some(text), None) = (texts.pop(), texts.pop()) {
                    *self = text;
                }
            },
            Text::Component(ref mut component) => {
                component.simplify();
                if let Some(mut extra) = component.extra.take() {
                    extra.insert(0, Text::Component(component.clone()));
                    *self = Text::Array(extra);
                    self.simplify();
                } else {
                    if component.is_plain() {
                        match component.value {
                            TextValue::Text { ref mut text } => *self = Text::from(text.clone()),
                            _ => {},
                        };
                    }
                }
            }
        };
    }
}

impl Simplification for Vec<Text> {
    fn simplify(&mut self) {
        self.iter_mut().for_each(Text::simplify);
        
        let mut buf = String::new();
        let mut acc = Vec::with_capacity(self.len());
        self.reverse();
        while let Some(text) = self.pop() {
            match text {
                Text::String(s) => {
                    if let Cow::Owned(owned) = s {
                        buf = owned;
                    } else {
                        buf.push_str(&s);
                    }
                },
                _ => {
                    if !buf.is_empty() {
                        acc.push(Text::from(buf));
                        buf = String::new();
                    }
                    acc.push(text);
                }
            };
        }
        if !buf.is_empty() {
            acc.push(Text::from(buf));
        }
        *self = acc;
    }
}

impl Simplification for TextComponent {
    fn simplify(&mut self) {
        match self.value {
            TextValue::Translate { ref mut with, .. } => {
                with.iter_mut().for_each(Text::simplify);
            },
            _ => {}
        }
        if let Some(ref mut extra) = self.extra {
            extra.simplify()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::text::{Text, TextComponent, Simplification};
    #[test]
    fn simple_splification() {
        let mut a = Text::from("hello");
        let mut b = Text::from(TextComponent::from("hello"));
        a.simplify();
        b.simplify();
        assert_eq!(a, b);
    }

    #[test]
    fn array_simplification() {
        let mut a = Text::from("hello world");
        let mut b = Text::from("hello") + Text::from(" ") + Text::from("world");
        a.simplify();
        b.simplify();
        assert_eq!(a, b);
    }

    #[test]
    fn component_extra_simplification() {
        let mut a = Text::from(TextComponent::from("hello").extra(vec![Text::from(" "), Text::from("world")]));
        let mut b = Text::from("hello world");
        a.simplify();
        b.simplify();
        assert_eq!(a, b);
    }
}