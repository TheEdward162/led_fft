use super::SpectrumOperator;

use crate::config::DataType;

pub struct AvgNormalizerOperator;
impl<const SPECTRUM_BINS: usize> SpectrumOperator<SPECTRUM_BINS> for AvgNormalizerOperator {
	fn apply(&mut self, spectrum: &mut [DataType; SPECTRUM_BINS]) {
		let mut sum: DataType = 0.0;
		let mut count = 0;
		
		for &s in spectrum.iter() {
			if s.is_finite() {
				sum += s;
				count += 1;
			}
		}
		if count == 0 {
			return;
		}

		let avg = sum / count as DataType;
		spectrum.iter_mut().for_each(
			|s| *s -= avg
		);
	}
}