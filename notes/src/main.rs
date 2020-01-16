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

fn pattern_matching(s: Color) {
	if let Color::Custom(_, g, _) = s {
		println!("{:?}", g);
	}
}

fn numbers() {
	let mut x = 5.012f32;
	let y = (1, 'a', false);
	let z = x;
	x += 1.0;
	println!("{} {:?} {}", x, y, z);
}

fn arrays() {
	// Array initialization
	let a = [0u32; 3];
	println!("{:?} len: {}", a, a.len());
	// Or
	let a = [ 'a', 'b', 'c'];
	println!("{:?} len: {}", a, a.len());
	
	// Fat pointers
	let a: &[char] = &['a', 'b'];
	println!("{:?} len: {}", a, a.len());

	// Panic
	let a = [1, 2, 3];
	let _ar: &[u8] = &a;
	//println!("{}", ar[3]);
}

fn vectors() {
	let a = Vec::new();
	// Or
	let a = Vec::with_capacity(15);
}

fn main() {
	numbers();
	arrays();
	vectors();

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

	pattern_matching(c4);
}
