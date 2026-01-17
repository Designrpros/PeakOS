pub mod battery;
pub mod hardware;
pub mod network;

pub trait SystemMonitor {
    fn update(&mut self);
}
