#[derive(Debug)]
pub enum Color {
	Red,
	Green,
	Blue,
	Custom(f32, f32, f32),
}

#[derive(Debug, Clone, Copy)]
struct Point {
	x: u64,
	y: u64,
}

impl Point {
	fn swp(&mut self) {
		self.x += self.y;
		self.y += self.x;
	}

	fn show_point(&self) {
		println!("{:?}", self);
	}
}

fn pattern_match(s: Color) {
	if let Color::Custom(_, g, _) = s {
		println!("{:?}", g);
	}
}

fn nums_1() {
	let mut x = 5.012f32;
	let y = (1, 'a', false);
	let z = x;
	x += 1.0;
	println!("{} {:?} {}", x, y, z);
}

fn main() {
	nums_1();

	let mut p = Point { x:12, y: 20 };
	p.show_point();
	p.swp();
	p.show_point();
	let t = p.clone();
	t.show_point();

	let c1: Color = Color::Red;
	let c2: Color = Color::Green;
	let c3: Color = Color::Blue;
	let c4: Color = Color::Custom(0.8, 0.7, 0.6);
	println!("{:?} {:?} {:?}\n", c1, c2, c3);

	pattern_match(c4);
}
