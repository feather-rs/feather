use crate::{Color, Style};
use super::*;
use nom::IResult;
use nom::combinator::*;
use nom::multi::*;
use nom::branch::alt;
use nom::bytes::complete::*;

#[derive(Debug, PartialEq)]
pub enum Token {
    Color(ColorToken),
    Style(StyleToken),
    Text(String)
}

#[derive(Debug, PartialEq)]
pub struct ColorToken {
    color: Color,
    rest: Vec<Token>
}

#[derive(Debug, PartialEq)]
pub struct StyleToken {
    style: Style,
    rest: Vec<Token>
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
        LexToken::Word(_) | LexToken::Space(_) => 
            Ok((rest, tok.tok[0].clone())),
        _ => Err(nom::Err::Error((rest, nom::error::ErrorKind::Tag)))
    }
}

pub fn tokens_to_text(tokens: Vec<LexToken>) -> Token {
    let mut s = String::new();

    for tok in tokens {
        match tok {
            LexToken::Word(word) => {
                s.push_str(&word)
            }
            LexToken::Space(space) => {
                s.push_str(&space)
            }
            _ => unreachable!()
        }
    }

    Token::Text(s)
}

pub fn has_rbrace(input: Tokens) -> bool {
    for tok in input.tok {
        if let LexToken::Space(_) = tok {
            return true
        }
    }

    false
}

pub fn parse_text(input: Tokens) -> IResult<Tokens, Token> {
//    if brace_delimited {
//        map(many_till(word_or_space, token(LexToken::RBrace)), |(toks, _)| tokens_to_text(toks))(input)
//    } else {
        map(many1(word_or_space), tokens_to_text)(input)
//    }
}

pub fn parse_control_word(i: Tokens) -> IResult<Tokens, Token> {
    let (i, _) = token(LexToken::ControlWordStarter)(i)?;
    let (i, next) = take(1usize)(i)?;

    match &next.tok[0] {
        LexToken::Word(s) => match s.as_str() {
            "red" => {
                // RBR|SPC,RBR (valid locations for RBrace)
                let (i, rest) = parse_tokens(i)?;
                Ok((i, Token::Color(ColorToken { color: Color::Red, rest })))
            }
            "bold" => Ok((i, Token::Style(StyleToken { style: Style::Bold, rest: Vec::new() }))),
            "color" => {
                let (i, next) = take(1usize)(i)?;
                match &next.tok[0] {
                    LexToken::Word(cc) => {
                        todo!("ColorCode into Color")
                    }
                    _ => panic!("Error branch")
                }
            }
            _ => panic!("Error branch")
        }
        _ => panic!("Error branch")
    }
}

pub fn parse_tokens(input: Tokens) -> IResult<Tokens, Vec<Token>> {
    many1(alt((
        parse_control_word,
        parse_text,
    )))(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::markdown::{lex_input, Tokens};

    #[test]
    fn test_basic() {
        let text = "@red some red text";
        let (_, lexed) = lex_input(text).unwrap();
        let (_, parsed) = parse_tokens(Tokens::new(&lexed)).unwrap();

        assert_eq!(parsed, vec![Token::Color(ColorToken { color: Color::Red, rest: vec![Token::Text(" some red text".to_string())] })]);

        let text = "some red text";
        let (_, lexed) = lex_input(text).unwrap();
        let (_, parsed) = parse_tokens(Tokens::new(&lexed)).unwrap();

        assert_eq!(parsed, vec![Token::Text("some red text".to_string())]);
    }
}