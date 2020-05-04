use fapi_common::states::StartupState as Inner;
use std::ptr::NonNull;

/// Represents the state of the server during plugin initialization.
///
/// This value allows for registration of systems, event handlers,
/// and resources. Currently, it is only available at startup,
/// though this restriction may be relaxed.
pub struct StartupState(NonNull<Inner>);

#[doc(hidden)]
impl From<*mut Inner> for StartupState {
    fn from(inner: *mut Inner) -> Self {
        StartupState(NonNull::new(inner).expect("startup state was null pointer"))
    }
}
