use super::SpectrumOperator;

pub struct RangeScaling {
	pub range: std::ops::Range<usize>,
	pub factor: crate::DataType
}
pub enum SpectrumScaler {
	Uniform(crate::DataType),
	Ranges(Vec<RangeScaling>),
	Bins([crate::DataType; crate::SPECTRUM_BINS])
}
impl SpectrumScaler {
	pub fn new_interpolated(factors: Vec<crate::DataType>) -> Self {
		if factors.len() < 2 {
			return SpectrumScaler::Uniform(factors[0]);
		}

		let mut final_factors = [Default::default(); crate::SPECTRUM_BINS];

		let index_step = (crate::SPECTRUM_BINS - 1) as f32 / (factors.len() - 1) as f32;
		for (index, final_factor) in final_factors.iter_mut().enumerate() {
			let index_f32 = index as f32;
			let previous_index = (index_f32 / index_step).floor();
			let next_index = (index_f32 / index_step).ceil();
			let interpolation_factor = index_f32 / index_step - previous_index;
			
			*final_factor = factors[next_index as usize] * interpolation_factor + factors[previous_index as usize] * (1.0 - interpolation_factor);
		}
		
		{
			log::debug!("Interpolating factors: {:?}", factors);
			const HEIGHT: usize = 10;
			
			let mut min_factor = std::f32::MAX;
			let mut max_factor = std::f32::MIN;
			final_factors.iter().for_each(|factor| {
				if *factor < min_factor {
					min_factor = *factor;
				}
				if *factor > max_factor {
					max_factor = *factor;
				}
			});
			for row in 0 .. HEIGHT {
				let mut row_string = String::new();
				
				for bin_factor in final_factors.iter() {
					let normalized_factor = (bin_factor - min_factor) / (max_factor - min_factor);
					if normalized_factor >= (HEIGHT - row) as f32 / HEIGHT as f32 {
						row_string.push('x');
					} else {
						row_string.push(' ');
					}
				}

				log::debug!("[{}]", row_string);
			}
		}

		SpectrumScaler::Bins(final_factors)
	}
}
impl SpectrumOperator for SpectrumScaler {
	fn apply(&mut self, spectrum: &mut [crate::DataType]) {
		match self {
			SpectrumScaler::Uniform(factor) => {
				for value in spectrum.iter_mut() {
					*value *= *factor;
				}
			}
			SpectrumScaler::Ranges(ranges) => {
				for range in ranges.iter() {
					for index in range.range.clone() {
						spectrum[index] *= range.factor;
					}
				}
			}
			SpectrumScaler::Bins(factors) => {
				for (index, value) in spectrum.iter_mut().enumerate() {
					*value *= factors[index];
				}
			}
		}
	}
}
