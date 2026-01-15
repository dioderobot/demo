//! Startup sounds and status beeps
//!
//! Uses PWM to generate tones through the motor windings.

/// Sound frequencies (Hz)
pub mod frequencies {
    pub const C4: u32 = 262;
    pub const D4: u32 = 294;
    pub const E4: u32 = 330;
    pub const F4: u32 = 349;
    pub const G4: u32 = 392;
    pub const A4: u32 = 440;
    pub const B4: u32 = 494;
    pub const C5: u32 = 523;
}

/// Sound type
#[derive(Debug, Clone, Copy, PartialEq, Eq, defmt::Format)]
pub enum Sound {
    /// Startup sound
    Startup,
    /// Armed beep
    Armed,
    /// Direction change beep
    DirectionChange,
    /// Settings saved beep
    SettingsSaved,
    /// Error beep
    Error,
    /// Beacon (for finding lost model)
    Beacon(u8), // 1-5
    /// Cell count indication
    CellCount(u8),
    /// Custom tone
    Tone(u32, u16), // frequency, duration_ms
}

/// Sound player state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SoundState {
    Idle,
    Playing,
}

/// Sound player
pub struct SoundPlayer {
    state: SoundState,
    current_sound: Option<Sound>,
    /// Queue of sounds to play
    queue: heapless::Deque<Sound, 8>,
    /// Current note index in sequence
    note_index: usize,
    /// Remaining duration for current note
    remaining_ms: u16,
}

impl SoundPlayer {
    /// Create a new sound player
    pub fn new() -> Self {
        Self {
            state: SoundState::Idle,
            current_sound: None,
            queue: heapless::Deque::new(),
            note_index: 0,
            remaining_ms: 0,
        }
    }
    
    /// Queue a sound to play
    pub fn play(&mut self, sound: Sound) {
        let _ = self.queue.push_back(sound);
    }
    
    /// Check if currently playing
    pub fn is_playing(&self) -> bool {
        self.state == SoundState::Playing
    }
    
    /// Update the sound player (call periodically)
    /// 
    /// Returns the frequency to play (0 = silence)
    pub fn update(&mut self, elapsed_ms: u16) -> u32 {
        // Check if current note is done
        if self.remaining_ms > 0 {
            self.remaining_ms = self.remaining_ms.saturating_sub(elapsed_ms);
            if self.remaining_ms > 0 {
                return self.current_frequency();
            }
        }
        
        // Move to next note or sound
        if let Some(sound) = &self.current_sound {
            let sequence = Self::get_sequence(*sound);
            self.note_index += 1;
            
            if self.note_index < sequence.len() {
                let (freq, duration) = sequence[self.note_index];
                self.remaining_ms = duration;
                return freq;
            } else {
                // Sound complete
                self.current_sound = None;
                self.state = SoundState::Idle;
            }
        }
        
        // Start next sound from queue
        if let Some(sound) = self.queue.pop_front() {
            self.current_sound = Some(sound);
            self.note_index = 0;
            self.state = SoundState::Playing;
            
            let sequence = Self::get_sequence(sound);
            if !sequence.is_empty() {
                let (freq, duration) = sequence[0];
                self.remaining_ms = duration;
                return freq;
            }
        }
        
        0 // Silence
    }
    
    /// Get current frequency
    fn current_frequency(&self) -> u32 {
        if let Some(sound) = &self.current_sound {
            let sequence = Self::get_sequence(*sound);
            if self.note_index < sequence.len() {
                return sequence[self.note_index].0;
            }
        }
        0
    }
    
    /// Get note sequence for a sound
    fn get_sequence(sound: Sound) -> &'static [(u32, u16)] {
        use frequencies::*;
        
        match sound {
            Sound::Startup => &[
                (C4, 100), (0, 50),
                (E4, 100), (0, 50),
                (G4, 100), (0, 50),
                (C5, 200),
            ],
            Sound::Armed => &[
                (A4, 100), (0, 50),
                (A4, 100),
            ],
            Sound::DirectionChange => &[
                (G4, 150), (0, 50),
                (C5, 150),
            ],
            Sound::SettingsSaved => &[
                (C5, 100), (0, 50),
                (G4, 100), (0, 50),
                (C5, 200),
            ],
            Sound::Error => &[
                (C4, 200), (0, 100),
                (C4, 200), (0, 100),
                (C4, 200),
            ],
            Sound::Beacon(n) => {
                // Return different patterns based on beacon number
                match n {
                    1 => &[(A4, 500)],
                    2 => &[(A4, 250), (0, 100), (A4, 250)],
                    3 => &[(A4, 150), (0, 50), (A4, 150), (0, 50), (A4, 150)],
                    4 => &[(C5, 500)],
                    _ => &[(C5, 250), (0, 100), (C5, 250)],
                }
            }
            Sound::CellCount(cells) => {
                // This is a simplification - real implementation would
                // dynamically generate beeps based on cell count
                match cells {
                    1 => &[(A4, 200)],
                    2 => &[(A4, 200), (0, 200), (A4, 200)],
                    3 => &[(A4, 200), (0, 200), (A4, 200), (0, 200), (A4, 200)],
                    4 => &[(A4, 200), (0, 200), (A4, 200), (0, 200), (A4, 200), (0, 200), (A4, 200)],
                    _ => &[(A4, 200)],
                }
            }
            Sound::Tone(_, _) => {
                // Custom tones handled specially
                &[]
            }
        }
    }
    
    /// Clear all queued sounds
    pub fn clear(&mut self) {
        self.queue.clear();
        self.current_sound = None;
        self.state = SoundState::Idle;
        self.remaining_ms = 0;
    }
}

impl Default for SoundPlayer {
    fn default() -> Self {
        Self::new()
    }
}
