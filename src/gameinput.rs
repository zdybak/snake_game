extern crate sdl2;
use crate::{gamecontext::GameContext, gamesound::GameSound};
use sdl2::keyboard::Keycode;

pub struct GameInput {}

impl GameInput {
    pub fn handle_key(
        context: &mut GameContext,
        sound_context: &mut GameSound,
        keycode: Keycode,
    ) -> Result<(), String> {
        match keycode {
            Keycode::W => context.move_up(),
            Keycode::S => context.move_down(),
            Keycode::D => context.move_right(),
            Keycode::A => context.move_left(),
            Keycode::Space => context.toggle_pause(),
            Keycode::Escape => context.quit_game(),
            Keycode::N => context.new_game(),
            Keycode::P => context.toggle_fps(),
            Keycode::M => {
                if context.music {
                    sound_context.stop_sound("bgm");
                    context.music = false;
                } else {
                    sound_context.play_sound("bgm", -1)?;
                    context.music = true;
                }
            }
            _ => {}
        };
        Ok(())
    }
}
