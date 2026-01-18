//! Protocol messages for PeakDrop file transfer

use serde::{Deserialize, Serialize};

/// A message in the PeakDrop protocol
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Message {
    /// Request to send a file
    TransferRequest(TransferRequest),
    /// Response to a transfer request
    TransferResponse(TransferResponse),
    /// File chunk during transfer
    FileChunk { data: Vec<u8>, offset: u64 },
    /// Transfer complete notification
    TransferComplete { hash: String },
    /// Error during transfer
    Error { message: String },
}

/// Request to transfer a file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferRequest {
    /// Unique ID for this transfer
    pub id: String,
    /// Name of the file
    pub filename: String,
    /// Size in bytes
    pub size: u64,
    /// MIME type (optional)
    pub mime_type: Option<String>,
    /// Sender's device name
    pub sender_name: String,
}

/// Response to a transfer request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferResponse {
    /// ID of the transfer request
    pub id: String,
    /// Whether the transfer was accepted
    pub accepted: bool,
    /// Reason for rejection (if any)
    pub reason: Option<String>,
}

impl TransferRequest {
    /// Create a new transfer request
    pub fn new(filename: String, size: u64, sender_name: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            filename,
            size,
            mime_type: None,
            sender_name,
        }
    }
}
