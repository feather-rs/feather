use super::token;
use crate::markdown::{LexToken, Tokens};
use nom::bytes::complete::take;
use nom::combinator::map;
use nom::sequence::preceded;
use nom::IResult;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum EventType {
    OnHover,
    OnClick,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum EventAction {
    ShowText,
    OpenUrl,
    OpenFile,
    RunCommand,
    SuggestCommand,
    CopyToClipboard,
}

pub fn parse_event_type_word(i: &str) -> EventType {
    match i {
        "on_hover" => EventType::OnHover,
        "on_click" => EventType::OnClick,
        _ => todo!("Error branch"),
    }
}

pub fn parse_event_action_word(i: Tokens) -> IResult<Tokens, EventAction> {
    preceded(
        token(LexToken::ControlWordStarter),
        map(take(1usize), |tok: Tokens| match &tok.tok[0] {
            LexToken::Word(s) => match s.as_str() {
                "show_text" => EventAction::ShowText,
                "open_url" => EventAction::OpenUrl,
                "open_file" => EventAction::OpenFile,
                "run_command" => EventAction::RunCommand,
                "suggest_command" => EventAction::SuggestCommand,
                "copy_to_clipboard" => EventAction::CopyToClipboard,
                _ => todo!("Error branch"),
            },
            _ => todo!("Error branch"),
        }),
    )(i)
}
