use sonr::reactor::Reactor;
use sonr::errors::Result;
use sonr::net::tcp::{ReactiveTcpListener, TcpStream};

#[cfg(unix)]
use sonr::net::uds::{ReactiveUdsListener, UnixStream};

pub mod codecs;
pub mod connections;
pub mod timer;

// -----------------------------------------------------------------------------
// 		- Connections -
// -----------------------------------------------------------------------------
pub use connections::connection::Connection;
pub use connections::connections::Connections;

// -----------------------------------------------------------------------------
// 		- Codecs -
// -----------------------------------------------------------------------------
pub use codecs::{Codec, CodecError};
pub use codecs::line::LineCodec;

// -----------------------------------------------------------------------------
// 		- Aux functions -
// -----------------------------------------------------------------------------
/// Create a reactive tcp listener that skips the ip addr 
/// upon accept
pub fn tcp_listener(addr: &str) -> Result<impl Reactor<Output=TcpStream>> {
    let listener = ReactiveTcpListener::bind(addr)?;
    Ok(listener.map(|(s, _)| s))
}

#[cfg(unix)]
pub fn uds_listener(path: &str) -> Result<impl Reactor<Output=UnixStream>> {
    let listener = ReactiveUdsListener::bind(path)?;
    Ok(listener.map(|(s, _)| s))
}

// -----------------------------------------------------------------------------
// 		- Re-exports -
// -----------------------------------------------------------------------------
pub use num_cpus::get as num_cpus;
