//! File transfer functionality for PeakDrop

use anyhow::Result;
use std::path::Path;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader, BufWriter};
use tokio::net::{TcpListener, TcpStream};

use crate::protocol::{Message, TransferRequest, TransferResponse};
use crate::DEFAULT_PORT;

const CHUNK_SIZE: usize = 64 * 1024; // 64KB chunks

/// Send a file to a remote device
pub async fn send_file(target_addr: &str, file_path: &Path, sender_name: &str) -> Result<()> {
    let file = File::open(file_path).await?;
    let metadata = file.metadata().await?;
    let filename = file_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();

    // Connect to target
    let mut stream = TcpStream::connect(format!("{}:{}", target_addr, DEFAULT_PORT)).await?;

    // Send transfer request
    let request = TransferRequest::new(filename, metadata.len(), sender_name.to_string());
    let request_json = serde_json::to_string(&Message::TransferRequest(request.clone()))?;
    stream.write_all(request_json.as_bytes()).await?;
    stream.write_all(b"\n").await?;

    // Wait for response
    let mut buf = vec![0u8; 4096];
    let n = stream.read(&mut buf).await?;
    let response: Message = serde_json::from_slice(&buf[..n])?;

    match response {
        Message::TransferResponse(resp) if resp.accepted => {
            tracing::info!("Transfer accepted, sending file...");

            // Send file in chunks
            let mut reader = BufReader::new(file);
            let mut offset = 0u64;
            let mut chunk_buf = vec![0u8; CHUNK_SIZE];

            loop {
                let bytes_read = reader.read(&mut chunk_buf).await?;
                if bytes_read == 0 {
                    break;
                }

                let chunk = Message::FileChunk {
                    data: chunk_buf[..bytes_read].to_vec(),
                    offset,
                };
                let chunk_json = serde_json::to_string(&chunk)?;
                stream.write_all(chunk_json.as_bytes()).await?;
                stream.write_all(b"\n").await?;

                offset += bytes_read as u64;
                tracing::debug!("Sent {} / {} bytes", offset, metadata.len());
            }

            // Send completion
            let complete = Message::TransferComplete {
                hash: format!("{:x}", offset), // Simple hash for now
            };
            let complete_json = serde_json::to_string(&complete)?;
            stream.write_all(complete_json.as_bytes()).await?;

            tracing::info!("Transfer complete!");
            Ok(())
        }
        Message::TransferResponse(resp) => {
            anyhow::bail!("Transfer rejected: {}", resp.reason.unwrap_or_default())
        }
        _ => anyhow::bail!("Unexpected response"),
    }
}

/// Start listening for incoming file transfers
pub async fn receive_file(
    save_dir: &Path,
    _on_request: impl Fn(&TransferRequest) -> bool,
) -> Result<()> {
    let listener = TcpListener::bind(format!("0.0.0.0:{}", DEFAULT_PORT)).await?;
    tracing::info!("Listening for PeakDrop transfers on port {}", DEFAULT_PORT);

    loop {
        let (mut stream, addr) = listener.accept().await?;
        tracing::info!("Connection from {}", addr);

        let save_dir = save_dir.to_path_buf();

        tokio::spawn(async move {
            let mut buf = vec![0u8; 4096];

            // Read transfer request
            let n = stream.read(&mut buf).await?;
            let request: Message = serde_json::from_slice(&buf[..n])?;

            if let Message::TransferRequest(req) = request {
                // Check if we should accept
                let accepted = true; // For now, auto-accept

                let response = TransferResponse {
                    id: req.id.clone(),
                    accepted,
                    reason: None,
                };
                let response_json = serde_json::to_string(&Message::TransferResponse(response))?;
                stream.write_all(response_json.as_bytes()).await?;

                if accepted {
                    // Receive file
                    let file_path = save_dir.join(&req.filename);
                    let file = File::create(&file_path).await?;
                    let mut writer = BufWriter::new(file);

                    loop {
                        let n = stream.read(&mut buf).await?;
                        if n == 0 {
                            break;
                        }

                        let msg: Message = serde_json::from_slice(&buf[..n])?;
                        match msg {
                            Message::FileChunk { data, .. } => {
                                writer.write_all(&data).await?;
                            }
                            Message::TransferComplete { .. } => {
                                writer.flush().await?;
                                tracing::info!("Received file: {:?}", file_path);
                                break;
                            }
                            Message::Error { message } => {
                                tracing::error!("Transfer error: {}", message);
                                break;
                            }
                            _ => {}
                        }
                    }
                }
            }

            Ok::<_, anyhow::Error>(())
        });
    }
}
