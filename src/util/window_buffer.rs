/// A fixed-size ring buffer for `Copy` data (i.e. numbers).
pub struct WindowBuffer<Data: Default + Copy, const SIZE: usize> {
	buffer: [Data; SIZE],
	index: usize
}
impl<Data: Default + Copy, const SIZE: usize> WindowBuffer<Data, SIZE> {
	pub fn new() -> Self {
		WindowBuffer {
			buffer: [Default::default(); SIZE],
			index: 0
		}
	}

	pub fn push(&mut self, data: Data) {
		self.buffer[self.index] = data;
		self.index = (self.index + 1) % SIZE;
	}

	// pub fn buffer(&self) -> &[Data; SIZE] {
	// 	&self.buffer
	// }

	/// Returns an iterator of the elements in insertion order.
	pub fn iter_ordered(&self) -> impl Iterator<Item = Data> + '_ {
		self.buffer.iter().skip(self.index).chain(self.buffer.iter()).take(SIZE).copied()
	}
}
