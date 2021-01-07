use super::SpectrumOperator;

use crate::config::DataType;

pub struct BinWeightsOperator<const SPECTRUM_BINS: usize> {
	weights: [DataType; SPECTRUM_BINS]
}
impl<const SPECTRUM_BINS: usize> BinWeightsOperator<SPECTRUM_BINS> {
	pub const fn constant(v: DataType) -> Self {
		BinWeightsOperator {
			weights: [v; SPECTRUM_BINS]
		}
	}

	/// Interpolates stop using function `f`.
	///
	/// `f` shall map from `[0.0; 1.0]` to `[0.0; 1.0]`.
	pub fn interpolate_stops(
		mut stops: impl Iterator<Item = (usize, DataType)>,
		f: impl Fn(DataType) -> DataType
	) -> Self {
		// TODO: assert there is at least one stop
		// TODO: assert stop indices are in ascending order and in range [0.0; 1.0]

		let mut weights = [0.0; SPECTRUM_BINS];
		let mut i = 0;

		let mut current_stop = stops.next().expect("at least one stop is required");
		debug_assert!(current_stop.0 < SPECTRUM_BINS);

		// before the first stop
		while i <= current_stop.0 {
			weights[i] = current_stop.1;
			i += 1;
		}

		// actual interpolation
		let mut next_stop = stops.next().unwrap_or(current_stop);
		loop {
			if i == next_stop.0 {
				current_stop = next_stop;
				match stops.next() {
					Some(s) => {
						next_stop = s;
					}
					None => break
				};
			}

			// relative factor in `[0.0; 1.0]` between `current_stop` and `next_stop`
			let t = (i - current_stop.0) as DataType / (next_stop.0 - current_stop.0) as DataType;
			// compute the factor using the provided `f`
			// use `1 - t` if the value is descending
			let factor = if current_stop.1 < next_stop.1 {
				f(t)
			} else {
				f(1.0 - t)
			};

			weights[i] = current_stop.1 * (1.0 - factor) + next_stop.1 * factor;

			i += 1;
		}

		// after the last stop
		while i < SPECTRUM_BINS {
			weights[i] = next_stop.1;
			i += 1;
		}

		log::debug!("Interpolated weights: {:?} {}", weights, SPECTRUM_BINS);

		BinWeightsOperator {
			weights
		}
	}
}
impl<const SPECTRUM_BINS: usize> SpectrumOperator<SPECTRUM_BINS> for BinWeightsOperator<SPECTRUM_BINS> {
	fn apply(&mut self, spectrum: &mut [DataType; SPECTRUM_BINS]) {
		spectrum.iter_mut().zip(self.weights.iter()).for_each(
			|(s, w)| *s *= w
		);
	}
}