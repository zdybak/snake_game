extern crate rand;
use crate::gamesettings::{GRID_X_SIZE, GRID_Y_SIZE};
use crate::point::Point;
use rand::Rng;

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

#[derive(Copy, Clone, PartialEq)]
pub enum SoundEffect {
    Gulp1,
    Gulp2,
    Thunk,
    Selfbite,
}

pub struct GameContext {
    pub player_position: Vec<Point>,
    pub player_direction: PlayerDirection,
    pub food: Point,
    pub state: GameState,
    pub food_eaten: u32,
    pub fps: i32,
    pub show_fps: bool,
    pub sound_queue: Vec<Option<SoundEffect>>,
    pub music: bool,
    pub quit: bool,
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
            show_fps: true,
            sound_queue: vec![None; 12],
            music: true,
            quit: false,
        }
    }

    pub fn new_game(&mut self) {
        self.player_position = vec![Point(3, 1), Point(2, 1), Point(1, 1)];
        self.player_direction = PlayerDirection::Right;
        self.state = GameState::Paused;
        self.food = Point(3, 3);
        self.food_eaten = 3;
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
        //println!("Game Over, food eaten: {}", self.food_eaten);
        self.state = GameState::Over;
    }

    fn check_collision(&mut self) {
        //x/y wall collision
        if self.player_position[0].0 < 0 || self.player_position[0].0 > (GRID_X_SIZE - 1) as i32 {
            //println!("Collision with wall");
            self.sound_queue.push(Some(SoundEffect::Thunk));
            self.game_over();
        }
        if self.player_position[0].1 < 0 || self.player_position[0].1 > (GRID_Y_SIZE - 1) as i32 {
            //println!("Collision with wall");
            self.sound_queue.push(Some(SoundEffect::Thunk));
            self.game_over();
        }
        //self collision must be above food collision to avoid colliding with newly grown segment.
        for i in 1..self.player_position.len() {
            if self.player_position[0] == self.player_position[i] {
                /*println!(
                    "Collision with head {},{} and segment {},{} in segment position {}",
                    self.player_position[0].0,
                    self.player_position[0].1,
                    self.player_position[i].0,
                    self.player_position[i].1,
                    i + 1
                );*/
                self.sound_queue.push(Some(SoundEffect::Selfbite));
                self.game_over();
            }
        }
        //fruit collision
        if self.player_position[0] == self.food {
            //spawn new food
            let tail = self.food.clone();
            self.player_position.push(tail);
            self.food_eaten += 1;

            let mut rng = rand::thread_rng();
            let gulp1 = rng.gen_bool(0.5);
            let random_gulp = match gulp1 {
                true => SoundEffect::Gulp1,
                false => SoundEffect::Gulp2,
            };
            self.sound_queue.push(Some(random_gulp));

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

    pub fn quit_game(&mut self) {
        self.quit = true;
    }
}
