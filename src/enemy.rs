use ggez::graphics;
use glam::Vec2;

use crate::constants::GRID_SIZE;

pub struct Enemy {
    pos: Vec2,
    health: u32,
    room: u64,
    active: bool,
}

impl Enemy {
    pub fn new(pos: Vec2, health: u32, room: u64) -> Self {
        Self {
            pos,
            health,
            room,
            active: false,
        }
    }

    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    pub fn active(&self) -> bool {
        self.active
    }

    pub fn room(&self) -> u64 {
        self.room
    }
    pub fn draw(&self, canvas: &mut graphics::Canvas) {
        if self.active {
            let color = [1., 0., 0., 1.];
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
}
impl From<&Enemy> for graphics::Rect {
    fn from(enemy: &Enemy) -> Self {
        graphics::Rect::new_i32(
            enemy.pos.x as i32 * GRID_SIZE,
            enemy.pos.y as i32 * GRID_SIZE,
            GRID_SIZE,
            GRID_SIZE,
        )
    }
}
