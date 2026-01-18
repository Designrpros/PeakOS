#[cfg(feature = "voice")]
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
#[cfg(feature = "voice")]
use std::sync::{Arc, Mutex};

#[cfg(feature = "voice")]
pub struct Recorder {
    _stream: cpal::Stream,
    data: Arc<Mutex<Vec<f32>>>,
}

#[cfg(feature = "voice")]
impl Recorder {
    pub fn start() -> anyhow::Result<Self> {
        let host = cpal::default_host();
        let device = host
            .default_input_device()
            .ok_or_else(|| anyhow::anyhow!("No input device found"))?;

        let config = device.default_input_config()?;
        let data = Arc::new(Mutex::new(Vec::new()));
        let data_clone = data.clone();

        let stream = match config.sample_format() {
            cpal::SampleFormat::F32 => device.build_input_stream(
                &config.into(),
                move |samples: &[f32], _| {
                    let mut data = data_clone.lock().unwrap();
                    data.extend_from_slice(samples);
                },
                |err| eprintln!("Audio record error: {}", err),
                None,
            )?,
            _ => return Err(anyhow::anyhow!("Unsupported sample format")),
        };

        stream
            .play()
            .map_err(|e| anyhow::anyhow!("Failed to play stream: {:?}", e))?;

        Ok(Self {
            _stream: stream,
            data,
        })
    }

    pub fn stop(self) -> Vec<f32> {
        let data = self.data.lock().unwrap();
        data.clone()
    }
}
