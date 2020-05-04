use fapi::{plugin_init, StartupState};

#[plugin_init(
    name = "HelloWorld",
    description = "A test plugin",
    author = "caelunshun"
)]
pub fn init(_state: &mut StartupState) {
    log::info!("Hello world!");
}
