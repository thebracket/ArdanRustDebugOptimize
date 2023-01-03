#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

type Angle = f32;

fn project_angle(start: &Point, radius: f32, angle: Angle) -> Point
{
    Point {
        x: (0.0 - (start.x as f32 + radius * f32::sin(angle))) as i32,
        y: (start.y as f32 + radius * f32::cos(angle)) as i32,
    }
}

fn main() {
    let start = Point{ x: 0, y: 0};
    let end = project_angle(&start, 10.0, 90.0);
    //let end = project_angle(&start, 10.0, (90.0f32).to_radians());
    println!("{:#?}", end);
}
