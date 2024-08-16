extern crate rand;
extern crate sdl2;

use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mixer::{InitFlag, AUDIO_S16LSB, DEFAULT_CHANNELS};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::video::Window;
use std::ops::Add;
use std::path::Path;
use std::time::Instant;

const GRID_X_SIZE: u32 = 40;
const GRID_Y_SIZE: u32 = 30;
const DOT_SIZE_IN_PXS: u32 = 20;

#[derive(PartialEq)]
pub enum GameState {
    Playing,
    Paused,
    Over,
}

#[derive(PartialEq)]
pub enum PlayerDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone)]
pub struct Point(pub i32, pub i32);

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl PartialEq<Point> for Point {
    fn eq(self: &Point, rhs: &Point) -> bool {
        return self.0 == rhs.0 && self.1 == rhs.1;
    }
}

pub struct GameContext {
    pub player_position: Vec<Point>,
    pub player_direction: PlayerDirection,
    pub food: Point,
    pub state: GameState,
    pub food_eaten: u32,
    pub fps: i32,
    pub show_fps: bool,
    pub play_sound_gulp: bool,
}

impl GameContext {
    pub fn new() -> GameContext {
        GameContext {
            player_position: vec![Point(3, 1), Point(2, 1), Point(1, 1)],
            player_direction: PlayerDirection::Right,
            state: GameState::Paused,
            food: Point(3, 3),
            food_eaten: 0,
            fps: 0,
            show_fps: false,
            play_sound_gulp: false,
        }
    }

    pub fn next_tick(&mut self) {
        if let GameState::Over = self.state {
            return;
        }
        if let GameState::Paused = self.state {
            return;
        }
        let head_position = self.player_position.first().unwrap();
        let next_head_position = match self.player_direction {
            PlayerDirection::Up => *head_position + Point(0, -1),
            PlayerDirection::Down => *head_position + Point(0, 1),
            PlayerDirection::Right => *head_position + Point(1, 0),
            PlayerDirection::Left => *head_position + Point(-1, 0),
        };

        self.player_position.pop();
        self.player_position.reverse();
        self.player_position.push(next_head_position);
        self.player_position.reverse();

        self.check_collision();
    }

    pub fn move_up(&mut self) {
        if self.player_direction != PlayerDirection::Down {
            self.player_direction = PlayerDirection::Up;
        }
    }

    pub fn move_down(&mut self) {
        if self.player_direction != PlayerDirection::Up {
            self.player_direction = PlayerDirection::Down;
        }
    }

    pub fn move_right(&mut self) {
        if self.player_direction != PlayerDirection::Left {
            self.player_direction = PlayerDirection::Right;
        }
    }

    pub fn move_left(&mut self) {
        if self.player_direction != PlayerDirection::Right {
            self.player_direction = PlayerDirection::Left;
        }
    }

    pub fn toggle_pause(&mut self) {
        self.state = match self.state {
            GameState::Playing => GameState::Paused,
            GameState::Paused => GameState::Playing,
            GameState::Over => GameState::Over,
        }
    }

    fn game_over(&mut self) {
        println!("Game Over, food eaten: {}", self.food_eaten);
        self.state = GameState::Over;
    }

    fn check_collision(&mut self) {
        //x/y wall collision
        if self.player_position[0].0 < 0 || self.player_position[0].0 > (GRID_X_SIZE - 1) as i32 {
            println!("Collision with wall");
            self.game_over();
        }
        if self.player_position[0].1 < 0 || self.player_position[0].1 > (GRID_Y_SIZE - 1) as i32 {
            println!("Collision with wall");
            self.game_over();
        }
        //self collision must be above food collision to avoid colliding with newly grown segment.
        for i in 1..self.player_position.len() {
            if self.player_position[0] == self.player_position[i] {
                println!(
                    "Collision with head {},{} and segment {},{} in segment position {}",
                    self.player_position[0].0,
                    self.player_position[0].1,
                    self.player_position[i].0,
                    self.player_position[i].1,
                    i + 1
                );
                self.game_over();
            }
        }
        //fruit collision
        if self.player_position[0] == self.food {
            //spawn new food
            let tail = self.food.clone();
            self.player_position.push(tail);
            self.food_eaten += 1;
            self.play_sound_gulp = true;

            self.spawn_food();
        }
    }

    fn spawn_food(&mut self) {
        //todo: make it so food can't spawn inside snake body.
        let mut rng = rand::thread_rng();

        let x = rng.gen_range(0..GRID_X_SIZE as i32);
        let y = rng.gen_range(0..GRID_Y_SIZE as i32);

        let new_food = Point(x, y);
        self.food = new_food;
    }

    pub fn toggle_fps(&mut self) {
        self.show_fps = match self.show_fps {
            true => false,
            false => true,
        };
    }

