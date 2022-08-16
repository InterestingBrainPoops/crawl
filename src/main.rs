//! The simplest possible example that does something.
#![allow(clippy::unnecessary_wraps)]

use ggez::{
    event,
    graphics::{self, Canvas},
    winit::event::VirtualKeyCode,
    Context, GameResult,
};
use glam::*;

const GRID_SIZE: i32 = 20;

struct Player {
    pos: Vec2,
}

#[derive(Clone, Copy)]
struct Cell {
    id: u64,
    size: Vec2,
    location: Vec2,
    visible: bool,
}

struct Map {
    cells: Vec<Cell>,
}

struct Projectile {
    kind: ProjectileType,
    position: Vec2,
    velocity: Vec2,
}

impl Projectile {
    fn draw(&self, canvas: &mut Canvas) {
        let color = [0.5, 0.5, 0.5, 1.];
        // Then we draw a rectangle with the Fill draw mode, and we convert the
        // Food's position into a `ggez::Rect` using `.into()` which we can do
        // since we implemented `From<GridPosition>` for `Rect` earlier.
        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest_rect(self.into())
                .color(color),
        );
    }
    fn update(&mut self) {
        self.position += self.velocity;
    }
}
impl From<&Projectile> for graphics::Rect {
    fn from(player: &Projectile) -> Self {
        graphics::Rect::new_i32(
            player.position.x as i32 * GRID_SIZE,
            player.position.y as i32 * GRID_SIZE,
            GRID_SIZE,
            GRID_SIZE,
        )
    }
}

enum ProjectileType {
    Bullet,
}

struct Enemy {
    pos: Vec2,
    health: Vec2,
    room: u64,
}

struct MainState {
    enemies: Vec<Enemy>,
    projectiles: Vec<Projectile>,
    player: Player,
    map: Map,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        Ok(MainState {
            enemies: vec![],
            projectiles: vec![],
            player: Player {
                pos: vec2(20., 20.),
            },
            map: Map {
                cells: vec![
                    Cell {
                        id: 2,
                        size: vec2(20., 20.),
                        location: vec2(10., 10.),
                        visible: false,
                    },
                    Cell {
                        id: 1,
                        size: vec2(22., 5.),
                        location: vec2(29., 10.),
                        visible: false,
                    },
                    Cell {
                        id: 0,
                        visible: false,
                        size: vec2(20., 20.),
                        location: vec2(50., 10.),
                    },
                ],
            },
        })
    }
}

impl Player {
    fn draw(&self, canvas: &mut graphics::Canvas) {
        let color = [1., 1., 0., 1.];
        // Then we draw a rectangle with the Fill draw mode, and we convert the
        // Food's position into a `ggez::Rect` using `.into()` which we can do
        // since we implemented `From<GridPosition>` for `Rect` earlier.
        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest_rect(self.into())
                .color(color),
        );
    }
}

impl From<&Player> for graphics::Rect {
    fn from(player: &Player) -> Self {
        graphics::Rect::new_i32(
            player.pos.x as i32 * GRID_SIZE,
            player.pos.y as i32 * GRID_SIZE,
            GRID_SIZE,
            GRID_SIZE,
        )
    }
}

impl From<&Cell> for graphics::Rect {
    fn from(cell: &Cell) -> Self {
        graphics::Rect::new_i32(
            cell.location.x as i32 * GRID_SIZE,
            cell.location.y as i32 * GRID_SIZE,
            cell.size.x as i32 * GRID_SIZE,
            cell.size.y as i32 * GRID_SIZE,
        )
    }
}
impl Cell {
    fn draw(&self, canvas: &mut graphics::Canvas) {
        let color = [0.75, 0.75, 0.75, 1.0];
        // Then we draw a rectangle with the Fill draw mode, and we convert the
        // Food's position into a `ggez::Rect` using `.into()` which we can do
        // since we implemented `From<GridPosition>` for `Rect` earlier.
        if self.visible {
            canvas.draw(
                &graphics::Quad,
                graphics::DrawParam::new()
                    .dest_rect(self.into())
                    .color(color),
            );
        }
    }

    fn contains(&self, pos: Vec2) -> bool {
        pos.x > self.location.x
            && pos.x < (self.location.x + self.size.x)
            && pos.y > self.location.y
            && pos.y < (self.location.y + self.size.y)
    }
}

impl Map {
    fn draw(&self, canvas: &mut graphics::Canvas) {
        for cell in &self.cells {
            cell.draw(canvas);
        }
    }
}
impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        for cell in &mut self.map.cells {
            if cell.contains(self.player.pos) {
                cell.visible = true;
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
        self.projectiles.push(Projectile {
            kind: ProjectileType::Bullet,
            position: self.player.pos,
            velocity: difference,
        });
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
