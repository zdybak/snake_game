extern crate sdl2;
mod gamecontext;
mod gamerenderer;
mod gamesettings;
mod gamesound;
mod point;

use gamecontext::SoundEffect;
use gamesettings::{DOT_SIZE_IN_PXS, GRID_X_SIZE, GRID_Y_SIZE};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mixer::{InitFlag, AUDIO_S16LSB, DEFAULT_CHANNELS};
use std::time::Instant;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _audio_subsystem = sdl_context.audio()?;

    let frequency = 44_100;
    let format = AUDIO_S16LSB; // signed 16 bit samples, in little-endian byte order
    let channels = DEFAULT_CHANNELS;
    let chunk_size = 1_024;
    sdl2::mixer::open_audio(frequency, format, channels, chunk_size)?;
    sdl2::mixer::allocate_channels(16);
    let _mixer_context =
        sdl2::mixer::init(InitFlag::MP3 | InitFlag::FLAC | InitFlag::MOD | InitFlag::OGG)?;

    let mut sound_context = gamesound::GameSound::new()?;
    sound_context.load_sound("gulp1", "snake_gulp_1.wav")?;
    sound_context.load_sound("bgm", "midnight-forest-184304.wav")?;
    sound_context.load_sound("gulp2", "snake_gulp_2.wav")?;
    sound_context.load_sound("thunk", "thunk.wav")?;
    sound_context.load_sound("selfbite", "self_bite.wav")?;

    let window = video_subsystem
        .window(
            "snake_game",
            GRID_X_SIZE * DOT_SIZE_IN_PXS,
            GRID_Y_SIZE * DOT_SIZE_IN_PXS,
        )
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut renderer = gamerenderer::Renderer::new(window)?;

    let mut event_pump = sdl_context.event_pump()?;

    let mut context = gamecontext::GameContext::new();

    let mut frame_counter = 0;
    let mut game_time = Instant::now();
    let mut fps_time = Instant::now();

    //start music
    sound_context.play_sound("bgm", -1)?;

    'running: loop {
        //handle input
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => match keycode {
                    Keycode::W => context.move_up(),
                    Keycode::S => context.move_down(),
                    Keycode::D => context.move_right(),
                    Keycode::A => context.move_left(),
                    Keycode::Space => context.toggle_pause(),
                    Keycode::Escape => break 'running,
                    Keycode::N => context = gamecontext::GameContext::new(),
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
                },
                _ => {}
            }
        }

        //calculate FPS
        let fps_duration = fps_time.elapsed();
        if fps_duration.as_millis() >= 1_000 {
            context.fps = frame_counter;
            frame_counter = 0;
            fps_time = Instant::now();
        }

        //draw game
        renderer.draw(&context)?;
        frame_counter += 1;

        //control speed of game
        let duration = game_time.elapsed();
        if duration.as_millis() >= 64 {
            context.next_tick();
            game_time = Instant::now();
        }

        //play sound
        while let Some(sound) = context.sound_queue.pop() {
            match sound {
                Some(effect) => match effect {
                    SoundEffect::Gulp1 => sound_context.play_sound("gulp1", 0)?,
                    SoundEffect::Gulp2 => sound_context.play_sound("gulp2", 0)?,
                    SoundEffect::Thunk => sound_context.play_sound("thunk", 0)?,
                    SoundEffect::Selfbite => sound_context.play_sound("selfbite", 0)?,
                },
                None => {}
            };
        }
    }

    Ok(())
}
