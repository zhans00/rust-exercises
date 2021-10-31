use  std::f64::consts::PI;

#[derive(Debug)]
pub struct Circle {
	pub center: Point,//..
	pub radius: f64,//..
}
#[derive(Debug)]
pub struct Point {
	pub x: f64,
	pub y: f64,
}

impl Point {
	pub fn distance(&self, p2: &Point) -> f64 {
		return ((p2.x - self.x).powf(2.0) + (p2.y - self.y).powf(2.0)).sqrt();
	}

}

impl Circle {
// Circle
	pub fn new(x: f64, y: f64, radius: f64) -> Circle {
		return Circle { 
			center: Point { x: x, y :y},
			radius: radius,
		}
	}

	pub fn diameter(&self) -> f64 {
		return 2.0 * self.radius;
	}

	pub fn area(&self) -> f64 {
		return PI * self.radius.powf(2.0);
	}

	pub fn intersect(self, other: &Circle ) -> bool {
		let dist = self.center.distance(&(other.center));
		return (self.radius - other.radius).abs() <= dist && dist <= self.radius + other.radius;
	}
}