use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};
use std::fs::File;
use std::io::{BufReader, Cursor};
use std::sync::{Arc, Mutex};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
use std::collections::HashMap;

pub struct AudioManager {
    _stream: OutputStream,
    stream_handle: OutputStreamHandle,
    music_sink: Arc<Mutex<Sink>>,
    fx_sink: Arc<Mutex<Option<Sink>>>,
    // State tracking to avoid expensive checks
    music_playing: Arc<Mutex<bool>>,
    last_music_check: Arc<Mutex<Instant>>,
    music_check_interval: Duration,
    // Audio caching to avoid repeated file I/O and decoding
    audio_cache: Arc<Mutex<HashMap<PathBuf, Vec<u8>>>>,
    current_music_path: Arc<Mutex<Option<PathBuf>>>,
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
            music_playing: Arc::new(Mutex::new(false)),
            last_music_check: Arc::new(Mutex::new(Instant::now())),
            music_check_interval: Duration::from_millis(500), // Check every 500ms instead of every frame
            audio_cache: Arc::new(Mutex::new(HashMap::new())),
            current_music_path: Arc::new(Mutex::new(None)),
        })
    }

    /// Play background music on loop (optimized with caching)
    pub fn play_music<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let path_buf = path.as_ref().to_path_buf();

        // Check if we're already playing this exact file
        {
            let current_music = self.current_music_path.lock().unwrap();
            if let Some(current_path) = current_music.as_ref() {
                if *current_path == path_buf && *self.music_playing.lock().unwrap() {
                    // Already playing the same file, no need to restart
                    println!("Music is already playing: {}", path_buf.display());
                    return Ok(());
                }
            }
        }

        // Check cache first
        let audio_data = {
            let mut cache = self.audio_cache.lock().unwrap();
            if let Some(cached_data) = cache.get(&path_buf) {
                println!("Using cached audio data for: {}", path_buf.display());
                cached_data.clone()
            } else {
                println!("Loading audio file: {}", path_buf.display());
                // Load and cache the file
                let mut file_data = Vec::new();
                let mut file = File::open(&path_buf)?;
                std::io::Read::read_to_end(&mut file, &mut file_data)?;
                cache.insert(path_buf.clone(), file_data.clone());
                file_data
            }
        };

        // Decode from memory (much faster than file I/O)
        let cursor = Cursor::new(audio_data);
        let source = Decoder::new(cursor)?;

        let music_sink = self.music_sink.lock().unwrap();

        // Stop any existing music
        music_sink.stop();

        // Play new music on repeat
        music_sink.append(source.repeat_infinite());
        music_sink.play();

        // Update our state tracking
        *self.music_playing.lock().unwrap() = true;
        *self.current_music_path.lock().unwrap() = Some(path_buf);

        Ok(())
    }

    /// Stop background music
    pub fn stop_music(&self) {
        let music_sink = self.music_sink.lock().unwrap();
        music_sink.stop();
        *self.music_playing.lock().unwrap() = false;
        *self.current_music_path.lock().unwrap() = None;
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

    /// Check if music is currently playing (rate-limited to avoid CPU spikes)
    pub fn is_music_playing(&self) -> bool {
        let now = Instant::now();
        let mut last_check = self.last_music_check.lock().unwrap();

        // Only do expensive check occasionally
        if now.duration_since(*last_check) >= self.music_check_interval {
            let music_sink = self.music_sink.lock().unwrap();
            let actually_playing = !music_sink.empty();
            *self.music_playing.lock().unwrap() = actually_playing;
            *last_check = now;
            actually_playing
        } else {
            // Return cached state for frequent calls
            *self.music_playing.lock().unwrap()
        }
    }

    /// Fast check without rate limiting (uses cached state)
    pub fn is_music_playing_cached(&self) -> bool {
        *self.music_playing.lock().unwrap()
    }

    /// Force an immediate check of music state (use sparingly)
    pub fn check_music_state_now(&self) -> bool {
        let music_sink = self.music_sink.lock().unwrap();
        let actually_playing = !music_sink.empty();
        *self.music_playing.lock().unwrap() = actually_playing;
        *self.last_music_check.lock().unwrap() = Instant::now();
        actually_playing
    }

    /// Set how often to check music state (default: 500ms)
    pub fn set_music_check_interval(&mut self, interval: Duration) {
        self.music_check_interval = interval;
    }

    /// Preload audio files into cache for instant playback
    pub fn preload_audio<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let path_buf = path.as_ref().to_path_buf();
        let mut cache = self.audio_cache.lock().unwrap();

        if !cache.contains_key(&path_buf) {
            let mut file_data = Vec::new();
            let mut file = File::open(&path_buf)?;
            std::io::Read::read_to_end(&mut file, &mut file_data)?;
            cache.insert(path_buf, file_data);
        }
        Ok(())
    }

    /// Clear audio cache to free memory
    pub fn clear_audio_cache(&self) {
        self.audio_cache.lock().unwrap().clear();
    }

    /// Get cache size in bytes
    pub fn get_cache_size(&self) -> usize {
        self.audio_cache.lock().unwrap()
            .values()
            .map(|data| data.len())
            .sum()
    }

    /// Get the current music volume
    pub fn get_music_volume(&self) -> f32 {
        let music_sink = self.music_sink.lock().unwrap();
        music_sink.volume()
    }
}
