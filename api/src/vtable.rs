use crate::common::util::FStr;
use fapi_common::vtable::HostVTable;

static mut VTABLE: Option<HostVTable> = None;

/// Intializes the vtable from the given string.
///
/// # Safety
/// This function may not be called from multiple threads
/// at once.
pub fn init_vtable(s: FStr) -> Result<(), serde_json::Error> {
    let vtable = serde_json::from_slice(s.into())?;

    unsafe {
        VTABLE = Some(vtable);
    }

    Ok(())
}

/// Returns a reference to the host vtable.
///
/// The vtable must have already been initialized.
///
/// # Safety
/// This call must be made _after_ any calls to `init_vtable`.
pub fn vtable() -> &'static HostVTable {
    unsafe { VTABLE.as_ref().expect("vtable not initialized") }
}
