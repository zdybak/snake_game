extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::video::Window;
use std::path::Path;

use crate::gamecontext::{GameContext, GameState};
use crate::gamesettings::DOT_SIZE_IN_PXS;
use crate::point::Point;

pub struct Renderer {
    canvas: WindowCanvas,
    ttf: Sdl2TtfContext,
}

impl Renderer {
    pub fn new(window: Window) -> Result<Renderer, String> {
        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        let ttf = sdl2::ttf::init().map_err(|e| e.to_string())?;

        ttf.load_font(Path::new("Zdybak-Regular.ttf"), 128)?;

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
                "WASD keys,DPad: MOVE SNAKE   N key,Button A: NEW GAME",
                Color::GRAY,
                28,
                40,
                480,
            );
            self.draw_text(
                "M key,X Button: TOGGLE MUSIC   P key,Button B: SHOW FPS",
                Color::GRAY,
                28,
                40,
                520,
            );
            self.draw_text(
                "ESC key,Back Button: QUIT      Space,Start Button: UNPAUSE",
                Color::GRAY,
                28,
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
