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

pub fn is_point_within_triange(p: (f64, f64), t1: (f64, f64), t2: (f64, f64), t3: (f64, f64)) -> bool {
    let (px, py) = p;
    let (t1x, t1y) = t1;
    let (t2x, t2y) = t2;
    let (t3x, t3y) = t3;
    
    let b1 = (px - t2x) * (t1y - t2y) - (t1x - t2x) * (py - t2y) < 0.0;
    let b2 = (px - t3x) * (t2y - t3y) - (t2x - t3x) * (py - t3y) < 0.0;
    let b3 = (px - t1x) * (t3y - t1y) - (t3x - t1x) * (py - t1y) < 0.0;
    
    return b1 == b2 && b2 == b3;
}

pub fn point_to_segment_distances(p: (f64, f64), s1: (f64, f64), s2: (f64, f64)) -> (f64, f64) {
    let (px, py) = p;
    let (s1x, s1y) = s1;
    let (s2x, s2y) = s2;

    let ab_x = s2x - s1x;
    let ab_y = s2y - s1y;
    
    let ap_x = px - s1x;
    let ap_y = py - s1y;
    
    let dot_ap_ab = ap_x * ab_x + ap_y * ab_y;
    
    let ab_len_sq = ab_x * ab_x + ab_y * ab_y;
    
    let t = dot_ap_ab / ab_len_sq;
    
    let (x_c, y_c) = if t < 0.0 {
        (s1x, s1y)
    } else if t > 1.0 {
        (s2x, s2y)
    } else {
        (s1x + t * ab_x, s1y + t * ab_y)
    };
    
    let d_x = (px - x_c).abs();
    let d_y = (py - y_c).abs();
    
    (d_x, d_y)
}


pub fn point_to_point_distance(p1: (f64, f64), p2: (f64, f64)) -> (f64, f64) {
    let (x1, y1) = p1;
    let (x2, y2) = p2;
    
    let dx = x2 - x1;
    let dy = y2 - y1;
    
    return (dx, dy);
}