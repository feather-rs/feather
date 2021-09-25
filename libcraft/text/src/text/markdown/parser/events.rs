#[derive(Debug, Clone)]
pub enum EventParseError<'a> {
    InvalidEventType(&'a str),
    InvalidEventAction(&'a str),
}

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

pub fn parse_event_type_word(i: &str) -> Result<EventType, EventParseError> {
    match i {
        "on_hover" => Ok(EventType::OnHover),
        "on_click" => Ok(EventType::OnClick),
        _ => Err(EventParseError::InvalidEventType(i)),
    }
}

pub fn parse_event_action_word(i: &str) -> Result<EventAction, EventParseError> {
    match i {
        "show_text" => Ok(EventAction::ShowText),
        "open_url" => Ok(EventAction::OpenUrl),
        "open_file" => Ok(EventAction::OpenFile),
        "run_command" => Ok(EventAction::RunCommand),
        "suggest_command" => Ok(EventAction::SuggestCommand),
        "copy_to_clipboard" => Ok(EventAction::CopyToClipboard),
        _ => Err(EventParseError::InvalidEventAction(i)),
    }
}
