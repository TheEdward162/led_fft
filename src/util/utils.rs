use std::ops::{Add, AddAssign, Sub, Mul, Div, MulAssign};

pub trait SmoothedValue: Add + AddAssign + MulAssign + Sub<Output = Self> + Mul + Div<Output = Self> + Copy + Clone + std::fmt::Display {}
impl SmoothedValue for i8 {}
impl SmoothedValue for i16 {}
impl SmoothedValue for i32 {}
impl SmoothedValue for i64 {}
impl SmoothedValue for f32 {}
impl SmoothedValue for f64 {}

pub struct MomentumSmoother<V: SmoothedValue> {
	pub current: V,

	pub weight: V,
	pub momentum: V,
	pub momentum_decay_factor: V
}
impl<V: SmoothedValue> MomentumSmoother<V> {
	pub fn apply_force(&mut self, force: V) {
		self.momentum += force / self.weight;
	}

	pub fn track_target(&mut self, target: V) {
		let force = target - self.current;
		self.apply_force(force);
	}

	pub fn update(&mut self) {
		self.current += self.momentum;
		self.momentum *= self.momentum_decay_factor;

		log::trace!("MomentumSmoother({}, {})", self.current, self.momentum);
	}
}