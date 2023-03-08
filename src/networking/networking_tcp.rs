/** Networking logic for a proxy-server client-host setup */

use std::net::{TcpStream, TcpListener};
use std::time::{Duration};

use serde::{Serialize, Deserialize};
use crate::gameplay::round_manager::RoundState;


/** The network configuration object

*Fields*
- `transfer_ms` (u64): the amount of transfer time we want to block for on requests
- `timeout_ms` (u64): the timeout amount for the tcp stream config
- `packet_ttl` (u32): the packet time-to-live
- `non_blocking` (bool): whether to use TCP blocking (true means no blocking)
- `nodelay` (bool): whether to use TCP nodelay (true means yes)
*/
#[derive(Debug, Serialize, Deserialize, Clone)]
struct NetworkConfig {
    transfer_ms: u64,
    timeout_ms: u64,
    packet_ttl: u32,
    non_blocking: bool,
    nodelay: bool
}

/**
TODO: Use cfg_attr to break up possible network config overrides using local files for non-web builds
    and URLS for web builds
*/

impl Default for NetworkConfig {
    /// The default network configuration.
    fn default() -> Self {
        Self {
            transfer_ms: 33,
            timeout_ms: 1000,
            packet_ttl: 60,
            non_blocking: false,
            nodelay: true,
        }
    }
}

impl NetworkConfig {
    #[allow(unused_must_use)]
    pub fn configure_stream(&self, stream: &mut TcpStream) {
        stream.set_nodelay(self.nodelay);
        let timeout = if self.timeout_ms > 0 { Some(Duration::from_millis(self.timeout_ms)) } else { None };
        stream.set_read_timeout(timeout);
        stream.set_write_timeout(timeout);
        stream.set_ttl(self.packet_ttl);
        stream.set_nonblocking(self.non_blocking);
    }
}

/** Main client entrypoint for non-web targets


*/
fn client_main() -> std::io::Result<()> {
    return Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
fn spawn_state_update_observer(state: &mut RoundState, server_address: &String, net: &NetworkConfig) -> std::io::Result<()> {
    let mut recv_stream = TcpStream::connect(format!("{}:9942", server_address))?; // Todo: Port?
    net.configure_stream(&mut recv_stream);

    // let ptr = state.get_ref();


    Ok(())
}

#[cfg(target_arch = "wasm32")]
fn spawn_state_update_observer_web(ptr: RoundStatePtr, ) -> std::io::Result<()> {
    Ok(())
}

// #[cfg(target_arch != "wasm32")]
// fn spawn_observer_thread() ->

#[cfg(test)]
mod tests {

    // Mock the tcp stream?

    #[test]
    fn test_configure_stream() {
        assert!(true);
    }
}