extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::time::Duration;

mod calculations;

const SCREEN_WIDTH: i32 = 1280;
const SCREEN_HEIGHT: i32 = 720;

const FIELD_VIEW_LENGTH: f64 = 300.0;
const FIELD_VIEW_ANGLE: f64 = 40.0;

struct Point2D {
    x: f64,
    y: f64,
}

const MAP_WALLS: [(Point2D, Point2D); 9] = [
    (
        Point2D {
            x: -100.0,
            y: -100.0,
        },
        Point2D {
            x: -100.0,
            y: 100.0,
        },
    ),
    (
        Point2D {
            x: -100.0,
            y: 100.0,
        },
        Point2D { x: -50.0, y: 100.0 },
    ),
    (
        Point2D { x: -50.0, y: 100.0 },
        Point2D { x: -50.0, y: 50.0 },
    ),
    (Point2D { x: -50.0, y: 50.0 }, Point2D { x: 0.0, y: 50.0 }),
    (Point2D { x: 0.0, y: 50.0 }, Point2D { x: 0.0, y: 100.0 }),
    (Point2D { x: 0.0, y: 100.0 }, Point2D { x: 100.0, y: 100.0 }),
    (
        Point2D { x: 100.0, y: 100.0 },
        Point2D { x: 150.0, y: 50.0 },
    ),
    (
        Point2D { x: 150.0, y: 50.0 },
        Point2D {
            x: 150.0,
            y: -100.0,
        },
    ),
    (
        Point2D {
            x: 150.0,
            y: -100.0,
        },
        Point2D {
            x: -100.0,
            y: -100.0,
        },
    ),
];

const INITIAL_CAM_POS: Point2D = Point2D { x: 0.0, y: 0.0 };
const INITIAL_CAM_ANGLE: f64 = 90.0;
const CAM_SPEED: f64 = 5.0;
const CAM_ROATION_SPEED: f64 = 5.0;

const POINT_WIDTH: u32 = 1;

pub fn main() -> Result<(), String> {
    let mut cam_pos = INITIAL_CAM_POS;
    let mut cam_angle = INITIAL_CAM_ANGLE;

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window(
            "raycasting: Video",
            SCREEN_WIDTH.try_into().unwrap(),
            SCREEN_HEIGHT.try_into().unwrap(),
        )
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    cam_pos.y += CAM_SPEED;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    cam_pos.y -= CAM_SPEED;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    cam_pos.x += CAM_SPEED;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    cam_pos.x -= CAM_SPEED;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    cam_angle += CAM_ROATION_SPEED;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    cam_angle -= CAM_ROATION_SPEED;
                }
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        print_2d_camera(&mut canvas, &cam_pos, &cam_angle);

        scan(&mut canvas, &cam_pos, &cam_angle);

        for w in MAP_WALLS {
            print_2d_line(&mut canvas, &w.0, &w.1);
        }

        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }

    Ok(())
}

const MIN_DISTANCE_HEIGHT: i32 = 300;

fn scan(canvas: &mut Canvas<Window>, cam_pos: &Point2D, cam_angle: &f64) {
    let mut i = 0.0;
    for a in -FIELD_VIEW_ANGLE as i32..FIELD_VIEW_ANGLE as i32+1 {
        let angle = *cam_angle + a as f64;
        print!("angle: {}\n", angle);
        print!("cam_pos: ({}, {})\n", cam_pos.x, cam_pos.y);
        
        let mut min_distance = FIELD_VIEW_LENGTH;
        for w in MAP_WALLS {
            let intersection = calculations::intersection_point_with_segment(
                w.0.x,
                w.0.y,
                w.1.x,
                w.1.y,
                cam_pos.x,
                cam_pos.y,
                angle,
            );

            match intersection {
                Some((xi, yi)) => {
                    let distance = calculations::distance_between_points(cam_pos.x, cam_pos.y, xi, yi);
                    if distance < min_distance {
                        min_distance = distance;
                    };
                },
                None => {},
            }
        }

        print!("min_distance: {}\n", min_distance);

        let color_mul = 255 - (min_distance / FIELD_VIEW_LENGTH * 255.0) as u8;
        canvas.set_draw_color(Color::RGB(0, 0, 255 & color_mul));

        let x = SCREEN_WIDTH as f64 - (i / (2.0 * (FIELD_VIEW_ANGLE) + 1.0)) * SCREEN_WIDTH as f64;
        let y = ((FIELD_VIEW_LENGTH - min_distance) / FIELD_VIEW_LENGTH) * MIN_DISTANCE_HEIGHT as f64;

        canvas
            .draw_line(
                Point::new(x as i32, SCREEN_HEIGHT / 2 - y as i32),
                Point::new(x as i32, SCREEN_HEIGHT / 2 + y as i32),
            )
            .unwrap();

        i += 1.0;
    }
}

