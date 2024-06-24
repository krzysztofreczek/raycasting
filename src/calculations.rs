use std::f64::consts::PI;

/// Calculates the coordinates of the other endpoint of a line segment given one endpoint, 
/// the length of the segment, and the angle of the segment with respect to the horizontal axis.
pub fn calculate_other_endpoint(x1: f64, y1: f64, length: f64, angle: f64) -> (f64, f64) {
    // Convert angle to radians
    let angle_rad = angle * PI / 180.0;
    
    // Calculate the coordinates of the other endpoint
    let x2 = x1 + length * angle_rad.cos();
    let y2 = y1 + length * angle_rad.sin();
    
    (x2, y2)
}

/// Rotates a point (x, y) by a given angle in radians.
fn rotate_point(x: f64, y: f64, angle_rad: f64) -> (f64, f64) {
    let cos_theta = angle_rad.cos();
    let sin_theta = angle_rad.sin();
    let x_rot = x * cos_theta - y * sin_theta;
    let y_rot = x * sin_theta + y * cos_theta;
    (x_rot, y_rot)
}

/// Rotates a point (x, y) by a given angle in radians in the opposite direction.
fn rotate_point_back(x: f64, y: f64, angle_rad: f64) -> (f64, f64) {
    let cos_theta = angle_rad.cos();
    let sin_theta = angle_rad.sin();
    let x_rot = x * cos_theta + y * sin_theta;
    let y_rot = -x * sin_theta + y * cos_theta;
    (x_rot, y_rot)
}

/// Calculates the intersection point between the segment and the perpendicular dropped from the point.
pub fn intersection_point_with_segment(
    x1: f64, y1: f64,  // Segment start
    x2: f64, y2: f64,  // Segment end
    xp: f64, yp: f64,  // Point
    angle_deg: f64     // Angle of the segment with respect to the horizontal axis in degrees
) -> Option<(f64, f64)> {
    // Convert angle to radians
    let angle_rad = angle_deg * PI / 180.0;

    // Rotate the segment endpoints and the point by the negative of the angle to align segment with horizontal axis
    let (x1_rot, y1_rot) = rotate_point(x1, y1, -angle_rad);
    let (x2_rot, _) = rotate_point(x2, y2, -angle_rad);
    let (xp_rot, _) = rotate_point(xp, yp, -angle_rad);

    // Calculate the intersection point in the rotated coordinate system
    let x_intersect = xp_rot;
    let y_intersect = y1_rot;

    // Check if the intersection point lies within the segment bounds
    if x_intersect >= x1_rot.min(x2_rot) && x_intersect <= x1_rot.max(x2_rot) {
        // Rotate the intersection point back to the original coordinate system
        let (xi, yi) = rotate_point_back(x_intersect, y_intersect, angle_rad);
        Some((xi, yi))
    } else {
        None // No valid intersection within the segment bounds
    }
}

/// Calculates the Euclidean distance between two points.
pub fn distance_between_points(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
}
