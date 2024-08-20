use std::ops::Add;

#[derive(Copy, Clone)]
pub struct Point(pub i32, pub i32);

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl PartialEq<Point> for Point {
    fn eq(self: &Point, rhs: &Point) -> bool {
        return self.0 == rhs.0 && self.1 == rhs.1;
    }
}
