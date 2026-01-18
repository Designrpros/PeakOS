//! mDNS-based device discovery for PeakDrop

use anyhow::Result;
use mdns_sd::{ServiceDaemon, ServiceEvent, ServiceInfo};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{DEFAULT_PORT, SERVICE_TYPE};

/// Information about a discovered PeakDrop device
#[derive(Debug, Clone)]
pub struct DeviceInfo {
    /// Unique device ID
    pub id: String,
    /// Human-readable device name
    pub name: String,
    /// IP addresses
    pub addresses: Vec<std::net::IpAddr>,
    /// Port number
    pub port: u16,
}

/// PeakDrop service for discovery and advertising
pub struct PeakDropService {
    daemon: ServiceDaemon,
    devices: Arc<RwLock<HashMap<String, DeviceInfo>>>,
    device_name: String,
}

impl PeakDropService {
    /// Create a new PeakDrop service
    pub fn new(device_name: String) -> Result<Self> {
        let daemon = ServiceDaemon::new()?;
        Ok(Self {
            daemon,
            devices: Arc::new(RwLock::new(HashMap::new())),
            device_name,
        })
    }

    /// Start advertising this device on the network
    pub fn advertise(&self) -> Result<()> {
        let device_id = uuid::Uuid::new_v4().to_string();
        let mut properties = std::collections::HashMap::new();
        properties.insert("id".to_string(), device_id.clone());

        let service_info = ServiceInfo::new(
            SERVICE_TYPE,
            &self.device_name,
            &format!(
                "{}.local.",
                self.device_name.to_lowercase().replace(' ', "-")
            ),
            "",
            DEFAULT_PORT,
            properties,
        )?;

        self.daemon.register(service_info)?;
        tracing::info!("Advertising PeakDrop service as '{}'", self.device_name);
        Ok(())
    }

    /// Start browsing for nearby PeakDrop devices
    pub async fn browse(&self) -> Result<()> {
        let receiver = self.daemon.browse(SERVICE_TYPE)?;
        let devices = self.devices.clone();

        tokio::spawn(async move {
            while let Ok(event) = receiver.recv() {
                match event {
                    ServiceEvent::ServiceResolved(info) => {
                        let device = DeviceInfo {
                            id: info
                                .get_property_val_str("id")
                                .unwrap_or_default()
                                .to_string(),
                            name: info.get_fullname().to_string(),
                            addresses: info.get_addresses().iter().cloned().collect(),
                            port: info.get_port(),
                        };
                        tracing::info!("Discovered device: {}", device.name);
                        devices.write().await.insert(device.id.clone(), device);
                    }
                    ServiceEvent::ServiceRemoved(_, fullname) => {
                        tracing::info!("Device removed: {}", fullname);
                        devices.write().await.retain(|_, v| v.name != fullname);
                    }
                    _ => {}
                }
            }
        });

        Ok(())
    }

    /// Get list of currently discovered devices
    pub async fn get_devices(&self) -> Vec<DeviceInfo> {
        self.devices.read().await.values().cloned().collect()
    }

    /// Stop the service
    pub fn stop(&self) -> Result<()> {
        self.daemon.shutdown()?;
        Ok(())
    }
}
