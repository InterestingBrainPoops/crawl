use ggez::graphics;
use glam::Vec2;

use crate::constants::GRID_SIZE;

pub struct CellBuilder {
    next_id: u64,
}

impl CellBuilder {
    pub fn new() -> Self {
        Self { next_id: 0 }
    }

    pub fn new_cell(&mut self, size: Vec2, location: Vec2, visible: bool) -> Cell {
        self.next_id += 1;
        Cell {
            id: self.next_id,
            size,
            location,
            visible,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Cell {
    id: u64,
    size: Vec2,
    location: Vec2,
    visible: bool,
}
pub struct Map {
    pub cells: Vec<Cell>,
}
impl Cell {
    pub fn new(id: u64, size: Vec2, location: Vec2, visible: bool) -> Self {
        Self {
            id,
            size,
            location,
            visible,
        }
    }

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

    pub fn contains(&self, pos: Vec2) -> bool {
        pos.x > self.location.x
            && pos.x < (self.location.x + self.size.x)
            && pos.y > self.location.y
            && pos.y < (self.location.y + self.size.y)
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn id(&self) -> u64 {
        self.id
    }
}

impl Map {
    pub fn draw(&self, canvas: &mut graphics::Canvas) {
        for cell in &self.cells {
            cell.draw(canvas);
        }
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
