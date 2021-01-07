use super::SpectrumOperator;

use crate::config::DataType;

/// Smooths the bins using `max_delta` tracking.
pub struct TrackingSmootherOperator<const SPECTRUM_BINS: usize> {
	current: [DataType; SPECTRUM_BINS],
	max_delta: DataType
}
impl<const SPECTRUM_BINS: usize> TrackingSmootherOperator<SPECTRUM_BINS> {
	pub const fn new(max_delta: DataType) -> Self {
		TrackingSmootherOperator {
			current: [0.0; SPECTRUM_BINS],
			max_delta
		}
	}
}
impl<const SPECTRUM_BINS: usize> SpectrumOperator<SPECTRUM_BINS> for TrackingSmootherOperator<SPECTRUM_BINS> {
	fn apply(&mut self, spectrum: &mut [DataType; SPECTRUM_BINS]) {
		for (spec, curr) in spectrum.iter_mut().zip(&mut self.current) {
			let diff = *spec - *curr;
			let track_diff = diff.abs().min(self.max_delta).copysign(diff);

			*curr += track_diff;
			*spec = *curr;
		}
	}
}