use super::{events::*, lex_input, parse_tokens, Token, Tokens};
use crate::{Color, Style, Text, TextComponent, TextComponentBuilder};
use nom::error::{convert_error, ErrorKind, VerboseError};
use nom::Err;
use std::convert::TryFrom;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TextMarkupError<'a> {
    #[error("Incomplete input.")]
    Incomplete,
    #[error("Error while lexing data: {}", convert_error(.0, .1.clone()))]
    LexError(&'a str, VerboseError<&'a str>),
    #[error("Error while parsing data: {0:?}")]
    ParseError(ErrorKind),
    #[error("Error while evaluating data: {0}")]
    EvalError(&'a str),
}

impl<'a> From<(&'a str, Err<VerboseError<&'a str>>)> for TextMarkupError<'a> {
    fn from((i, e): (&'a str, Err<VerboseError<&'a str>>)) -> Self {
        match e {
            Err::Incomplete(_) => TextMarkupError::Incomplete,
            Err::Error(e) => TextMarkupError::LexError(i, e),
            Err::Failure(e) => TextMarkupError::LexError(i, e),
        }
    }
}

//TODO: Convert to returning a nice Result type that isn't IResult
pub fn translate_text(text: &str) -> Result<TextComponent, TextMarkupError> {
    let (_, lexed) = lex_input(text).map_err(|e| (text, e))?;

    match parse_tokens(false)(Tokens::new(&lexed)) {
        Ok((_, parsed)) => apply_tokens(parsed),
        Err(e) => match e {
            Err::Incomplete(_) => Err(TextMarkupError::Incomplete),
            Err::Error((_, e)) => Err(TextMarkupError::ParseError(e)),
            Err::Failure((_, e)) => Err(TextMarkupError::ParseError(e)),
        },
    }
}

pub fn apply_tokens(tokens: Vec<Token>) -> Result<TextComponent, TextMarkupError<'static>> {
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
                    component = component.push_extra(apply_tokens(call.body.clone())?.color(color))
                }
                (_, Ok(style), _) => {
                    component = component.push_extra(apply_tokens(call.body.clone())?.style(style))
                }
                (_, _, "color") => match call.args {
                    Some(v) => {
                        component = component.push_extra(
                            apply_tokens(call.body.clone())?.color(Color::Custom(v[0].clone())),
                        )
                    }
                    None => {
                        return Err(TextMarkupError::EvalError(
                            "@color call not provided with any arguments.",
                        ))
                    }
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
                                            .on_hover_show_text(apply_tokens(call.body.clone())?)
                                    }
                                    _ => return Err(TextMarkupError::EvalError("The only supported action type for @on_hover is @show_text."))
                                },
                                EventType::OnClick => return Err(TextMarkupError::EvalError("@on_click is unimplemented"))
                            }
                        }
                        None => {
                            return Err(TextMarkupError::EvalError(
                                "Text event not provided a target action.",
                            ))
                        }
                    }
                }
            },
        }
    }

    Ok(component)
}

#[cfg(test)]
mod tests {
    use super::translate_text;
    use crate::*;

    #[test]
    fn test_translate_simple() {
        let text = "@red Some red text";

        let component = translate_text(text).unwrap();
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

        let component = translate_text(text).unwrap();
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
