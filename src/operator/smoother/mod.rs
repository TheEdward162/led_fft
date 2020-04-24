use super::SpectrumOperator;
use crate::{util::RunningAverage, DataType};

pub struct SpectrumSmoother {
	bin_avg: Vec<RunningAverage<DataType>>
}
impl SpectrumSmoother {
	pub fn new(smooth_history: usize) -> Self {
		let mut bin_avg = Vec::with_capacity(crate::SPECTRUM_BINS);
		for _ in 0 .. crate::SPECTRUM_BINS {
			bin_avg.push(RunningAverage::new(smooth_history));
		}
		SpectrumSmoother { bin_avg }
	}
}
impl SpectrumOperator for SpectrumSmoother {
	fn apply(&mut self, spectrum: &mut [crate::DataType]) {
		for (index, bin) in spectrum.iter_mut().enumerate() {
			*bin = self.bin_avg[index].update(*bin);
		}
	}
}
