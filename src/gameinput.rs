extern crate sdl2;
use crate::{gamecontext::GameContext, gamesound::GameSound};
use sdl2::{
    controller::Button, controller::GameController, keyboard::Keycode, GameControllerSubsystem,
};

pub enum InputType {
    Keyboard(Keycode),
    Controller(Button),
}
pub struct GameInput {}

impl GameInput {
    pub fn check_controllers(
        controller_subsystem: &GameControllerSubsystem,
    ) -> Option<GameController> {
        let num_controllers = controller_subsystem.num_joysticks().unwrap_or(0);

        // Open the first available controller if one is connected
        let controller: Option<GameController> = if num_controllers > 0 {
            match controller_subsystem.open(0) {
                Ok(ctrl) => Some(ctrl),
                Err(e) => {
                    eprintln!("Failed to open game controller: {}", e);
                    return None;
                }
            }
        } else {
            None
        };

        controller
    }

    fn handle_key(
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

    fn handle_controller(
        context: &mut GameContext,
        sound_context: &mut GameSound,
        button: Button,
    ) -> Result<(), String> {
        match button {
            Button::DPadUp => context.move_up(),
            Button::DPadDown => context.move_down(),
            Button::DPadRight => context.move_right(),
            Button::DPadLeft => context.move_left(),
            Button::Start => context.toggle_pause(),
            Button::Back => context.quit_game(),
            Button::A => context.new_game(),
            Button::B => context.toggle_fps(),
            Button::X => {
                if context.music {
                    sound_context.stop_sound("bgm");
                    context.music = false;
                } else {
                    sound_context.play_sound("bgm", -1)?;
                    context.music = true;
                }
            }
            _ => {}
        }
        Ok(())
    }

    pub fn handle_input(
        context: &mut GameContext,
        sound_context: &mut GameSound,
        input_type: InputType,
    ) -> Result<(), String> {
        match input_type {
            InputType::Keyboard(key) => Self::handle_key(context, sound_context, key)?,
            InputType::Controller(button) => {
                Self::handle_controller(context, sound_context, button)?
            }
        }

        Ok(())
    }
}
