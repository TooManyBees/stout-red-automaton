use std::ops::Deref;
use rand::Rng;

pub struct Kernel1D([usize; 3]);

impl Kernel1D {
	pub fn new(x: usize, size: usize) -> Self {
		Kernel1D([
			if x == 0 { size - 1 } else { x - 1 },
			x,
			if x == size - 1 { 0 } else { x + 1 },
		])
	}
}

pub struct Kernel2D([usize; 9]);

impl Kernel2D {
	pub fn new(x: usize, y: usize, size: usize) -> Self {
		let left = if x == 0 { size - 1 } else { x - 1 };
		let right = if x == size - 1 { 0 } else { x + 1 };
		let top = if y == 0 { size - 1 } else { y - 1} * size;
		let middle = y * size;
		let bottom = if y == size - 1 { 0 } else { y + 1 } * size;

		Kernel2D([
			top + left,    top + x,    top + right,
			middle + left, middle + x, middle + right,
			bottom + left, bottom + x, bottom + right,
		])
	}
}

impl Deref for Kernel1D {
	type Target = [usize];

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl Deref for Kernel2D {
	type Target = [usize];

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

pub trait Kernel: Deref<Target=[usize]> {
	fn eval(&self, space: &[bool]) -> u8 {
		let mut val = 0u8;
		for &idx in Deref::deref(self) {
			val = val << 1;
			if space[idx] {
				val |= 1;
			}
		}
		val
	}
}

impl Kernel for Kernel1D {}
impl Kernel for Kernel2D {}

pub struct Automata {
	size: usize,
	generations: Vec<Vec<bool>>,
	f: fn(u8) -> bool,

}

impl Automata {
	pub fn new(size: usize, f: fn(u8) -> bool) -> Self {
		Automata {
			size,
			generations: vec![],
			f,
		}
	}

	pub fn with_seed(size: usize, f: fn(u8) -> bool, seed: Vec<bool>) -> Self {
		assert!(seed.len() == size, "Seed generation does not match size of the automata's space");
		Automata {
			size,
			generations: vec![seed],
			f,
		}
	}

	pub fn size(&self) -> usize {
		self.size
	}

	pub fn new_generation(&self) -> Vec<bool> {
		self.generations.last().map(|old_generation| {
			(0..self.size).map(|x| {
				let kernel = Kernel1D::new(x, self.size);
				(self.f)(kernel.eval(&old_generation))
			}).collect()
		}).unwrap_or_else(|| {
			let mut rng = rand::thread_rng();
			(0..self.size).map(|_| rng.gen()).collect()
		})
	}

	pub fn advance(&mut self) {
		let new_generation = self.new_generation();
		self.generations.push(new_generation);
	}

	pub fn generations(&self) -> impl Iterator<Item=&[bool]> {
		self.generations.iter().map(|v| v.as_slice())
	}
}
