use vane::Component;

#[derive(Debug)]
pub struct PluginMessageEvent {
    pub channel: String,
    pub data: Vec<u8>,
}

impl Component for PluginMessageEvent {}
