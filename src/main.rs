#![windows_subsystem = "windows"]

pub mod configs;
pub mod particle;
pub mod quadtree;

use std::f32::consts::TAU;

use crate::particle::Particle;
use configs::*;
use quadtree::QuadTree;

use macroquad::{
    color::colors::*,
    prelude::{is_key_pressed, Circle, KeyCode, Rect, Vec2},
    text::{draw_text_ex, TextParams},
    time::get_fps,
    window::{clear_background, next_frame, screen_height, screen_width, Conf},
};

use rand::Rng;

fn generate_random_points(number_of_points: f32) -> Vec<Particle> {
    let mut points: Vec<Particle> = Vec::new();
    let mut rng = rand::thread_rng();

    for _ in 0..number_of_points as i32 {
        points.push(Particle::new(
            Vec2 {
                x: rng.gen_range(POINT_RADIUS..screen_width() - POINT_RADIUS),
                y: rng.gen_range(POINT_RADIUS..screen_height() - POINT_RADIUS),
            },
            POINT_COLOR,
            POINT_RADIUS,
        ));
    }
    points
}

fn draw_points(points: &Vec<Particle>) {
    for point in points {
        point.display();
    }
}

fn move_points(points: &mut Vec<Particle>) {
    let mut rng = rand::thread_rng();

    for point in points {
        let angle = rng.gen_range(0.0..TAU);
        let pos = point.get_pos();
        point.set_pos(Vec2 {
            x: (pos.x + RANDOM_WALK_DISTANCE * angle.cos() + screen_width()) % screen_width(),
            y: (pos.y + RANDOM_WALK_DISTANCE * angle.sin() + screen_height()) % screen_height(),
        });
    }
}

// fn check_overlap(points: &mut Vec<Particle>) {
//     let len = points.len();
//     'outer: for index_1 in 0..len {
//         for index_2 in 0..len {
//             if index_1 != index_2 {
//                 if points[index_1].itersects(&points[index_2]) {
//                     points[index_1].color = BLUE;
//                     points[index_2].color = BLUE;
//                     continue 'outer;
//                 }
//             }
//             points[index_1].color = RED;
//         }
//     }
// }

fn check_overlap(points: &mut Vec<Particle>, quadtree: QuadTree) {
    for index in 0..points.len() {
        let pos = points[index].get_pos();
        let overlap = quadtree.query(Circle {
            x: pos.x,
            y: pos.y,
            r: 2.0 * points[index].get_radius(),
        });
        if overlap.len() > 1 {
            points[index].set_color(BLUE);
            // for index_2 in 0..overlap.len() {
            //     overlap[index_2].color = BLUE;
            // }
        } else {
            points[index].set_color(RED);
        }
    }
}

fn build_quadtree(points: &mut Vec<Particle>) -> QuadTree {
    let mut quadtree = QuadTree::new(
        QUADTREE_CAPACITY,
        Rect::new(0.0, 0.0, screen_width(), screen_height()),
    );
    for index in 0..points.len() {
        quadtree.insert(points[index]);
    }
    quadtree
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Quadtree Visualizer".to_owned(),
        window_width: WINDOW_WIDTH as i32,
        window_height: WINDOW_HEIGHT as i32,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut points = generate_random_points(NUMBER_OF_POINTS);

    loop {
        clear_background(GRAY);

        if is_key_pressed(KeyCode::Space) {
            points = generate_random_points(NUMBER_OF_POINTS);
        }

        move_points(&mut points);

        let mut quadtree = build_quadtree(&mut points);
        quadtree.display(4.0, GREEN);

        // check_overlap(&mut points);
        check_overlap(&mut points, quadtree);

        draw_points(&points);

        let fps_text = format!("{}", get_fps());
        draw_text_ex(
            &fps_text,
            screen_width() as f32 - 45.0,
            screen_height() as f32 - 14.0,
            TextParams {
                font_size: 30u16,
                color: BLACK,
                ..Default::default()
            },
        );

        // let quadtree_text = format!("{:?}", quadtree.boundary);
        // draw_text_ex(
        //     &quadtree_text,
        //     45.0,
        //     WINDOW_HEIGHT as f32 - 30.0,
        //     TextParams {
        //         font_size: 30u16,
        //         color: BLACK,
        //         ..Default::default()
        //     },
        // );

        next_frame().await
    }
}

// fps counter
// let fps_text = format!("{}", get_fps());
// draw_text_ex(
//     &fps_text,
//     WINDOW_WIDTH as f32 - 45.0,
//     WINDOW_HEIGHT as f32 - 14.0,
//     TextParams {
//         font_size: 30u16,
//         color: BLACK,
//         ..Default::default()
//     },
// );
