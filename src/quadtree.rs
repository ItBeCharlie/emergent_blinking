use crate::particle::Particle;

use macroquad::{
    prelude::{Circle, Color, Rect},
    shapes::draw_rectangle_lines,
};

pub struct QuadTree {
    capacity: f32,
    is_full: bool,
    boundary: Rect,
    data: Vec<Particle>,
    northeast: Option<Box<QuadTree>>,
    northwest: Option<Box<QuadTree>>,
    southeast: Option<Box<QuadTree>>,
    southwest: Option<Box<QuadTree>>,
}

impl QuadTree {
    pub fn new(capacity: f32, boundary: Rect) -> QuadTree {
        return QuadTree {
            capacity: capacity,
            is_full: false,
            boundary: boundary,
            data: Vec::new(),
            northeast: None,
            northwest: None,
            southeast: None,
            southwest: None,
        };
    }

    pub fn insert(&mut self, point: Particle) {
        if self.is_full {
            if self.northeast.is_some() {
                let northeast = self.northeast.as_mut().unwrap();
                if northeast.contains(&point) {
                    northeast.insert(point);
                    return;
                }
            }
            if self.northwest.is_some() {
                let northwest = self.northwest.as_mut().unwrap();
                if northwest.contains(&point) {
                    northwest.insert(point);
                    return;
                }
            }
            if self.southeast.is_some() {
                let southeast = self.southeast.as_mut().unwrap();
                if southeast.contains(&point) {
                    southeast.insert(point);
                    return;
                }
            }
            if self.southwest.is_some() {
                let southwest = self.southwest.as_mut().unwrap();
                if southwest.contains(&point) {
                    southwest.insert(point);
                    return;
                }
            }
            return;
        }

        if self.data.len() as f32 >= self.capacity {
            self.is_full = true;
            let x = self.boundary.x;
            let y = self.boundary.y;
            let w_2 = self.boundary.clone().w / 2.0;
            let h_2 = self.boundary.clone().h / 2.0;

            self.northeast = Some(Box::new(QuadTree::new(
                self.capacity,
                Rect::new(x + w_2, y, w_2, h_2),
            )));
            self.northwest = Some(Box::new(QuadTree::new(
                self.capacity,
                Rect::new(x, y, w_2, h_2),
            )));
            self.southeast = Some(Box::new(QuadTree::new(
                self.capacity,
                Rect::new(x + w_2, y + h_2, w_2, h_2),
            )));
            self.southwest = Some(Box::new(QuadTree::new(
                self.capacity,
                Rect::new(x, y + h_2, w_2, h_2),
            )));

            // for index in 0..self.data.len() {
            //     self.insert(self.data[index]);
            // }
            return;
        }

        self.data.push(point);
    }

    fn contains(&self, point: &Particle) -> bool {
        self.boundary.contains(point.get_pos())
    }

    pub fn query(&self, range: Circle) -> Vec<Particle> {
        let mut res = Vec::new();

        if !range.overlaps_rect(&self.boundary) {
            return res;
        }

        for p in self.data.iter() {
            if range.contains(&p.get_pos()) {
                res.push(p.clone());
            }
        }

        if let Some(v) = &self.northwest {
            res.extend(v.query(range));
        }
        if let Some(v) = &self.northeast {
            res.extend(v.query(range));
        }
        if let Some(v) = &self.southwest {
            res.extend(v.query(range));
        }
        if let Some(v) = &self.southeast {
            res.extend(v.query(range));
        }

        res
    }

    pub fn display(&mut self, thickness: f32, color: Color) {
        draw_rectangle_lines(
            self.boundary.x,
            self.boundary.y,
            self.boundary.w,
            self.boundary.h,
            thickness,
            color,
        );
        if self.is_full {
            self.northeast.as_mut().unwrap().display(thickness, color);
            self.northwest.as_mut().unwrap().display(thickness, color);
            self.southeast.as_mut().unwrap().display(thickness, color);
            self.southwest.as_mut().unwrap().display(thickness, color);
        }
    }
}
