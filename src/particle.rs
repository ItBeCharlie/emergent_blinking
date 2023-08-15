use macroquad::{
    color::*,
    prelude::{Color, Vec2},
    shapes::draw_circle,
};

use crate::configs::*;

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Particle {
    pos: Vec2,
    color: Color,
    radius: f32,
    blink_charge: f32,
    is_discharging: bool,
}

impl Particle {
    pub fn new(pos: Vec2, color: Color, radius: f32, blink_charge: f32) -> Particle {
        Particle {
            pos,
            color,
            radius,
            blink_charge,
            is_discharging: false,
        }
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

    pub fn get_blink_charge(&self) -> f32 {
        self.blink_charge
    }

    pub fn is_discharging(&self) -> bool {
        self.is_discharging
    }

    pub fn update_blink_charge(&mut self, points: &Vec<Particle>) {
        if self.is_discharging {
            self.blink_charge -= BLINK_DISCHARGE_RATE;
            self.color = hsl_to_rgb(276.9 / 360.0, 1.0, self.blink_charge / MAX_BLINK_CHARGE);
            if self.blink_charge <= 0.0 {
                self.is_discharging = false;
                self.blink_charge = 0.0;
            }
        } else {
            self.blink_charge += BLINK_INCREMENT_RATE;
            for point in points {
                // Detection for if a point is currently blinking
                // It must be currently discharging, and it's charge is greater than one unit of decrease of blink rate
                if point.is_discharging()
                    && point.get_blink_charge()
                        > MAX_BLINK_CHARGE - (BLINK_DISCHARGE_RATE / MAX_BLINK_CHARGE)
                {
                    self.blink_charge *= 1.0 + BLINK_DETECTION_INCREMENT;
                }
            }
            // Lightness goes from 0.0..0.5 on charge up based on how charged light is
            self.color = hsl_to_rgb(
                276.9 / 360.0,
                1.0,
                self.blink_charge / MAX_BLINK_CHARGE / 2.0,
            );
            if self.blink_charge >= MAX_BLINK_CHARGE {
                self.is_discharging = true;
                self.blink_charge = MAX_BLINK_CHARGE;
            }
        }
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
