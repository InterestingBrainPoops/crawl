//! The simplest possible example that does something.
#![allow(clippy::unnecessary_wraps)]

mod constants;
mod enemy;
mod map;
mod player;
mod projectile;

use constants::GRID_SIZE;
use enemy::Enemy;
use ggez::{
    event,
    graphics::{self, Canvas},
    winit::event::VirtualKeyCode,
    Context, GameResult,
};
use glam::*;
use map::{Cell, CellBuilder, Map};
use player::Player;
use projectile::{Projectile, ProjectileType};

struct MainState {
    enemies: Vec<Enemy>,
    projectiles: Vec<Projectile>,
    player: Player,
    map: Map,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let mut cbuilder = CellBuilder::new();
        Ok(MainState {
            enemies: vec![Enemy::new(vec2(60., 20.), 100, 3)],
            projectiles: vec![],
            player: Player {
                pos: vec2(20., 20.),
            },

            map: Map {
                cells: vec![
                    cbuilder.new_cell(vec2(20., 20.), vec2(10., 10.), false),
                    cbuilder.new_cell(vec2(22., 5.), vec2(29., 10.), false),
                    cbuilder.new_cell(vec2(20., 20.), vec2(50., 10.), false),
                ],
            },
        })
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        for cell in &mut self.map.cells {
            if cell.contains(self.player.pos) {
                cell.set_visible(true);

                for enemy in &mut self.enemies {
                    if enemy.room() == cell.id() {
                        enemy.set_active(true);
                    }
                }
            }
        }
        for proj in &mut self.projectiles {
            proj.update();
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(
            ctx,
            graphics::CanvasLoadOp::Clear([0.0, 0.0, 0.0, 1.0].into()),
        );

        self.map.draw(&mut canvas);
        self.player.draw(&mut canvas);
        for proj in &self.projectiles {
            proj.draw(&mut canvas);
        }

        for enemy in &self.enemies {
            enemy.draw(&mut canvas);
        }
        canvas.finish(ctx)?;

        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: event::MouseButton,
        x: f32,
        y: f32,
    ) -> Result<(), ggez::GameError> {
        let difference =
            (vec2(x / GRID_SIZE as f32, y / GRID_SIZE as f32) - self.player.pos).normalize() * 0.5;
        self.projectiles.push(Projectile::new(
            ProjectileType::Bullet,
            self.player.pos + vec2(0.5, 0.5),
            difference,
        ));
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        input: ggez::input::keyboard::KeyInput,
        _repeated: bool,
    ) -> Result<(), ggez::GameError> {
        let old_pos = self.player.pos;

        match input.keycode.unwrap() {
            VirtualKeyCode::W => {
                self.player.pos.y -= 1.;
            }
            VirtualKeyCode::A => {
                self.player.pos.x -= 1.;
            }
            VirtualKeyCode::D => {
                self.player.pos.x += 1.;
            }
            VirtualKeyCode::S => {
                self.player.pos.y += 1.;
            }
            _ => {}
        }
        if self
            .map
            .cells
            .iter()
            .all(|&cell| !cell.contains(self.player.pos))
        {
            self.player.pos = old_pos;
        }
        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (mut ctx, event_loop) = cb
        .window_mode(ggez::conf::WindowMode::default().dimensions(800., 600.))
        .build()?;
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
