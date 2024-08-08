extern crate rand;
extern crate sdl2;
use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::video::Window;
use std::ops::Add;
use std::time::Duration;

const GRID_X_SIZE: u32 = 40;
const GRID_Y_SIZE: u32 = 30;
const DOT_SIZE_IN_PXS: u32 = 20;

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
}

impl GameContext {
    pub fn new() -> GameContext {
        GameContext {
            player_position: vec![Point(3, 1), Point(2, 1), Point(1, 1)],
            player_direction: PlayerDirection::Right,
            state: GameState::Paused,
            food: Point(3, 3),
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
        self.state = GameState::Over;
    }

    fn check_collision(&mut self) {
        //x/y wall collision
        if self.player_position[0].0 < 0 || self.player_position[0].0 > (GRID_X_SIZE - 1) as i32 {
            self.game_over();
        }
        if self.player_position[0].1 < 0 || self.player_position[0].1 > (GRID_Y_SIZE - 1) as i32 {
            self.game_over();
        }
        //fruit collision
        if self.player_position[0] == self.food {
            //spawn new food
            self.spawn_food();
            let tail = self.food.clone();
            self.player_position.push(tail);
        }
        //self collision
        for i in 1..self.player_position.len() {
            if self.player_position[0] == self.player_position[i] {
                self.game_over();
            }
        }
    }

    fn spawn_food(&mut self) {
        let mut rng = rand::thread_rng();

        let x = rng.gen_range(0..GRID_X_SIZE as i32);
        let y = rng.gen_range(0..GRID_Y_SIZE as i32);

        let new_food = Point(x, y);
        self.food = new_food;
    }
}

pub struct Renderer {
    canvas: WindowCanvas,
}

impl Renderer {
    pub fn new(window: Window) -> Result<Renderer, String> {
        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        Ok(Renderer { canvas })
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
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window(
            "Rust sdl2 Snake",
            GRID_X_SIZE * DOT_SIZE_IN_PXS,
            GRID_Y_SIZE * DOT_SIZE_IN_PXS,
        )
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut renderer = Renderer::new(window)?;

    let mut event_pump = sdl_context.event_pump()?;

    let mut context = GameContext::new();

    let mut frame_counter = 0;

    'running: loop {
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
                    _ => {}
                },
                _ => {}
            }
        }

        renderer.draw(&context)?;
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));

        frame_counter += 1;
        if frame_counter % 5 == 0 {
            context.next_tick();
            frame_counter = 0;
        }
    }

    Ok(())
}
