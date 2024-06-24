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

const VIEW_WIDTH: f64 = 500.0;
const VIEW_HEIGHT: f64 = 500.0;
struct Point2D {
    x: f64,
    y: f64,
}

const MAP_POINTS: [Point2D; 5] = [
    Point2D {
        x: -100.0,
        y: -100.0,
    },
    Point2D { x: -70.0, y: 100.0 },
    Point2D { x: 100.0, y: 100.0 },
    Point2D {
        x: 120.0,
        y: -100.0,
    },
    Point2D { x: 70.0, y: -120.0 },
];

pub fn main() -> Result<(), String> {
    let mut cam_pos = Point2D { x: 0.0, y: 0.0 };

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
                    cam_pos.y += 10.0;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    cam_pos.y -= 10.0;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    cam_pos.x += 10.0;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    cam_pos.x -= 10.0;
                }
                _ => {}
            }
        }

        let cam_pos_points = cam_pos_view_points(&cam_pos);

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        for i in 0..MAP_POINTS.len() {
            let j0 = i % MAP_POINTS.len();
            let j1 = (i + 1) % MAP_POINTS.len();
            print_2d_line(&mut canvas, &cam_pos, &MAP_POINTS[j0], &MAP_POINTS[j1]);

            let ir = calculations::do_segments_intersect(
                (MAP_POINTS[j0].x, MAP_POINTS[j0].y),
                (MAP_POINTS[j1].x, MAP_POINTS[j1].y),
                (cam_pos.x, cam_pos.y),
                (cam_pos_points.0.x, cam_pos_points.0.y),
            );
            if ir.0 {
                print_2d_point(
                    &mut canvas,
                    &cam_pos,
                    &Point2D {
                        x: ir.1.unwrap().0,
                        y: ir.1.unwrap().1,
                    },
                );
            }

            let ir = calculations::do_segments_intersect(
                (MAP_POINTS[j0].x, MAP_POINTS[j0].y),
                (MAP_POINTS[j1].x, MAP_POINTS[j1].y),
                (cam_pos.x, cam_pos.y),
                (cam_pos_points.1.x, cam_pos_points.1.y),
            );
            if ir.0 {
                print_2d_point(
                    &mut canvas,
                    &cam_pos,
                    &Point2D {
                        x: ir.1.unwrap().0,
                        y: ir.1.unwrap().1,
                    },
                );
            }
        }

        print_2d_camera(&mut canvas);

        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }

    Ok(())
}

fn print_2d_line(canvas: &mut Canvas<Window>, cam_pos: &Point2D, p1: &Point2D, p2: &Point2D) {
    canvas.set_draw_color(Color::RGB(0, 255, 0));

    let x1 = ((SCREEN_WIDTH / 2) as f64 + cam_pos.x - p1.x) as i32;
    let y1 = ((SCREEN_HEIGHT / 2) as f64 + cam_pos.y - p1.y) as i32;
    let x2 = ((SCREEN_WIDTH / 2) as f64 + cam_pos.x - p2.x) as i32;
    let y2 = ((SCREEN_HEIGHT / 2) as f64 + cam_pos.y - p2.y) as i32;

    canvas
        .draw_line(Point::new(x1, y1), Point::new(x2, y2))
        .unwrap();
}

fn print_2d_point(canvas: &mut Canvas<Window>, cam_pos: &Point2D, p: &Point2D) {
    canvas.set_draw_color(Color::RGB(0, 255, 0));

    let x1 = ((SCREEN_WIDTH / 2) as f64 + cam_pos.x - p.x) as i32;
    let y1 = ((SCREEN_HEIGHT / 2) as f64 + cam_pos.y - p.y) as i32;

    canvas.draw_rect(Rect::new(x1 - 5, y1 - 5, 10, 10)).unwrap();
}

fn print_2d_camera(canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::RGB(255, 0, 0));
    canvas
        .draw_rect(Rect::new(
            SCREEN_WIDTH / 2 - 5,
            SCREEN_HEIGHT / 2 - 5,
            10,
            10,
        ))
        .unwrap();

    let cam_pos_static = Point2D {
        x: (SCREEN_WIDTH / 2) as f64,
        y: (SCREEN_HEIGHT / 2) as f64,
    };
    let cam_pos_static_points = cam_pos_view_points(&cam_pos_static);

    canvas
        .draw_line(
            Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2),
            Point::new(
                cam_pos_static_points.0.x as i32,
                cam_pos_static_points.0.y as i32,
            ),
        )
        .unwrap();

    canvas
        .draw_line(
            Point::new(
                cam_pos_static_points.0.x as i32,
                cam_pos_static_points.0.y as i32,
            ),
            Point::new(
                cam_pos_static_points.1.x as i32,
                cam_pos_static_points.1.y as i32,
            ),
        )
        .unwrap();

    canvas
        .draw_line(
            Point::new(
                cam_pos_static_points.1.x as i32,
                cam_pos_static_points.1.y as i32,
            ),
            Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2),
        )
        .unwrap();
}

fn cam_pos_view_points(cam_pos: &Point2D) -> (Point2D, Point2D) {
    let x1 = cam_pos.x + VIEW_WIDTH / 2.0;
    let y1 = cam_pos.y - VIEW_HEIGHT;
    let x2 = cam_pos.x - VIEW_WIDTH / 2.0;
    let y2 = cam_pos.y - VIEW_HEIGHT;

    return (Point2D { x: x1, y: y1 }, Point2D { x: x2, y: y2 });
}
