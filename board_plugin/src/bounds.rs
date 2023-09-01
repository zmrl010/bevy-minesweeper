use bevy::prelude::Vec2;

#[derive(Debug, Copy, Clone)]
pub struct Bounds2 {
    pub position: Vec2,
    pub size: Vec2,
}

impl Bounds2 {
    #[inline]
    #[must_use]
    pub fn in_bounds(&self, pos: Vec2) -> bool {
        pos.x >= self.position.x
            && pos.y >= self.position.y
            && pos.x <= self.position.x + self.size.x
            && pos.y <= self.position.y + self.size.y
    }
}
