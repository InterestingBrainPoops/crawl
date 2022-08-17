use ggez::graphics::{self, Canvas};
use glam::Vec2;

use crate::constants::GRID_SIZE;

pub struct Projectile {
    kind: ProjectileType,
    position: Vec2,
    velocity: Vec2,
}
pub enum ProjectileType {
    Bullet,
}

impl Projectile {
    pub fn new(kind: ProjectileType, position: Vec2, velocity: Vec2) -> Self {
        Self {
            kind,
            position,
            velocity,
        }
    }

    pub fn draw(&self, canvas: &mut Canvas) {
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
    pub fn update(&mut self) {
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
