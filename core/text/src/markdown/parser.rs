use super::*;
use crate::{Color, Style};
use nom::branch::alt;
use nom::bytes::complete::*;
use nom::combinator::*;
use nom::multi::*;
use nom::sequence::*;
use nom::IResult;
use std::convert::TryFrom;

pub mod events;

use events::*;

#[derive(Debug, PartialEq)]
pub enum Token {
    Color(ColorToken),
    Style(StyleToken),
    Text(String),
    Event(EventToken),
}

#[derive(Debug, PartialEq)]
pub struct ColorToken {
    pub color: Color,
    pub rest: Vec<Token>,
}

#[derive(Debug, PartialEq)]
pub struct StyleToken {
    pub style: Style,
    pub rest: Vec<Token>,
}

#[derive(Debug, PartialEq)]
pub struct EventToken {
    pub event_type: EventType,
    pub event_action: EventAction,
    pub body: Vec<Token>,
}

fn token(t: LexToken) -> impl Fn(Tokens) -> IResult<Tokens, LexToken> {
    move |input: Tokens| {
        let (rest, tok) = take(1usize)(input)?;
        if tok.tok[0] == t {
            Ok((rest, tok.tok[0].clone()))
        } else {
            Err(nom::Err::Error((rest, nom::error::ErrorKind::Tag)))
        }
    }
}

fn space(input: Tokens) -> IResult<Tokens, LexToken> {
    let (rest, tok) = take(1usize)(input)?;
    if let LexToken::Space(_) = tok.tok[0] {
        Ok((rest, tok.tok[0].clone()))
    } else {
        Err(nom::Err::Error((rest, nom::error::ErrorKind::Tag)))
    }
}

fn word_or_space(input: Tokens) -> IResult<Tokens, LexToken> {
    let (rest, tok) = take(1usize)(input)?;
    match tok.tok[0] {
        LexToken::Word(_) | LexToken::Space(_) => Ok((rest, tok.tok[0].clone())),
        _ => Err(nom::Err::Error((rest, nom::error::ErrorKind::Tag))),
    }
}

pub fn tokens_to_text(tokens: Vec<LexToken>) -> Token {
    let mut s = String::new();

    for tok in tokens {
        match tok {
            LexToken::Word(word) => s.push_str(&word),
            LexToken::Space(space) => s.push_str(&space),
            _ => unreachable!(),
        }
    }

    Token::Text(s.trim().to_string())
}

pub fn has_lbrace(input: Tokens) -> Option<usize> {
    for (i, tok) in input.tok.iter().enumerate() {
        if let LexToken::LBrace = tok {
            return Some(i);
        }
    }

    None
}

fn consume_scope(lbrace_idx: Option<usize>) -> impl Fn(Tokens) -> IResult<Tokens, Vec<Token>> {
    move |input: Tokens| {
        if let Some(idx) = lbrace_idx {
            let (i, _) = take(idx + 1)(input)?;
            parse_tokens(true)(i)
        } else {
            parse_tokens(false)(input)
        }
    }
}

pub fn parse_control_word(i: Tokens) -> IResult<Tokens, Token> {
    let (i, _) = token(LexToken::ControlWordStarter)(i)?;
    let (i, next) = take(1usize)(i)?;
    let (_, peeked) = peek(take(2usize))(i)?;

    match &next.tok[0] {
        LexToken::Word(s) => match (
            Color::try_from(s.clone()),
            Style::try_from(s.clone()),
            s.as_str(),
        ) {
            (Ok(color), _, _) => map(consume_scope(has_lbrace(peeked)), move |rest| {
                Token::Color(ColorToken {
                    color: color.clone(),
                    rest,
                })
            })(i),
            (_, Ok(style), _) => map(consume_scope(has_lbrace(peeked)), move |rest| {
                Token::Style(StyleToken { style, rest })
            })(i),
            (_, _, "color") => {
                // let (i, next) = take(2usize)(i)?;
                let (i, next) = preceded(many0(space), take(1usize))(i)?;
                let (_, peeked) = peek(take(2usize))(i)?;
                match &next.tok[0] {
                    LexToken::Word(cc) => map(consume_scope(has_lbrace(peeked)), move |rest| {
                        Token::Color(ColorToken {
                            color: Color::Custom(cc.clone()),
                            rest,
                        })
                    })(i),
                    _ => panic!("Error branch"),
                }
            }
            _ => {
                let event_type = parse_event_type_word(&s);
                let (i, event_action) = preceded(many0(space), parse_event_action_word)(i)?;
                let (_, peeked) = peek(take(2usize))(i)?;
                map(consume_scope(has_lbrace(peeked)), move |body| {
                    Token::Event(EventToken {
                        event_type,
                        event_action,
                        body,
                    })
                })(i)
            }
        },
        _ => panic!("Error branch"),
    }
}

pub fn parse_text(brace_delimited: bool) -> impl Fn(Tokens) -> IResult<Tokens, Token> {
    move |input: Tokens| {
        if brace_delimited {
            map(
                many_till(
                    word_or_space,
                    alt((
                        peek(token(LexToken::RBrace)),
                        peek(token(LexToken::ControlWordStarter)),
                    )),
                ),
                |(toks, _)| tokens_to_text(toks),
            )(input)
        } else {
            map(many1(word_or_space), tokens_to_text)(input)
        }
    }
}

fn trim_empty_text(v: Vec<Token>) -> Vec<Token> {
    v.into_iter()
        .filter(|tok| match tok {
            Token::Text(s) => !s.is_empty(),
            _ => true,
        })
        .collect()
}

