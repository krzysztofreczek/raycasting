use std::f64::consts::PI;

pub fn calculate_other_endpoint(x1: f64, y1: f64, length: f64, angle: f64) -> (f64, f64) {
    // Convert angle to radians
    let angle_rad = angle * PI / 180.0;
    
    // Calculate the coordinates of the other endpoint
    let x2 = x1 + length * angle_rad.cos();
    let y2 = y1 + length * angle_rad.sin();
    
    (x2, y2)
}

fn orientation(p: (f64, f64), q: (f64, f64), r: (f64, f64)) -> i32 {
    let val = (q.1 - p.1) * (r.0 - q.0) - (q.0 - p.0) * (r.1 - q.1);
    if val == 0.0 {
        return 0;
    } else if val > 0.0 {
        return 1;
    } else {
        return 2;
    }
}

fn on_segment(p: (f64, f64), q: (f64, f64), r: (f64, f64)) -> bool {
    q.0 <= p.0.max(r.0) && q.0 >= p.0.min(r.0) &&
        q.1 <= p.1.max(r.1) && q.1 >= p.1.min(r.1)
}

fn segments_intersect(s1: ((f64, f64), (f64, f64)), s2: ((f64, f64), (f64, f64))) -> bool {
    let o1 = orientation(s1.0, s1.1, s2.0);
    let o2 = orientation(s1.0, s1.1, s2.1);
    let o3 = orientation(s2.0, s2.1, s1.0);
    let o4 = orientation(s2.0, s2.1, s1.1);

    if o1 != o2 && o3 != o4 {
        return true;
    }

    if o1 == 0 && on_segment(s1.0, s2.0, s1.1) {
        return true;
    }
    if o2 == 0 && on_segment(s1.0, s2.1, s1.1) {
        return true;
    }
    if o3 == 0 && on_segment(s2.0, s1.0, s2.1) {
        return true;
    }
    if o4 == 0 && on_segment(s2.0, s1.1, s2.1) {
        return true;
    }

    false
}

pub fn find_intersection(s1: ((f64, f64), (f64, f64)), s2: ((f64, f64), (f64, f64))) -> Option<(f64, f64)> {
    if !segments_intersect(s1, s2) {
        return None;
    }

    let a1 = s1.1.1 - s1.0.1;
    let b1 = s1.0.0 - s1.1.0;
    let c1 = a1 * s1.0.0 + b1 * s1.0.1;

    let a2 = s2.1.1 - s2.0.1;
    let b2 = s2.0.0 - s2.1.0;
    let c2 = a2 * s2.0.0 + b2 * s2.0.1;

    let determinant = a1 * b2 - a2 * b1;

    if determinant == 0.0 {
        return None;
    }

    let x = (b2 * c1 - b1 * c2) / determinant;
    let y = (a1 * c2 - a2 * c1) / determinant;

    let intersection_point = (x, y);

    if on_segment(s1.0, intersection_point, s1.1) && on_segment(s2.0, intersection_point, s2.1) {
        return Some(intersection_point);
    }

    None
}

pub fn distance_between_points(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
}
