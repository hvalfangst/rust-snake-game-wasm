use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};
use std::path::Path;


pub struct AudioManager {
    _stream: OutputStream,
    stream_handle: OutputStreamHandle,
    music_sink: Arc<Mutex<Sink>>,
    fx_sink: Arc<Mutex<Option<Sink>>>,
}

impl AudioManager {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Get the default audio device with full control
        let (_stream, stream_handle) = OutputStream::try_default()?;

        // Create dedicated sink for background music
        let music_sink = Arc::new(Mutex::new(Sink::try_new(&stream_handle)?));

        // FX sink will be created on-demand
        let fx_sink = Arc::new(Mutex::new(None));

        Ok(AudioManager {
            _stream,
            stream_handle,
            music_sink,
            fx_sink,
        })
    }

    /// Play background music on loop
    pub fn play_music<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let source = Decoder::new(BufReader::new(file))?;

        let music_sink = self.music_sink.lock().unwrap();

        // Stop any existing music
        music_sink.stop();

        // Play new music on repeat
        music_sink.append(source.repeat_infinite());
        music_sink.play();

        Ok(())
    }

    /// Stop background music
    pub fn stop_music(&self) {
        let music_sink = self.music_sink.lock().unwrap();
        music_sink.stop();
    }

    /// Pause/resume background music
    pub fn pause_music(&self) {
        let music_sink = self.music_sink.lock().unwrap();
        music_sink.pause();
    }

    pub fn resume_music(&self) {
        let music_sink = self.music_sink.lock().unwrap();
        music_sink.play();
    }

    /// Set music volume (0.0 to 1.0)
    pub fn set_music_volume(&self, volume: f32) {
        let music_sink = self.music_sink.lock().unwrap();
        music_sink.set_volume(volume.clamp(0.0, 1.0));
    }

    /// Play a sound effect (stops any currently playing FX)
    pub fn play_fx<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let source = Decoder::new(BufReader::new(file))?;

        let mut fx_sink_guard = self.fx_sink.lock().unwrap();

        // Stop and replace any existing FX
        if let Some(existing_sink) = fx_sink_guard.as_ref() {
            existing_sink.stop();
        }

        // Create new sink for this FX
        let new_sink = Sink::try_new(&self.stream_handle)?;
        new_sink.append(source);
        new_sink.play();

        *fx_sink_guard = Some(new_sink);

        Ok(())
    }

    /// Play FX with custom volume
    pub fn play_fx_with_volume<P: AsRef<Path>>(&self, path: P, volume: f32) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let source = Decoder::new(BufReader::new(file))?;

        let mut fx_sink_guard = self.fx_sink.lock().unwrap();

        // Stop and replace any existing FX
        if let Some(existing_sink) = fx_sink_guard.as_ref() {
            existing_sink.stop();
        }

        // Create new sink for this FX
        let new_sink = Sink::try_new(&self.stream_handle)?;
        new_sink.set_volume(volume.clamp(0.0, 1.0));
        new_sink.append(source);
        new_sink.play();

        *fx_sink_guard = Some(new_sink);

        Ok(())
    }

    /// Stop any currently playing sound effects
    pub fn stop_fx(&self) {
        let mut fx_sink_guard = self.fx_sink.lock().unwrap();
        if let Some(sink) = fx_sink_guard.as_ref() {
            sink.stop();
        }
        *fx_sink_guard = None;
    }

    /// Check if FX is currently playing
    pub fn is_fx_playing(&self) -> bool {
        let fx_sink_guard = self.fx_sink.lock().unwrap();
        match fx_sink_guard.as_ref() {
            Some(sink) => !sink.empty(),
            None => false,
        }
    }

    /// Check if music is currently playing
    pub fn is_music_playing(&self) -> bool {
        let music_sink = self.music_sink.lock().unwrap();
        !music_sink.empty()
    }

    /// Get the current music volume
    pub fn get_music_volume(&self) -> f32 {
        let music_sink = self.music_sink.lock().unwrap();
        music_sink.volume()
    }
}


// Example main function showing usage
// fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let audio_manager = AudioManager::new()?;

    // println!("Audio Manager initialized successfully!");

    // Example usage (uncomment and provide actual audio files to test):

    // Start background music
    // audio_manager.play_music("background_music.mp3")?;
    // audio_manager.set_music_volume(0.7);

    // Play some sound effects
    // thread::sleep(Duration::from_secs(2));
    // audio_manager.play_fx("sound1.wav")?;

    // thread::sleep(Duration::from_secs(1));
    // audio_manager.play_fx("sound2.wav")?; // This will stop sound1

    // thread::sleep(Duration::from_secs(2));
    // audio_manager.stop_fx();

    // Music continues playing in background...
    // thread::sleep(Duration::from_secs(5));

    // println!("Example completed. In a real application, keep the main thread alive or integrate with your event loop.");

    // Ok(())
// }