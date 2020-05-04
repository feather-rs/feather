use crate::{Color, Style};
use super::*;
use nom::IResult;
use nom::bytes::complete::*;

pub enum Token {
    Color(ColorToken),
    Style(StyleToken),
    Text(String)
}

pub struct ColorToken {
    color: Color,
    bounded_rest: Vec<Token>
}

pub struct StyleToken {
    style: Style,
    bounded_rest: Vec<Token>
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

pub fn parse_control_word(i: Tokens) -> IResult<Tokens, Token> {
    let (i, _) = token(LexToken::ControlWordStarter)(i)?;
    let (i, next) = take(1usize)(i)?;

    match &next.tok[0] {
        LexToken::Text(s) => match s.as_str() {
            "red" => Ok((i, Token::Color(ColorToken { color: Color::Red, bounded_rest: Vec::new() })));
            "bold" => Ok((i, Token::Style(StyleToken { style: Style::Bold, bounded_rest: Vec::new() })));
            "color" => {
                let (i, next) = take(1usize)(i)?;
                match next {
                    LexToken::ColorCode(cc) => {
                        todo!("ColorCode into Color")
                    }
                    _ => panic!("Error branch")
                }
            }
        }
        _ => panic!("Error branch")
    }
}