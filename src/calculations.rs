fn determinant(a: f64, b: f64, c: f64, d: f64) -> f64 {
    a * d - b * c
}

pub fn do_segments_intersect(p1: (f64, f64), p2: (f64, f64), p3: (f64, f64), p4: (f64, f64)) -> (bool, Option<(f64, f64)>) {
    let (x1, y1) = p1;
    let (x2, y2) = p2;
    let (x3, y3) = p3;
    let (x4, y4) = p4;
    
    let det = determinant(x2 - x1, x3 - x4, y2 - y1, y3 - y4);
    
    if det == 0.0 {
        return (false, None);
    }
    
    let t = determinant(x3 - x1, x3 - x4, y3 - y1, y3 - y4) / det;
    let u = determinant(x2 - x1, x3 - x1, y2 - y1, y3 - y1) / det;
    
    if 0.0 <= t && t <= 1.0 && 0.0 <= u && u <= 1.0 {
        let intersection_point = (x1 + t * (x2 - x1), y1 + t * (y2 - y1));
        return (true, Some(intersection_point));
    } else {
        return (false, None);
    }
}