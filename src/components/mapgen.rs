use super::data::{Point};
use std::num;

// TODO: make this produce an iterator or something
// Bresenham's line algorithm
pub fn make_line(start: &Point, end: &Point) -> Vec<Point> {
    let Δx: f32 = end.x as f32 - start.x as f32;
    let Δy: f32 = end.y as f32 - start.y as f32;
    let Δerr = (Δy/Δx).abs();
    let mut err: f32 = 0.0;
    let mut points: Vec<Point> = Vec::new();
    let mut y = start.y;

    for x in start.x..=end.x {
        points.push(Point{x: x, y: y});
        err += Δerr;
        if err >= 0.5 {
            y += Δy.signum() as i32;
            err -= 1.0;
        }
    }

    return points
}

pub fn make_hollow_rectange(top_left: &Point, bottom_right: &Point) -> Vec<Point> {
    let mut points: Vec<Point> = Vec::new();
    points.append(&mut make_line(top_left, &Point{x: bottom_right.x, y: top_left.y}));
    points.append(&mut make_line(&Point{x: top_left.x, y: top_left.y + 1}, &Point{x: top_left.x, y: bottom_right.y}));
    points.append(&mut make_line(&Point{x: top_left.x + 1, y: bottom_right.y}, &Point{x: top_left.x + 1, y: bottom_right.y}));
    points.append(&mut make_line(&Point{x: bottom_right.x, y: top_left.y + 1}, &Point{x: bottom_right.x, y: bottom_right.y - 1}));
    return points
}