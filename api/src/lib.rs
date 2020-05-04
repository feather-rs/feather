#[doc(hidden)]
pub mod alloc;
#[doc(hidden)]
pub mod log;
#[doc(hidden)]
pub mod vtable;
#[doc(hidden)]
pub extern crate fapi_common as common;
#[doc(hidden)]
pub extern crate log as log_crate;

mod startup;

pub use fapi_macros::plugin_init;
pub use startup::StartupState;
