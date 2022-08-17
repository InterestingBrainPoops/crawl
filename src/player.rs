use ggez::graphics;
use glam::Vec2;

use crate::constants::GRID_SIZE;

pub struct Player {
    pub pos: Vec2,
}
impl Player {
    pub fn draw(&self, canvas: &mut graphics::Canvas) {
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
