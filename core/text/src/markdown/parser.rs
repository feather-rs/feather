use super::*;
use crate::{Color, Style};
use nom::branch::alt;
use nom::bytes::complete::*;
use nom::combinator::*;
use nom::multi::*;
use nom::sequence::*;
use nom::IResult;

pub mod events;

use events::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Call(CallToken),
    Text(String),
}

#[derive(Debug, PartialEq, Clone)]
pub struct CallToken {
    pub ident: String,
    pub args: Option<Vec<String>>,
    pub body: Vec<Token>,
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

pub fn parse_arg(i: Tokens) -> IResult<Tokens, String> {
    let (i, tok) = opt(token(LexToken::ControlWordStarter))(i)?;
    let (i, next) = take(1usize)(i)?;

    match &next.tok[0] {
        LexToken::Word(s) => {
            if tok.is_some() {
                Ok((i, s.clone()))
            } else {
                // Argument is a colour code
                if s.starts_with("#") {
                    Ok((i, s.clone()))
                } else {
                    Err(nom::Err::Error((i, nom::error::ErrorKind::Tag)))
                }
            }
        }
        _ => Err(nom::Err::Error((i, nom::error::ErrorKind::Tag))),
    }
}

pub fn parse_control_word(i: Tokens) -> IResult<Tokens, Token> {
    let (i, _) = token(LexToken::ControlWordStarter)(i)?;
    let (i, next) = take(1usize)(i)?;

    match &next.tok[0] {
        LexToken::Word(s) => {
            let (i, arg) = opt(preceded(many0(space), parse_arg))(i)?;
            let (_, peeked) = peek(take(2usize))(i)?;

            map(consume_scope(has_lbrace(peeked)), move |body| {
                Token::Call(CallToken {
                    ident: s.clone(),
                    args: arg.clone().map(|arg| vec![arg]),
                    body
                })
            })(i)
        }
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
            vec![Token::Call(CallToken {
                ident: "red".to_string(),
                args: None,
                body: vec![Token::Text("some red text".to_string())]
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
                Token::Call(CallToken {
                    ident: "red".to_string(),
                    args: None,
                    body: vec![Token::Text("Delimited red text".to_string())]
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
            vec![Token::Call(CallToken {
                ident: "color".to_string(),
                args: Some(vec!["#00FF00".to_string()]),
                body: vec![
                    Token::Text("Some green text".to_string()),
                    Token::Call(CallToken {
                        ident: "bold".to_string(),
                        args: None,
                        body: vec![Token::Text("Green bold text".to_string())]
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
                Token::Call(CallToken {
                    ident: "red".to_string(),
                    args: None,
                    body: vec![
                        Token::Text("Some red text".to_string()),
                        Token::Call(CallToken {
                            ident: "bold".to_string(),
                            args: None,
                            body: vec![Token::Text("Some red bold text".to_string())]
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
            vec![Token::Call(CallToken {
                ident: "red".to_string(),
                args: None,
                body: vec![
                    Token::Text("Some red text".to_string()),
                    Token::Call(CallToken {
                        ident: "bold".to_string(),
                        args: None,
                        body: vec![Token::Text("Some red bold text".to_string())]
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
                Token::Call(CallToken {
                    ident: "red".to_string(),
                    args: None,
                    body: vec![
                        Token::Text("Some red text".to_string()),
                        Token::Call(CallToken {
                            ident: "bold".to_string(),
                            args: None,
                            body: vec![Token::Text("Some red bold text".to_string())]
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
            vec![Token::Call(CallToken {
                ident: "red".to_string(),
                args: None,
                body: vec![
                    Token::Text("Some red text".to_string()),
                    Token::Call(CallToken {
                        ident: "bold".to_string(),
                        args: None,
                        body: vec![Token::Text(
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
                Token::Call(CallToken {
                    ident: "on_hover".to_string(),
                    args: Some(vec!["show_text".to_string()]),
                    body: vec![Token::Call(CallToken {
                        ident: "green".to_string(),
                        args: None,
                        body: vec![Token::Text("Some green hover text".to_string())]
                    })]
                })
            ]
        )
    }
}
