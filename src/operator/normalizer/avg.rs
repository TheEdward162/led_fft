use crate::{operator::SpectrumOperator, util::RunningAverage, DataType};

pub struct AvgNormalizer {
	bin_sum_average: RunningAverage<DataType>
}
impl AvgNormalizer {
	pub fn new(history_len: usize) -> Self {
		AvgNormalizer {
			bin_sum_average: RunningAverage::new(history_len)
		}
	}
}
impl SpectrumOperator for AvgNormalizer {
	fn apply(&mut self, spectrum: &mut [crate::DataType]) {
		let sum = spectrum.iter().sum();
		let sum_avg = self.bin_sum_average.update(sum).max(1.0);

		for value in spectrum.iter_mut() {
			*value /= sum_avg;
		}
	}
}
