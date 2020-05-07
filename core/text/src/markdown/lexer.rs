use nom::branch::*;
use nom::bytes::complete::*;
use nom::character::complete::*;
use nom::combinator::*;
use nom::error::VerboseError;
use nom::multi::*;
use nom::sequence::*;
use nom::{IResult, InputIter, InputLength, InputTake, Slice};
use nom_locate::*;
use std::iter::Enumerate;
use std::ops::{Range, RangeFrom, RangeFull, RangeTo};
use std::slice::Iter;

pub type Span<'a> = LocatedSpan<&'a str>;

#[derive(Debug, PartialEq, Clone)]
pub struct LexToken<'a> {
    pub tok: LexTokenType,
    pub span: Span<'a>,
}

impl<'a> LexToken<'a> {
    pub fn new(span: Span<'a>, tok: LexTokenType) -> LexToken<'a> {
        LexToken { tok, span }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum LexTokenType {
    ControlWordStarter,
    LBrace,
    RBrace,
    Space(String),
    Word(String),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Tokens<'a> {
    pub tok: &'a [LexToken<'a>],
    start: usize,
    end: usize,
}

impl<'a> Tokens<'a> {
    #[allow(clippy::ptr_arg)]
    pub fn new(vec: &'a Vec<LexToken<'a>>) -> Tokens<'a> {
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

impl<'a> InputLength for LexToken<'a> {
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
    type Item = &'a LexToken<'a>;
    type Iter = Enumerate<Iter<'a, LexToken<'a>>>;
    type IterElem = Iter<'a, LexToken<'a>>;

    fn iter_indices(&self) -> Self::Iter {
        self.tok.iter().enumerate()
    }

    fn iter_elements(&self) -> Self::IterElem {
        self.tok.iter()
    }

    fn position<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(Self::Item) -> bool,
    {
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

pub fn lex_control_word(input: Span) -> IResult<Span, LexToken, VerboseError<Span>> {
    map(tag("@"), |pos| {
        LexToken::new(pos, LexTokenType::ControlWordStarter)
    })(input)
}

pub fn lex_spaces(input: Span) -> IResult<Span, LexToken, VerboseError<Span>> {
    map(space1, |s: Span| {
        LexToken::new(s, LexTokenType::Space((*s.fragment()).to_string()))
    })(input)
}

pub fn valid_word(input: Span) -> IResult<Span, Span, VerboseError<Span>> {
    use nom::{AsChar, InputTakeAtPosition};
    input.split_at_position1_complete(
        |item| !item.is_alphanum() && item.as_char() != '_',
        nom::error::ErrorKind::AlphaNumeric,
    )
}

pub fn lex_word(input: Span) -> IResult<Span, LexToken, VerboseError<Span>> {
    map(valid_word, |s: Span| {
        LexToken::new(s, LexTokenType::Word((*s.fragment()).to_string()))
    })(input)
}

pub fn lex_color_code(input: Span) -> IResult<Span, LexToken, VerboseError<Span>> {
    map(preceded(peek(tag("#")), take(7usize)), |code: Span| {
        LexToken::new(code, LexTokenType::Word(code.to_string()))
    })(input)
}

pub fn lex_brace(input: Span) -> IResult<Span, LexToken, VerboseError<Span>> {
    alt((
        map(tag("{"), |pos| LexToken::new(pos, LexTokenType::LBrace)),
        map(tag("}"), |pos| LexToken::new(pos, LexTokenType::RBrace)),
    ))(input)
}

pub fn lex_input(input: Span) -> IResult<Span, Vec<LexToken>, VerboseError<Span>> {
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
        let input = Span::new(
            "@red red text @bold {Some bold text too} more text @color #00FF00 green text",
        );

        let expected = vec![
            LexTokenType::ControlWordStarter,
            LexTokenType::Word("red".to_string()),
            LexTokenType::Space(" ".to_string()),
            LexTokenType::Word("red".to_string()),
            LexTokenType::Space(" ".to_string()),
            LexTokenType::Word("text".to_string()),
            LexTokenType::Space(" ".to_string()),
            LexTokenType::ControlWordStarter,
            LexTokenType::Word("bold".to_string()),
            LexTokenType::Space(" ".to_string()),
            LexTokenType::LBrace,
            LexTokenType::Word("Some".to_string()),
            LexTokenType::Space(" ".to_string()),
            LexTokenType::Word("bold".to_string()),
            LexTokenType::Space(" ".to_string()),
            LexTokenType::Word("text".to_string()),
            LexTokenType::Space(" ".to_string()),
            LexTokenType::Word("too".to_string()),
            LexTokenType::RBrace,
            LexTokenType::Space(" ".to_string()),
            LexTokenType::Word("more".to_string()),
            LexTokenType::Space(" ".to_string()),
            LexTokenType::Word("text".to_string()),
            LexTokenType::Space(" ".to_string()),
            LexTokenType::ControlWordStarter,
            LexTokenType::Word("color".to_string()),
            LexTokenType::Space(" ".to_string()),
            LexTokenType::Word("#00FF00".to_string()),
            LexTokenType::Space(" ".to_string()),
            LexTokenType::Word("green".to_string()),
            LexTokenType::Space(" ".to_string()),
            LexTokenType::Word("text".to_string()),
        ];

        let (_, res) = lex_input(input).unwrap();
        assert_eq!(
            res.into_iter()
                .map(|tok| tok.tok)
                .collect::<Vec<LexTokenType>>(),
            expected
        );
    }
}
