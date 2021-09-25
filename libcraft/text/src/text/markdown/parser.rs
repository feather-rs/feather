use super::*;
use nom::branch::alt;
use nom::bytes::complete::*;
use nom::combinator::*;
use nom::error::ErrorKind;
use nom::multi::*;
use nom::sequence::*;
use nom::{Err, IResult};

pub mod events;

#[derive(Debug, PartialEq, Clone)]
pub struct DynamicSpan {
    pub fragment: String,
    pub col: usize,
    pub line: usize,
}

impl DynamicSpan {
    pub fn new(fragment: String, col: usize, line: usize) -> DynamicSpan {
        DynamicSpan {
            fragment,
            col,
            line,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub span: DynamicSpan,
    pub tok: TokenType,
}

impl Token {
    pub fn new(span: DynamicSpan, tok: TokenType) -> Token {
        Token { span, tok }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Call(CallToken),
    Text(String),
}

#[derive(Debug, PartialEq, Clone)]
pub struct CallToken {
    pub ident: String,
    pub args: Option<Vec<String>>,
    pub body: Vec<Token>,
}

fn token(t: LexTokenType<'static>) -> impl Fn(Tokens) -> IResult<Tokens, LexToken> {
    move |input: Tokens| {
        let (rest, tok) = take(1usize)(input)?;
        if tok.tok[0].tok == t {
            Ok((rest, tok.tok[0].clone()))
        } else {
            Err(nom::Err::Error((rest, nom::error::ErrorKind::Tag)))
        }
    }
}

fn space(input: Tokens) -> IResult<Tokens, LexToken> {
    let (rest, tok) = take(1usize)(input)?;
    if let LexTokenType::Space(_) = tok.tok[0].tok {
        Ok((rest, tok.tok[0].clone()))
    } else {
        Err(nom::Err::Error((rest, nom::error::ErrorKind::Tag)))
    }
}

fn word_or_space(input: Tokens) -> IResult<Tokens, LexToken> {
    let (rest, tok) = take(1usize)(input)?;
    match &tok.tok[0].tok {
        LexTokenType::Word(_) | LexTokenType::Space(_) => Ok((rest, tok.tok[0].clone())),
        _ => Err(nom::Err::Error((rest, nom::error::ErrorKind::Tag))),
    }
}

pub fn tokens_to_text(tokens: Vec<LexToken>) -> Token {
    let mut s = String::new();

    for tok in &tokens {
        match &tok.tok {
            LexTokenType::Word(word) => s.push_str(word),
            LexTokenType::Space(space) => s.push_str(space),
            _ => unreachable!(),
        }
    }

    let trimmed = s.trim();
    if s.starts_with(' ') && !trimmed.is_empty() {
        let first_span = &tokens[1].span;
        let t = TokenType::Text(trimmed.to_string());
        let span = DynamicSpan::new(
            trimmed.to_string(),
            first_span.get_column(),
            first_span.location_line() as usize,
        );
        Token { span, tok: t }
    } else {
        let first_span = &tokens[0].span;
        let t = TokenType::Text(trimmed.to_string());
        let span = DynamicSpan::new(
            trimmed.to_string(),
            first_span.get_column(),
            first_span.location_line() as usize,
        );
        Token { span, tok: t }
    }
}

pub fn has_lbrace(input: Tokens) -> Option<usize> {
    for (i, tok) in input.tok.iter().enumerate() {
        if let LexTokenType::LBrace = tok.tok {
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
    let (i, tok) = opt(token(LexTokenType::ControlWordStarter))(i)?;
    let (i, next) = take(1usize)(i)?;

    match &next.tok[0].tok {
        LexTokenType::Word(s) => {
            if tok.is_some() {
                Ok((i, (*s).to_string()))
            } else {
                // Argument is a colour code
                if s.starts_with('#') {
                    Ok((i, (*s).to_string()))
                } else {
                    Err(nom::Err::Error((i, nom::error::ErrorKind::Tag)))
                }
            }
        }
        _ => Err(nom::Err::Error((i, nom::error::ErrorKind::Tag))),
    }
}

pub fn parse_control_word(i: Tokens) -> IResult<Tokens, Token> {
    let (i, tok) = token(LexTokenType::ControlWordStarter)(i)?;
    let (i, next) = take(1usize)(i)?;

    let span = &tok.span;

    match &next.tok[0].tok {
        LexTokenType::Word(s) => {
            let (i, arg) = opt(preceded(many0(space), parse_arg))(i)?;
            let (_, peeked) = peek(take(2usize))(i)?;

            let span = DynamicSpan::new(
                format!("{}{}", span.fragment(), next.tok[0].span.fragment()),
                span.get_column(),
                span.location_line() as usize,
            );

            map(consume_scope(has_lbrace(peeked)), move |body| {
                Token::new(
                    span.clone(),
                    TokenType::Call(CallToken {
                        ident: (*s).to_string(),
                        args: arg.clone().map(|arg| vec![arg]),
                        body,
                    }),
                )
            })(i)
        }
        _ => Err(Err::Error((i, ErrorKind::Tag))),
    }
}

pub fn parse_text(brace_delimited: bool) -> impl Fn(Tokens) -> IResult<Tokens, Token> {
    move |input: Tokens| {
        if brace_delimited {
            map(
                many_till(
                    word_or_space,
                    alt((
                        peek(token(LexTokenType::RBrace)),
                        peek(token(LexTokenType::ControlWordStarter)),
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
        .filter(|tok| match &tok.tok {
            TokenType::Text(s) => !s.is_empty(),
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
                    token(LexTokenType::RBrace),
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
    use crate::text::markdown::{lex_input, Tokens};

    #[test]
    fn test_basic() {
        let text = Span::new("@red some red text");
        let (_, lexed) = lex_input(text).unwrap();
        let (_, parsed) = parse_tokens(false)(Tokens::new(&lexed)).unwrap();

        assert_eq!(
            parsed,
            vec![Token::new(
                DynamicSpan::new("@red".to_string(), 1, 1),
                TokenType::Call(CallToken {
                    ident: "red".to_string(),
                    args: None,
                    body: vec![Token::new(
                        DynamicSpan::new("some red text".to_string(), 6, 1),
                        TokenType::Text("some red text".to_string())
                    )]
                })
            )]
        );

        let text = Span::new("some red text");
        let (_, lexed) = lex_input(text).unwrap();
        let (_, parsed) = parse_tokens(false)(Tokens::new(&lexed)).unwrap();

        assert_eq!(
            parsed,
            vec![Token::new(
                DynamicSpan::new("some red text".to_string(), 1, 1),
                TokenType::Text("some red text".to_string())
            )]
        );

        let text = Span::new("@red { Some red text }");
        let (_, lexed) = lex_input(text).unwrap();
        let (_, parsed) = parse_tokens(false)(Tokens::new(&lexed)).unwrap();

        assert_eq!(
            parsed,
            vec![Token::new(
                DynamicSpan::new("@red".to_string(), 1, 1),
                TokenType::Call(CallToken {
                    ident: "red".to_string(),
                    args: None,
                    body: vec![Token::new(
                        DynamicSpan::new("Some red text".to_string(), 8, 1),
                        TokenType::Text("Some red text".to_string())
                    )]
                })
            )]
        );
    }

    #[test]
    fn test_parse_delimited() {
        let text = Span::new("@red { Delimited red text } Not red text");
        let (_, lexed) = lex_input(text).unwrap();
        let (_, parsed) = parse_tokens(false)(Tokens::new(&lexed)).unwrap();

        assert_eq!(
            parsed,
            vec![
                Token::new(
                    DynamicSpan::new("@red".to_string(), 1, 1),
                    TokenType::Call(CallToken {
                        ident: "red".to_string(),
                        args: None,
                        body: vec![Token::new(
                            DynamicSpan::new("Delimited red text".to_string(), 8, 1),
                            TokenType::Text("Delimited red text".to_string())
                        )]
                    })
                ),
                Token::new(
                    DynamicSpan::new("Not red text".to_string(), 29, 1),
                    TokenType::Text("Not red text".to_string())
                )
            ]
        );
    }

    #[test]
    fn test_parse_multiarg() {
        let text = Span::new("@color #00FF00 { Some green text @bold { Green bold text } }");
        let (_, lexed) = lex_input(text).unwrap();
        let (_, parsed) = parse_tokens(false)(Tokens::new(&lexed)).unwrap();

        assert_eq!(
            parsed,
            vec![Token::new(
                DynamicSpan::new("@color".to_string(), 1, 1),
                TokenType::Call(CallToken {
                    ident: "color".to_string(),
                    args: Some(vec!["#00FF00".to_string()]),
                    body: vec![
                        Token::new(
                            DynamicSpan::new("Some green text".to_string(), 18, 1),
                            TokenType::Text("Some green text".to_string())
                        ),
                        Token::new(
                            DynamicSpan::new("@bold".to_string(), 34, 1),
                            TokenType::Call(CallToken {
                                ident: "bold".to_string(),
                                args: None,
                                body: vec![Token::new(
                                    DynamicSpan::new("Green bold text".to_string(), 42, 1),
                                    TokenType::Text("Green bold text".to_string())
                                )]
                            })
                        )
                    ]
                })
            )]
        );
    }

    #[test]
    fn test_parse_nested() {
        let text = Span::new(
            "@red { Some red text @bold { Some red bold text } more red text } Normal text",
        );
        let (_, lexed) = lex_input(text).unwrap();
        let (_, parsed) = parse_tokens(false)(Tokens::new(&lexed)).unwrap();

        assert_eq!(
            parsed,
            vec![
                Token::new(
                    DynamicSpan::new("@red".to_string(), 1, 1),
                    TokenType::Call(CallToken {
                        ident: "red".to_string(),
                        args: None,
                        body: vec![
                            Token::new(
                                DynamicSpan::new("Some red text".to_string(), 8, 1),
                                TokenType::Text("Some red text".to_string())
                            ),
                            Token::new(
                                DynamicSpan::new("@bold".to_string(), 22, 1),
                                TokenType::Call(CallToken {
                                    ident: "bold".to_string(),
                                    args: None,
                                    body: vec![Token::new(
                                        DynamicSpan::new("Some red bold text".to_string(), 30, 1),
                                        TokenType::Text("Some red bold text".to_string())
                                    )]
                                })
                            ),
                            Token::new(
                                DynamicSpan::new("more red text".to_string(), 51, 1),
                                TokenType::Text("more red text".to_string())
                            )
                        ]
                    })
                ),
                Token::new(
                    DynamicSpan::new("Normal text".to_string(), 67, 1),
                    TokenType::Text("Normal text".to_string())
                )
            ]
        );

        let text = Span::new("@red Some red text @bold { Some red bold text } more red text");
        let (_, lexed) = lex_input(text).unwrap();
        let (_, parsed) = parse_tokens(false)(Tokens::new(&lexed)).unwrap();

        assert_eq!(
            parsed,
            vec![Token::new(
                DynamicSpan::new("@red".to_string(), 1, 1),
                TokenType::Call(CallToken {
                    ident: "red".to_string(),
                    args: None,
                    body: vec![
                        Token::new(
                            DynamicSpan::new("Some red text".to_string(), 6, 1),
                            TokenType::Text("Some red text".to_string())
                        ),
                        Token::new(
                            DynamicSpan::new("@bold".to_string(), 20, 1),
                            TokenType::Call(CallToken {
                                ident: "bold".to_string(),
                                args: None,
                                body: vec![Token::new(
                                    DynamicSpan::new("Some red bold text".to_string(), 28, 1),
                                    TokenType::Text("Some red bold text".to_string())
                                )]
                            })
                        ),
                        Token::new(
                            DynamicSpan::new("more red text".to_string(), 49, 1),
                            TokenType::Text("more red text".to_string())
                        )
                    ]
                })
            )]
        );

        let text = Span::new("@red { Some red text @bold Some red bold text } more text");
        let (_, lexed) = lex_input(text).unwrap();
        let (_, parsed) = parse_tokens(false)(Tokens::new(&lexed)).unwrap();

        assert_eq!(
            parsed,
            vec![
                Token::new(
                    DynamicSpan::new("@red".to_string(), 1, 1),
                    TokenType::Call(CallToken {
                        ident: "red".to_string(),
                        args: None,
                        body: vec![
                            Token::new(
                                DynamicSpan::new("Some red text".to_string(), 8, 1),
                                TokenType::Text("Some red text".to_string())
                            ),
                            Token::new(
                                DynamicSpan::new("@bold".to_string(), 22, 1),
                                TokenType::Call(CallToken {
                                    ident: "bold".to_string(),
                                    args: None,
                                    body: vec![Token::new(
                                        DynamicSpan::new("Some red bold text".to_string(), 28, 1),
                                        TokenType::Text("Some red bold text".to_string())
                                    )]
                                })
                            ),
                        ]
                    })
                ),
                Token::new(
                    DynamicSpan::new("more text".to_string(), 49, 1),
                    TokenType::Text("more text".to_string())
                ),
            ]
        );

        let text = Span::new("@red Some red text @bold Some red bold text and more red bold text");
        let (_, lexed) = lex_input(text).unwrap();
        let (_, parsed) = parse_tokens(false)(Tokens::new(&lexed)).unwrap();

        assert_eq!(
            parsed,
            vec![Token::new(
                DynamicSpan::new("@red".to_string(), 1, 1),
                TokenType::Call(CallToken {
                    ident: "red".to_string(),
                    args: None,
                    body: vec![
                        Token::new(
                            DynamicSpan::new("Some red text".to_string(), 6, 1),
                            TokenType::Text("Some red text".to_string())
                        ),
                        Token::new(
                            DynamicSpan::new("@bold".to_string(), 20, 1),
                            TokenType::Call(CallToken {
                                ident: "bold".to_string(),
                                args: None,
                                body: vec![Token::new(
                                    DynamicSpan::new(
                                        "Some red bold text and more red bold text".to_string(),
                                        26,
                                        1
                                    ),
                                    TokenType::Text(
                                        "Some red bold text and more red bold text".to_string()
                                    )
                                )]
                            })
                        ),
                    ]
                })
            )]
        );
    }

    #[test]
    fn test_parse_event() {
        let text = Span::new("Some text @on_hover @show_text @green Some green hover text");

        let (_, lexed) = lex_input(text).unwrap();
        let (_, parsed) = parse_tokens(false)(Tokens::new(&lexed)).unwrap();

        assert_eq!(
            parsed,
            vec![
                Token::new(
                    DynamicSpan::new("Some text".to_string(), 1, 1),
                    TokenType::Text("Some text".to_string())
                ),
                Token::new(
                    DynamicSpan::new("@on_hover".to_string(), 11, 1),
                    TokenType::Call(CallToken {
                        ident: "on_hover".to_string(),
                        args: Some(vec!["show_text".to_string()]),
                        body: vec![Token::new(
                            DynamicSpan::new("@green".to_string(), 32, 1),
                            TokenType::Call(CallToken {
                                ident: "green".to_string(),
                                args: None,
                                body: vec![Token::new(
                                    DynamicSpan::new("Some green hover text".to_string(), 39, 1),
                                    TokenType::Text("Some green hover text".to_string())
                                )]
                            })
                        )]
                    })
                )
            ]
        )
    }
}
