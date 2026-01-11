use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::io::Cursor;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::OnceLock;

// Embed the sound into the binary so it's portable
#[allow(dead_code)]
static CLICK_BYTES: &[u8] = include_bytes!("../assets/sounds/click.wav");

// Keep the handle globally accessible (it is Send + Sync)
static AUDIO_HANDLE: OnceLock<OutputStreamHandle> = OnceLock::new();
static MUSIC_SINK: OnceLock<Sink> = OnceLock::new();

// Global Volume State (f32 bits, default 1.0)
static GLOBAL_VOLUME: AtomicU32 = AtomicU32::new(1065353216);

// Must be called once. Returns the Stream which MUST be kept alive by the caller (App).
// Returns None if no audio device is available.
pub fn init() -> Option<OutputStream> {
    match OutputStream::try_default() {
        Ok((stream, handle)) => {
            let _ = AUDIO_HANDLE.set(handle);
            eprintln!("Audio: Initialized successfully");
            Some(stream)
        }
        Err(e) => {
            eprintln!(
                "Audio: No audio device available ({}), continuing without audio",
                e
            );
            None
        }
    }
}

pub fn set_volume(vol: f32) {
    GLOBAL_VOLUME.store(vol.to_bits(), Ordering::Relaxed);
}

pub fn get_volume() -> f32 {
    f32::from_bits(GLOBAL_VOLUME.load(Ordering::Relaxed))
}

#[allow(dead_code)]
pub fn play_click() {
    // Fire and forget sound
    std::thread::spawn(|| {
        if let Some(handle) = AUDIO_HANDLE.get() {
            if let Ok(sink) = Sink::try_new(handle) {
                if let Ok(source) = Decoder::new(Cursor::new(CLICK_BYTES)) {
                    sink.append(source);
                    let vol = get_volume();
                    sink.set_volume(vol * 0.4); // Scale down a bit as click is loud
                    sink.sleep_until_end();
                }
            }
        }
    });
}

pub fn play_track(path: String) {
    if let Some(handle) = AUDIO_HANDLE.get() {
        // Ensure sink exists
        let sink =
            MUSIC_SINK.get_or_init(|| Sink::try_new(handle).expect("Failed to create music sink"));

        // Stop current
        sink.stop();

        // Load new
        if let Ok(file) = std::fs::File::open(&path) {
            if let Ok(source) = Decoder::new(std::io::BufReader::new(file)) {
                sink.append(source);
                sink.set_volume(get_volume());
                sink.play();
                println!("Audio: Playing track -> {}", path);
            }
        }
    }
}

pub fn toggle_playback() {
    if let Some(sink) = MUSIC_SINK.get() {
        if sink.is_paused() {
            sink.play();
        } else {
            sink.pause();
        }
    }
}

#[allow(dead_code)]
pub fn stop_playback() {
    if let Some(sink) = MUSIC_SINK.get() {
        sink.stop();
    }
}
