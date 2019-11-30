use crate::DataType;
use super::{SpectrumOperator, RunningAverage};

pub struct SpectrumNormalizer {
	max_bin_avg: RunningAverage<DataType>
}
impl SpectrumNormalizer {
	pub fn new(history_len: usize) -> Self {
		SpectrumNormalizer {
			max_bin_avg: RunningAverage::new(history_len)
		}
	}
}
impl SpectrumOperator for SpectrumNormalizer {
	fn apply(&mut self, spectrum: &mut [crate::DataType]) {
		let max_bin = spectrum.iter().fold(0.0, |acc, val| {
			if *val > acc {
				*val
			} else {
				acc
			}
		});
		let max_bin = self.max_bin_avg.update(max_bin);

		for value in spectrum.iter_mut() {
			*value /= max_bin;
		}
	}
}