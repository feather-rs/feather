use super::*;
use crate::{Color, Style};
use nom::branch::alt;
use nom::bytes::complete::*;
use nom::combinator::*;
use nom::multi::*;
use nom::IResult;

#[derive(Debug, PartialEq)]
pub enum Token {
    Color(ColorToken),
    Style(StyleToken),
    Text(String),
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

pub fn has_lbrace(input: Tokens) -> isize {
    for (i, tok) in input.tok.iter().enumerate() {
        if let LexToken::LBrace = tok {
            return i as isize;
        }
    }

    -1isize
}

pub fn parse_control_word(i: Tokens) -> IResult<Tokens, Token> {
    let (i, _) = token(LexToken::ControlWordStarter)(i)?;
    let (i, next) = take(1usize)(i)?;

    match &next.tok[0] {
        LexToken::Word(s) => match s.as_str() {
            "red" => {
                let (_, peeked) = peek(take(2usize))(i)?;
                let idx = has_lbrace(peeked);
                if idx != -1 {
                    let (i, _) = take(idx as usize + 1)(i)?;
                    let (i, rest) = parse_tokens(true)(i)?;
                    Ok((
                        i,
                        Token::Color(ColorToken {
                            color: Color::Red,
                            rest,
                        }),
                    ))
                } else {
                    let (i, rest) = parse_tokens(false)(i)?;
                    Ok((
                        i,
                        Token::Color(ColorToken {
                            color: Color::Red,
                            rest,
                        }),
                    ))
                }
            }
            "bold" => {
                let (_, peeked) = peek(take(2usize))(i)?;
                let idx = has_lbrace(peeked);
                if idx != -1 {
                    let (i, _) = take(idx as usize + 1)(i)?;
                    let (i, rest) = parse_tokens(true)(i)?;
                    Ok((
                        i,
                        Token::Style(StyleToken {
                            style: Style::Bold,
                            rest,
                        }),
                    ))
                } else {
                    let (i, rest) = parse_tokens(false)(i)?;
                    Ok((
                        i,
                        Token::Style(StyleToken {
                            style: Style::Bold,
                            rest,
                        }),
                    ))
                }
            }
            "color" => {
                let (_i, next) = take(1usize)(i)?;
                match &next.tok[0] {
                    LexToken::Word(_cc) => todo!("ColorCode into Color"),
                    _ => panic!("Error branch"),
                }
            }
            _ => panic!("Error branch"),
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

pub fn parse_tokens(brace_delimited: bool) -> impl Fn(Tokens) -> IResult<Tokens, Vec<Token>> {
    move |input: Tokens| {
        if brace_delimited {
            map(
                many_till(
                    alt((parse_control_word, parse_text(brace_delimited))),
                    token(LexToken::RBrace),
                ),
                |(toks, _)| toks,
            )(input)
        } else {
            many1(alt((parse_control_word, parse_text(brace_delimited))))(input)
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
    }
}
