use crate::WINDOW_SIZE;

pub struct WindowBuffer<Data: Default + Copy> {
	buffer: [Data; WINDOW_SIZE],
	index: usize
}
impl<Data: Default + Copy> WindowBuffer<Data> {
	pub fn new() -> Self {
		WindowBuffer {
			buffer: [Default::default(); WINDOW_SIZE],
			index: 0
		}
	}

	pub fn push(&mut self, data: Data) {
		self.buffer[self.index] = data;
		self.index = (self.index + 1) % WINDOW_SIZE;
	}

	pub fn buffer(&self) -> &[Data; WINDOW_SIZE] {
		&self.buffer
	} 
}