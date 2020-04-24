use std::ops::{Add, Div};

pub struct RunningAverage<D: Add<D, Output = D> + Div<D, Output = D> + Default + Copy + From<u16>> {
	index: usize,
	array: Vec<D>
}
impl<D: Add<D, Output = D> + Div<D, Output = D> + Default + Copy + From<u16>> RunningAverage<D> {
	pub fn new(len: usize) -> Self {
		let mut array = Vec::with_capacity(len);
		for _ in 0 .. len {
			array.push(Default::default());
		}

		RunningAverage { index: 0, array }
	}

	pub fn update(&mut self, value: D) -> D {
		self.array[self.index] = value;
		self.index = (self.index + 1) % self.array.len();

		self.array.iter().fold(D::default(), |acc, val| acc + *val)
			/ D::from(self.array.len() as u16)
	}
}
