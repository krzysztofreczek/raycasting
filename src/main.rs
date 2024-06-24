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

const VIEW_WIDTH: f64 = 1000.0;
const VIEW_HEIGHT: f64 = 1000.0;
struct Point2D {
    x: f64,
    y: f64,
}

const MAP_POINTS: [Point2D; 7] = [
    Point2D {
        x: -100.0,
        y: -100.0,
    },
    Point2D { x: -70.0, y: 100.0 },
    Point2D { x: 10.0, y: 60.0 },
    Point2D { x: 20.0, y: 60.0 },
    Point2D { x: 100.0, y: 100.0 },
    Point2D {
        x: 120.0,
        y: -100.0,
    },
    Point2D { x: 70.0, y: -120.0 },
];

const INITIAL_CAM_POS: Point2D = Point2D { x: 0.0, y: 0.0 };
const CAM_SPEED: f64 = 5.0;

const POINT_WIDTH: u32 = 1;

pub fn main() -> Result<(), String> {
    let mut cam_pos = INITIAL_CAM_POS;

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
                _ => {}
            }
        }

        let cam_pos_points = cam_pos_view_points(&cam_pos);

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        let mut previous_visible_point: Option<Point2D> = None;

        for i in 0..MAP_POINTS.len() {
            let j0 = i % MAP_POINTS.len();
            let j1 = (i + 1) % MAP_POINTS.len();
            print_2d_line(&mut canvas, &MAP_POINTS[j0], &MAP_POINTS[j1]);

            let mut next_visible_point: Option<Point2D> = None;

            let ir = calculations::do_segments_intersect(
                (MAP_POINTS[j0].x, MAP_POINTS[j0].y),
                (MAP_POINTS[j1].x, MAP_POINTS[j1].y),
                (cam_pos.x, cam_pos.y),
                (cam_pos_points.0.x, cam_pos_points.0.y),
            );
            if ir.0 {
                print_2d_point(
                    &mut canvas,
                    &Point2D {
                        x: ir.1.unwrap().0,
                        y: ir.1.unwrap().1,
                    },
                );
                print_3d_point(
                    &mut canvas,
                    &cam_pos,
                    &Point2D {
                        x: ir.1.unwrap().0,
                        y: ir.1.unwrap().1,
                    },
                );
                next_visible_point = Some(Point2D {
                    x: ir.1.unwrap().0,
                    y: ir.1.unwrap().1,
                });
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
                    &Point2D {
                        x: ir.1.unwrap().0,
                        y: ir.1.unwrap().1,
                    },
                );
                print_3d_point(
                    &mut canvas,
                    &cam_pos,
                    &Point2D {
                        x: ir.1.unwrap().0,
                        y: ir.1.unwrap().1,
                    },
                );
                next_visible_point = Some(Point2D {
                    x: ir.1.unwrap().0,
                    y: ir.1.unwrap().1,
                });
            }

            let point_visible = calculations::is_point_within_triange(
                (MAP_POINTS[j0].x, MAP_POINTS[j0].y),
                (cam_pos.x, cam_pos.y),
                (cam_pos_points.0.x, cam_pos_points.0.y),
                (cam_pos_points.1.x, cam_pos_points.1.y),
            );
            if point_visible {
                print_2d_point(&mut canvas, &MAP_POINTS[j0]);
                print_3d_point(&mut canvas, &cam_pos, &MAP_POINTS[j0]);
                next_visible_point = Some(Point2D {
                    x: MAP_POINTS[j0].x,
                    y: MAP_POINTS[j0].y,
                });
            }

            if next_visible_point.is_some() {
                let np = next_visible_point.unwrap();
                if previous_visible_point.is_some() {
                    print_3d_line(
                        &mut canvas,
                        &cam_pos,
                        &previous_visible_point.unwrap(),
                        &np,
                    );
                }   
                previous_visible_point = Some(np);             
            } else {
                previous_visible_point = None;
            }
        }

        print_2d_camera(&mut canvas, &cam_pos);

        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }

    Ok(())
}

fn print_2d_point(canvas: &mut Canvas<Window>, p: &Point2D) {
    canvas.set_draw_color(Color::RGB(0, 255, 0));

    let x = SCREEN_WIDTH / 2 + p.x as i32;
    let y = SCREEN_HEIGHT / 2 - p.y as i32;

    canvas
        .draw_rect(Rect::new(x, y, POINT_WIDTH, POINT_WIDTH))
        .unwrap();
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

fn print_3d_point(canvas: &mut Canvas<Window>, cam_pos: &Point2D, p: &Point2D) {
    let (px, py) = point_3d(&cam_pos, &p);

    canvas.set_draw_color(Color::RGB(255, 255, 0));
    canvas
        .draw_rect(Rect::new(
            px as i32,
            SCREEN_HEIGHT - py as i32,
            POINT_WIDTH,
            POINT_WIDTH,
        ))
        .unwrap();
}

fn print_3d_line(canvas: &mut Canvas<Window>, cam_pos: &Point2D, p1: &Point2D, p2: &Point2D) {
    let (x1, y1) = point_3d(&cam_pos, &p1);
    let (x2, y2) = point_3d(&cam_pos, &p2);

    canvas.set_draw_color(Color::RGB(255, 255, 0));
    canvas
        .draw_line(
            Point::new(x1 as i32, SCREEN_HEIGHT - y1 as i32),
            Point::new(x2 as i32, SCREEN_HEIGHT - y2 as i32),
        )
        .unwrap();
}

fn print_2d_camera(canvas: &mut Canvas<Window>, cam_pos: &Point2D) {
    canvas.set_draw_color(Color::RGB(255, 0, 0));
    canvas
        .draw_rect(Rect::new(
            SCREEN_WIDTH / 2 + cam_pos.x as i32,
            SCREEN_HEIGHT / 2 - cam_pos.y as i32,
            POINT_WIDTH,
            POINT_WIDTH,
        ))
        .unwrap();

    let cam_pos_points = cam_pos_view_points(&cam_pos);

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

fn cam_pos_view_points(cam_pos: &Point2D) -> (Point2D, Point2D) {
    let x1 = cam_pos.x - VIEW_WIDTH / 2.0;
    let y1 = cam_pos.y + VIEW_HEIGHT;
    let x2 = cam_pos.x + VIEW_WIDTH / 2.0;
    let y2 = cam_pos.y + VIEW_HEIGHT;

    return (Point2D { x: x1, y: y1 }, Point2D { x: x2, y: y2 });
}

fn point_3d(cam_pos: &Point2D, p: &Point2D) -> (f64, f64) {
    let cam_pos_points = cam_pos_view_points(&cam_pos);

    let left_distances = calculations::point_to_segment_distances(
        (p.x, p.y),
        (cam_pos.x, cam_pos.y),
        (cam_pos_points.0.x, cam_pos_points.0.y),
    );

    let right_distances = calculations::point_to_segment_distances(
        (p.x, p.y),
        (cam_pos.x, cam_pos.y),
        (cam_pos_points.1.x, cam_pos_points.1.y),
    );

    let cam_distance = calculations::point_to_point_distance((cam_pos.x, cam_pos.y), (p.x, p.y));

    let px = left_distances.0 / (left_distances.0 + right_distances.0) * SCREEN_WIDTH as f64;
    let py = (cam_distance.1 / VIEW_HEIGHT) * (SCREEN_HEIGHT / 2) as f64;

    return (px, py);
}
