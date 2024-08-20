extern crate sdl2;

use sdl2::mixer::{Channel, Chunk};
use std::collections::HashMap;
use std::path::Path;

pub struct GameSound {
    pub sounds: HashMap<String, Chunk>,
    pub channels: Vec<Option<String>>,
}

impl GameSound {
    pub fn new() -> Result<GameSound, String> {
        let sound_context = GameSound {
            sounds: HashMap::new(),
            channels: vec![None; 16],
        };

        Ok(sound_context)
    }

    pub fn load_sound(&mut self, name: &str, file_path: &str) -> Result<(), String> {
        let mut sound = Chunk::from_file(Path::new(file_path)).map_err(|e| e.to_string())?;
        if name == "bgm" {
            sound.set_volume(32);
        }
        self.sounds.insert(name.to_string(), sound);
        Ok(())
    }

    pub fn play_sound(&mut self, name: &str, loops: i32) -> Result<(), String> {
        if let Some(sound) = self.sounds.get(name) {
            let channel = Channel::all().play(sound, loops).unwrap();
            let channelnum = channel.0;
            //println!("playing {} on channel {}", name, channelnum);
            if channelnum >= 0 && channelnum < self.channels.len() as i32 {
                self.channels[channelnum as usize] = Some(name.to_string());
            }

            Ok(())
        } else {
            Err(format!("Sound {} not found", name))
        }
    }

    pub fn stop_sound(&mut self, name: &str) {
        if let Some(channel_idx) = self
            .channels
            .iter()
            .position(|s| s.as_deref() == Some(name))
        {
            //println!("attempting to halt {} on channel {}", name, channel_idx);
            Channel(channel_idx as i32).halt();
            self.channels[channel_idx] = None;
        }
    }
}
