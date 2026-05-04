use macroquad::prelude::*;
#[derive(Clone)]
pub struct body {
pub mass: f32,
pub vel: Vec2,
pub pos: Vec2,
pub color: Color

}

impl body {
    pub fn get_pos(&self) -> Vec2 {
        self.pos
    }
}