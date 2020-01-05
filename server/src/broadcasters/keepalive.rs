use crate::state::State;
use crate::{TickCount, TPS};
use feather_core::network::packet::implementation::KeepAliveClientbound;

/// Broadcasts keep alives every second.
#[system]
fn broadcast_keepalive(state: &State, tick: &TickCount) {
    if tick.0 % TPS == 0 {
        let packet = KeepAliveClientbound { keep_alive_id: 0 };
        state.broadcast_global(packet, None);
    }
}
