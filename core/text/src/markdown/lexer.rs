use nom::{IResult, InputLength, InputTake, InputIter, Slice};
use std::ops::{Range, RangeFrom, RangeTo, RangeFull};
use std::slice::Iter;
use std::iter::Enumerate;
use nom::multi::*;
use nom::branch::*;
use nom::combinator::*;
use nom::bytes::complete::*;
use nom::character::complete::*;
use nom::sequence::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum LexToken {
    ControlWordStarter,
    LBrace,
    RBrace,
    Space(String),
    Word(String),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Tokens<'a> {
    pub tok: &'a [LexToken],
    start: usize,
    end: usize,
}

impl<'a> Tokens<'a> {
    pub fn new(vec: &'a Vec<LexToken>) -> Tokens<'a> {
        Tokens {
            tok: &vec[..],
            start: 0,
            end: vec.len(),
        }
    }
}

impl<'a> InputLength for Tokens<'a> {
    fn input_len(&self) -> usize {
        self.tok.len()
    }
}

impl<'a> InputTake for Tokens<'a> {
    fn take(&self, count: usize) -> Self {
        Tokens {
            tok: &self.tok[..count],
            start: 0,
            end: count,
        }
    }

    fn take_split(&self, count: usize) -> (Self, Self) {
        let (prefix, suffix) = self.tok.split_at(count);
        let first = Tokens {
            tok: prefix,
            start: 0,
            end: prefix.len(),
        };
        let second = Tokens {
            tok: suffix,
            start: 0,
            end: suffix.len(),
        };

        (second, first)
    }
}

impl<'a> InputLength for LexToken {
    fn input_len(&self) -> usize {
        1
    }
}

impl<'a> Slice<Range<usize>> for Tokens<'a> {
    fn slice(&self, range: Range<usize>) -> Self {
        Tokens {
            tok: &self.tok[range.clone()],
            start: self.start + range.start,
            end: self.end + range.end,
        }
    }
}

impl<'a> Slice<RangeTo<usize>> for Tokens<'a> {
    fn slice(&self, range: RangeTo<usize>) -> Self {
        self.slice(0..range.end)
    }
}

impl<'a> Slice<RangeFrom<usize>> for Tokens<'a> {
    fn slice(&self, range: RangeFrom<usize>) -> Self {
        self.slice(range.start..self.end - self.start)
    }
}

impl<'a> Slice<RangeFull> for Tokens<'a> {
    fn slice(&self, _: RangeFull) -> Self {
        Tokens {
            tok: self.tok,
            start: self.start,
            end: self.end,
        }
    }
}

impl<'a> InputIter for Tokens<'a> {
    type Item = &'a LexToken;
    type Iter = Enumerate<Iter<'a, LexToken>>;
    type IterElem = Iter<'a, LexToken>;

    fn iter_indices(&self) -> Self::Iter {
        self.tok.iter().enumerate()
    }

    fn iter_elements(&self) -> Self::IterElem {
        self.tok.iter()
    }

    fn position<P>(&self, predicate: P) -> Option<usize> where
        P: Fn(Self::Item) -> bool {
        self.tok.iter().position(|b| predicate(b))
    }

    fn slice_index(&self, count: usize) -> Option<usize> {
        if self.tok.len() >= count {
            Some(count)
        } else {
            None
        }
    }
}


pub fn lex_control_word(input: &str) -> IResult<&str, LexToken> {
    map(tag("@"), |_| LexToken::ControlWordStarter)(input)
}

pub fn lex_spaces(input: &str) -> IResult<&str, LexToken> {
    map(space1, |s: &str| LexToken::Space(s.to_string()))(input)
}

pub fn lex_word(input: &str) -> IResult<&str, LexToken> {
    map(alphanumeric1, |s: &str| LexToken::Word(s.to_string()))(input)
}

pub fn lex_color_code(input: &str) -> IResult<&str, LexToken> {
    map(preceded(tag("#"), take(6usize)), |code| LexToken::Word(format!("#{}", code)))(input)
}

pub fn lex_brace(input: &str) -> IResult<&str, LexToken> {
    alt((
        map(tag("{"), |_| LexToken::LBrace),
        map(tag("}"), |_| LexToken::RBrace)
    ))(input)
}


pub fn lex_input(input: &str) -> IResult<&str, Vec<LexToken>> {
    many1(alt((
        lex_control_word,
        lex_brace,
        lex_word,
        lex_color_code,
        lex_spaces,
    )))(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input() {
        let input = "@red red text @bold {Some bold text too} more text @color #00FF00 green text";

        let expected = vec![
            LexToken::ControlWordStarter,
            LexToken::Word("red".to_string()),
            LexToken::Space(" ".to_string()),
            LexToken::Word("red".to_string()),
            LexToken::Space(" ".to_string()),
            LexToken::Word("text".to_string()),
            LexToken::Space(" ".to_string()),
            LexToken::ControlWordStarter,
            LexToken::Word("bold".to_string()),
            LexToken::Space(" ".to_string()),
            LexToken::LBrace,
            LexToken::Word("Some".to_string()),
            LexToken::Space(" ".to_string()),
            LexToken::Word("bold".to_string()),
            LexToken::Space(" ".to_string()),
            LexToken::Word("text".to_string()),
            LexToken::Space(" ".to_string()),
            LexToken::Word("too".to_string()),
            LexToken::RBrace,
            LexToken::Space(" ".to_string()),
            LexToken::Word("more".to_string()),
            LexToken::Space(" ".to_string()),
            LexToken::Word("text".to_string()),
            LexToken::Space(" ".to_string()),
            LexToken::ControlWordStarter,
            LexToken::Word("color".to_string()),
            LexToken::Space(" ".to_string()),
            LexToken::Word("#00FF00".to_string()),
            LexToken::Space(" ".to_string()),
            LexToken::Word("green".to_string()),
            LexToken::Space(" ".to_string()),
            LexToken::Word("text".to_string())
        ];

        let res = lex_input(input).unwrap();
        assert_eq!(res, ("", expected));
    }
}