#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

struct Radians(f32);

struct Degrees(f32);

impl From<Radians> for Degrees {
    fn from(n: Radians) -> Self {
        Self(n.0.to_degrees())
    }
}

impl From<Degrees> for Radians {
    fn from(n: Degrees) -> Self {
        Self(n.0.to_radians())
    }
}

fn project_angle<ANGLE>(start: &Point, radius: f32, angle: ANGLE) -> Point
where ANGLE: Into<Radians>
{
    let radians = angle.into();
    Point {
        x: (0.0 - (start.x as f32 + radius * f32::sin(radians.0))) as i32,
        y: (start.y as f32 + radius * f32::cos(radians.0)) as i32,
    }
}

fn main() {
    let start = Point{ x: 0, y: 0};
    let end = project_angle(&start, 10.0, Degrees(90.0));
    println!("{:#?}", end);
}