fn print_2d_line(canvas: &mut Canvas<Window>, p1: &Point2D, p2: &Point2D) {
    canvas.set_draw_color(Color::RGB(0, 255, 0));

    let x1 = ((SCREEN_WIDTH / 2) as f64 + p1.x) as i32;
    let y1 = ((SCREEN_HEIGHT / 2) as f64 - p1.y) as i32;
    let x2 = ((SCREEN_WIDTH / 2) as f64 + p2.x) as i32;
    let y2 = ((SCREEN_HEIGHT / 2) as f64 - p2.y) as i32;

    canvas
        .draw_line(Point::new(x1, y1), Point::new(x2, y2))
        .unwrap();
}

fn print_2d_camera(canvas: &mut Canvas<Window>, cam_pos: &Point2D, cam_angle: &f64) {
    canvas.set_draw_color(Color::RGB(255, 0, 0));
    canvas
        .draw_rect(Rect::new(
            SCREEN_WIDTH / 2 + cam_pos.x as i32,
            SCREEN_HEIGHT / 2 - cam_pos.y as i32,
            POINT_WIDTH,
            POINT_WIDTH,
        ))
        .unwrap();

    let cam_pos_points = cam_pos_view_points(&cam_pos, &cam_angle);

    canvas
        .draw_line(
            Point::new(
                SCREEN_WIDTH / 2 + cam_pos.x as i32,
                SCREEN_HEIGHT / 2 - cam_pos.y as i32,
            ),
            Point::new(
                SCREEN_WIDTH / 2 + cam_pos_points.0.x as i32,
                SCREEN_HEIGHT / 2 - cam_pos_points.0.y as i32,
            ),
        )
        .unwrap();

    canvas
        .draw_line(
            Point::new(
                SCREEN_WIDTH / 2 + cam_pos_points.0.x as i32,
                SCREEN_HEIGHT / 2 - cam_pos_points.0.y as i32,
            ),
            Point::new(
                SCREEN_WIDTH / 2 + cam_pos_points.1.x as i32,
                SCREEN_HEIGHT / 2 - cam_pos_points.1.y as i32,
            ),
        )
        .unwrap();

    canvas
        .draw_line(
            Point::new(
                SCREEN_WIDTH / 2 + cam_pos_points.1.x as i32,
                SCREEN_HEIGHT / 2 - cam_pos_points.1.y as i32,
            ),
            Point::new(
                SCREEN_WIDTH / 2 + cam_pos.x as i32,
                SCREEN_HEIGHT / 2 - cam_pos.y as i32,
            ),
        )
        .unwrap();
}

fn cam_pos_view_points(cam_pos: &Point2D, cam_angle: &f64) -> (Point2D, Point2D) {
    let (x1, y1) = calculations::calculate_other_endpoint(
        cam_pos.x,
        cam_pos.y,
        FIELD_VIEW_LENGTH,
        cam_angle + FIELD_VIEW_ANGLE,
    );

    let (x2, y2) = calculations::calculate_other_endpoint(
        cam_pos.x,
        cam_pos.y,
        FIELD_VIEW_LENGTH,
        cam_angle - FIELD_VIEW_ANGLE,
    );

    return (Point2D { x: x1, y: y1 }, Point2D { x: x2, y: y2 });
}
