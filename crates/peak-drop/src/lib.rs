//! PeakDrop - AirDrop-like file sharing for PeakOS
//!
//! This crate provides peer-to-peer file transfer capabilities
//! using mDNS for device discovery and TCP for file transfer.

mod discovery;
mod protocol;
mod transfer;

pub use discovery::{DeviceInfo, PeakDropService};
pub use protocol::{Message, TransferRequest, TransferResponse};
pub use transfer::{receive_file, send_file};

/// Default port for PeakDrop service
pub const DEFAULT_PORT: u16 = 17530;

/// mDNS service type
pub const SERVICE_TYPE: &str = "_peakdrop._tcp.local.";
