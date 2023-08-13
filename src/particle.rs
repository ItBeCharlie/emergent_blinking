use macroquad::{
    prelude::{Color, Vec2},
    shapes::draw_circle,
};

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Particle {
    pos: Vec2,
    color: Color,
    radius: f32,
}

impl Particle {
    pub fn new(pos: Vec2, color: Color, radius: f32) -> Particle {
        Particle { pos, color, radius }
    }

    pub fn set_pos(&mut self, new_pos: Vec2) {
        self.pos = new_pos;
    }

    pub fn get_pos(&self) -> Vec2 {
        self.pos
    }

    pub fn set_color(&mut self, new_color: Color) {
        self.color = new_color;
    }

    pub fn get_color(&self) -> Color {
        self.color
    }

    pub fn get_radius(&self) -> f32 {
        self.radius
    }

    pub fn itersects(&self, other: &Particle) -> bool {
        (self.pos.x - other.pos.x) * (self.pos.x - other.pos.x)
            + (self.pos.y - other.pos.y) * (self.pos.y - other.pos.y)
            < (self.radius + other.radius) * (self.radius + other.radius)
    }

    pub fn display(&self) {
        draw_circle(self.pos.x, self.pos.y, self.radius, self.color);
    }
}
