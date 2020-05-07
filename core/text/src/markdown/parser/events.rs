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

pub fn parse_event_action_word(i: &str) -> EventAction {
    match i {
        "show_text" => EventAction::ShowText,
        "open_url" => EventAction::OpenUrl,
        "open_file" => EventAction::OpenFile,
        "run_command" => EventAction::RunCommand,
        "suggest_command" => EventAction::SuggestCommand,
        "copy_to_clipboard" => EventAction::CopyToClipboard,
        _ => todo!("Error branch"),
    }
}
