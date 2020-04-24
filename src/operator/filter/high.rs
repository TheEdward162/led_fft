use crate::{operator::SpectrumOperator, DataType};

pub struct HighPassFilter {
	pub min_value: DataType
}
impl HighPassFilter {
	pub fn new(min_value: DataType) -> Self {
		HighPassFilter { min_value }
	}
}
impl SpectrumOperator for HighPassFilter {
	fn apply(&mut self, spectrum: &mut [DataType]) {
		for value in spectrum.iter_mut() {
			if *value < self.min_value {
				*value = 0.0;
			}
		}
	}
}
