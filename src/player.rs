use crate::prelude::*;

pub struct Player {
    position: Point,
}
impl Player {
    pub fn new(position: Point)->Self {
        Self {
            position
        }
    }
    pub fn render(&self,ctx: &mut BTerm) {
        ctx.set(self.position.x, self.position.y, WHITE, BLACK, to_cp437('@))
    }
}