    pub fn check_sounds(&mut self) -> bool {
        if self.play_sound_gulp {
            self.play_sound_gulp = false;
            return true;
        } else {
            return false;
        }
    }
}

pub struct Renderer {
    canvas: WindowCanvas,
    ttf: Sdl2TtfContext,
}

impl Renderer {
    pub fn new(window: Window) -> Result<Renderer, String> {
        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        let ttf = sdl2::ttf::init().map_err(|e| e.to_string())?;

        Ok(Renderer { canvas, ttf })
    }

    fn draw_dot(&mut self, point: &Point) -> Result<(), String> {
        let Point(x, y) = point;
        self.canvas.fill_rect(Rect::new(
            x * DOT_SIZE_IN_PXS as i32,
            y * DOT_SIZE_IN_PXS as i32,
            DOT_SIZE_IN_PXS,
            DOT_SIZE_IN_PXS,
        ))?;

        Ok(())
    }

    fn draw_background(&mut self, context: &GameContext) {
        let color = match context.state {
            GameState::Playing => Color::RGB(0, 0, 0),
            GameState::Paused => Color::RGB(30, 30, 30),
            GameState::Over => Color::RED,
        };
        self.canvas.set_draw_color(color);
        self.canvas.clear();

        if context.state == GameState::Paused {
            self.draw_text("Snake Game", Color::WHITE, 128, 120, 100);
            self.draw_text("PAUSED", Color::RED, 64, 260, 240);
            self.draw_text(
                "ESC: quit   SB: toggle pause    N: new game WASD: move",
                Color::GRAY,
                32,
                40,
                560,
            );
        }

        if context.state == GameState::Over {
            self.draw_text("Snake Game", Color::WHITE, 128, 120, 100);
            self.draw_text("Game Over", Color::BLACK, 64, 260, 220);
            let status_text = format!(
                "You ate {} fruit before you ate the wall or yourself!",
                context.food_eaten
            );
            self.draw_text(&status_text, Color::BLACK, 32, 40, 560);
        }

        if context.state == GameState::Playing && context.show_fps {
            let fps_text = format!("fps:{}", context.fps);
            self.draw_text(&fps_text, Color::GREEN, 16, 0, 0);
        }
    }

    fn draw_player(&mut self, context: &GameContext) -> Result<(), String> {
        self.canvas.set_draw_color(Color::GREEN);
        for point in &context.player_position {
            self.draw_dot(point)?;
        }

        Ok(())
    }

    fn draw_food(&mut self, context: &GameContext) -> Result<(), String> {
        self.canvas.set_draw_color(Color::RED);
        self.draw_dot(&context.food)?;
        Ok(())
    }

    pub fn draw(&mut self, context: &GameContext) -> Result<(), String> {
        self.draw_background(context);
        self.draw_player(context)?;
        self.draw_food(context)?;
        self.canvas.present();

        Ok(())
    }

    fn draw_text(&mut self, text: &str, color: Color, size: u16, x_pos: i32, y_pos: i32) {
        // Render text to a surface, then create a texture from the surface
        let surface = self
            .ttf
            .load_font(Path::new("Zdybak-Regular.ttf"), size)
            .unwrap()
            .render(text)
            .blended(color)
            .map_err(|e| e.to_string())
            .unwrap();

        let texture_creator = self.canvas.texture_creator();
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())
            .unwrap();

        // Get the size of the rendered text to position it on the screen
        let target = Rect::new(x_pos, y_pos, surface.width(), surface.height());
        self.canvas.copy(&texture, None, Some(target)).unwrap();
    }
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _audio_subsystem = sdl_context.audio()?;

    let frequency = 44_100;
    let format = AUDIO_S16LSB; // signed 16 bit samples, in little-endian byte order
    let channels = DEFAULT_CHANNELS; // Stereo
    let chunk_size = 1_024;
    sdl2::mixer::open_audio(frequency, format, channels, chunk_size)?;
    let _mixer_context =
        sdl2::mixer::init(InitFlag::MP3 | InitFlag::FLAC | InitFlag::MOD | InitFlag::OGG)?;

    sdl2::mixer::allocate_channels(4);

    let gulp_sound = sdl2::mixer::Music::from_file(Path::new("snake_gulp_1.wav"))?;

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

    let mut renderer = Renderer::new(window)?;

    renderer
        .ttf
        .load_font(Path::new("Zdybak-Regular.ttf"), 128)?;

    let mut event_pump = sdl_context.event_pump()?;

    let mut context = GameContext::new();

    let mut frame_counter = 0;
    let mut game_time = Instant::now();
    let mut fps_time = Instant::now();

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
                    Keycode::N => context = GameContext::new(),
                    Keycode::P => context.toggle_fps(),
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
        if context.check_sounds() {
            gulp_sound.play(1)?;
        }
    }

    sdl2::mixer::Music::halt();

    Ok(())
}