pub fn parse_tokens(brace_delimited: bool) -> impl Fn(Tokens) -> IResult<Tokens, Vec<Token>> {
    move |input: Tokens| {
        if brace_delimited {
            map(
                many_till(
                    alt((parse_control_word, parse_text(brace_delimited))),
                    token(LexToken::RBrace),
                ),
                |(toks, _)| trim_empty_text(toks),
            )(input)
        } else {
            map(
                many1(alt((parse_control_word, parse_text(brace_delimited)))),
                trim_empty_text,
            )(input)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::markdown::{lex_input, Tokens};

    #[test]
    fn test_basic() {
        let text = "@red some red text";
        let (_, lexed) = lex_input(text).unwrap();
        let (_, parsed) = parse_tokens(false)(Tokens::new(&lexed)).unwrap();

        assert_eq!(
            parsed,
            vec![Token::Color(ColorToken {
                color: Color::Red,
                rest: vec![Token::Text("some red text".to_string())]
            })]
        );

        let text = "some red text";
        let (_, lexed) = lex_input(text).unwrap();
        let (_, parsed) = parse_tokens(false)(Tokens::new(&lexed)).unwrap();

        assert_eq!(parsed, vec![Token::Text("some red text".to_string())]);
    }

    #[test]
    fn test_parse_delimited() {
        let text = "@red { Delimited red text } Not red text";
        let (_, lexed) = lex_input(text).unwrap();
        let (_, parsed) = parse_tokens(false)(Tokens::new(&lexed)).unwrap();

        assert_eq!(
            parsed,
            vec![
                Token::Color(ColorToken {
                    color: Color::Red,
                    rest: vec![Token::Text("Delimited red text".to_string())]
                }),
                Token::Text("Not red text".to_string())
            ]
        );
    }

    #[test]
    fn test_parse_multiarg() {
        let text = "@color #00FF00 { Some green text @bold { Green bold text } }";
        let (_, lexed) = lex_input(text).unwrap();
        let (_, parsed) = parse_tokens(false)(Tokens::new(&lexed)).unwrap();

        assert_eq!(
            parsed,
            vec![Token::Color(ColorToken {
                color: Color::Custom("#00FF00".to_string()),
                rest: vec![
                    Token::Text("Some green text".to_string()),
                    Token::Style(StyleToken {
                        style: Style::Bold,
                        rest: vec![Token::Text("Green bold text".to_string())]
                    })
                ]
            })]
        );
    }

    #[test]
    fn test_parse_nested() {
        let text = "@red { Some red text @bold { Some red bold text } more red text } Normal text";
        let (_, lexed) = lex_input(text).unwrap();
        let (_, parsed) = parse_tokens(false)(Tokens::new(&lexed)).unwrap();

        assert_eq!(
            parsed,
            vec![
                Token::Color(ColorToken {
                    color: Color::Red,
                    rest: vec![
                        Token::Text("Some red text".to_string()),
                        Token::Style(StyleToken {
                            style: Style::Bold,
                            rest: vec![Token::Text("Some red bold text".to_string())]
                        }),
                        Token::Text("more red text".to_string())
                    ]
                }),
                Token::Text("Normal text".to_string())
            ]
        );

        let text = "@red Some red text @bold { Some red bold text } more red text";
        let (_, lexed) = lex_input(text).unwrap();
        let (_, parsed) = parse_tokens(false)(Tokens::new(&lexed)).unwrap();

        assert_eq!(
            parsed,
            vec![Token::Color(ColorToken {
                color: Color::Red,
                rest: vec![
                    Token::Text("Some red text".to_string()),
                    Token::Style(StyleToken {
                        style: Style::Bold,
                        rest: vec![Token::Text("Some red bold text".to_string())]
                    }),
                    Token::Text("more red text".to_string())
                ]
            }),]
        );

        let text = "@red { Some red text @bold Some red bold text } more text";
        let (_, lexed) = lex_input(text).unwrap();
        let (_, parsed) = parse_tokens(false)(Tokens::new(&lexed)).unwrap();

        assert_eq!(
            parsed,
            vec![
                Token::Color(ColorToken {
                    color: Color::Red,
                    rest: vec![
                        Token::Text("Some red text".to_string()),
                        Token::Style(StyleToken {
                            style: Style::Bold,
                            rest: vec![Token::Text("Some red bold text".to_string())]
                        }),
                    ]
                }),
                Token::Text("more text".to_string()),
            ]
        );

        let text = "@red Some red text @bold Some red bold text and more red bold text";
        let (_, lexed) = lex_input(text).unwrap();
        let (_, parsed) = parse_tokens(false)(Tokens::new(&lexed)).unwrap();

        assert_eq!(
            parsed,
            vec![Token::Color(ColorToken {
                color: Color::Red,
                rest: vec![
                    Token::Text("Some red text".to_string()),
                    Token::Style(StyleToken {
                        style: Style::Bold,
                        rest: vec![Token::Text(
                            "Some red bold text and more red bold text".to_string()
                        )]
                    }),
                ]
            }),]
        );
    }

    #[test]
    fn test_parse_event() {
        use super::events::*;
        let text = "Some text @on_hover @show_text @green Some green hover text";

        let (_, lexed) = lex_input(text).unwrap();
        let (_, parsed) = parse_tokens(false)(Tokens::new(&lexed)).unwrap();

        assert_eq!(
            parsed,
            vec![
                Token::Text("Some text".to_string()),
                Token::Event(EventToken {
                    event_type: EventType::OnHover,
                    event_action: EventAction::ShowText,
                    body: vec![Token::Color(ColorToken {
                        color: Color::Green,
                        rest: vec![Token::Text("Some green hover text".to_string())]
                    })]
                })
            ]
        )
    }
}
