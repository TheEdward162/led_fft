use super::SpectrumOperator;

use crate::config::DataType;

pub struct MinNormalizerOperator;
impl<const SPECTRUM_BINS: usize> SpectrumOperator<SPECTRUM_BINS> for MinNormalizerOperator {
	fn apply(&mut self, spectrum: &mut [DataType; SPECTRUM_BINS]) {
		let mut min = spectrum[0];
		for &s in spectrum.iter() {
			if min.is_infinite() || (s.is_finite() && s < min) {
				min = s;
			}
		}
		if min.is_infinite() {
			return;
		}

		spectrum.iter_mut().for_each(
			|s| *s -= min
		);
	}
}