use super::{events::*, lex_input, parse_tokens, Token, Tokens};
use crate::{Text, TextComponent, TextComponentBuilder};

//TODO: Convert to returning a nice Result type that isn't IResult
pub fn translate_text(text: &str) -> TextComponent {
    let (_, lexed) = lex_input(text).unwrap();
    let (_, parsed) = parse_tokens(false)(Tokens::new(&lexed)).unwrap();

    apply_tokens(parsed)
}

pub fn apply_tokens(tokens: Vec<Token>) -> TextComponent {
    let mut component = TextComponent::default();

    for token in tokens {
        match token {
            Token::Text(s) => component = component.push_extra(Text::of(s)),
            Token::Color(c) => {
                component = component.push_extra(apply_tokens(c.rest).color(c.color));
            }
            Token::Style(s) => {
                component = component.push_extra(apply_tokens(s.rest).style(s.style));
            }
            Token::Event(e) => match e.event_type {
                EventType::OnHover => match e.event_action {
                    EventAction::ShowText => {
                        component = component.on_hover_show_text(apply_tokens(e.body))
                    }
                    _ => todo!("Invalid branch for Hover."),
                },
                EventType::OnClick => todo!(),
            },
        }
    }

    component
}

#[cfg(test)]
mod tests {
    use super::translate_text;
    use crate::*;

    #[test]
    fn test_translate_simple() {
        let text = "@red Some red text";

        let component = translate_text(text);
        let s = serde_json::to_string_pretty(&component).unwrap();
        println!("{}", s);
        assert_eq!(
            component,
            TextComponent::default().push_extra(
                TextComponent::default()
                    .color(Color::Red)
                    .push_extra(Text::of("Some red text"))
            )
        );
    }

    #[test]
    fn test_component_with_event() {
        let text = "Some text @on_hover @show_text @green Some green hover text";

        let component = translate_text(text);
        let s = serde_json::to_string_pretty(&component).unwrap();
        println!("{}", s);
        assert_eq!(
            component,
            TextComponent::default()
                .push_extra(Text::of("Some text"))
                .on_hover_show_text(
                    TextComponent::default().push_extra(
                        TextComponent::default()
                            .color(Color::Green)
                            .push_extra(Text::of("Some green hover text"))
                    )
                )
        );
    }
}
