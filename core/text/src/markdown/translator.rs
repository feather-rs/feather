use super::{events::*, lex_input, parse_tokens, Token, Tokens};
use crate::{Color, Style, Text, TextComponent, TextComponentBuilder};
use std::convert::TryFrom;

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
            Token::Call(call) => match (
                Color::try_from(call.ident.clone()),
                Style::try_from(call.ident.clone()),
                call.ident.as_str(),
            ) {
                (Ok(color), _, _) => {
                    component = component.push_extra(apply_tokens(call.body.clone()).color(color))
                }
                (_, Ok(style), _) => {
                    component = component.push_extra(apply_tokens(call.body.clone()).style(style))
                }
                (_, _, "color") => match call.args {
                    Some(v) => {
                        component = component.push_extra(
                            apply_tokens(call.body.clone()).color(Color::Custom(v[0].clone())),
                        )
                    }
                    None => todo!("Invalid token stream. Return Err eventually"),
                },
                (_, _, event_name) => {
                    let ty = parse_event_type_word(&event_name);
                    match call.args {
                        Some(v) => {
                            let action = parse_event_action_word(&v[0]);
                            match ty {
                                EventType::OnHover => match action {
                                    EventAction::ShowText => {
                                        component = component
                                            .on_hover_show_text(apply_tokens(call.body.clone()))
                                    }
                                    _ => todo!("Invalid event action for type OnHover"),
                                },
                                EventType::OnClick => todo!("OnClick unimplemented"),
                            }
                        }
                        None => todo!("Invalid token stream. Return Err eventually"),
                    }
                }
            },
        }
        //        match token {
        //            Token::Text(s) => component = component.push_extra(Text::of(s)),
        //            Token::Color(c) => {
        //                component = component.push_extra(apply_tokens(c.rest).color(c.color));
        //            }
        //            Token::Style(s) => {
        //                component = component.push_extra(apply_tokens(s.rest).style(s.style));
        //            }
        //            Token::Event(e) => match e.event_type {
        //                EventType::OnHover => match e.event_action {
        //                    EventAction::ShowText => {
        //                        component = component.on_hover_show_text(apply_tokens(e.body))
        //                    }
        //                    _ => todo!("Invalid branch for Hover."),
        //                },
        //                EventType::OnClick => todo!(),
        //            },
        //        }
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
