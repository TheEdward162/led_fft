use crate::{operator::SpectrumOperator, parametrization::ParamFn, DataType};

pub struct AbsNormalizer {
	pub min: DataType,
	pub max: DataType,
	pub param_fn: ParamFn
}
impl SpectrumOperator for AbsNormalizer {
	fn apply(&mut self, spectrum: &mut [crate::DataType]) {
		let len = spectrum.len() as DataType;
		for value in spectrum.iter_mut() {
			let clamped = value.max(self.min).min(self.max);
			let abs_normalized = (clamped - self.min) / (self.max - self.min);
			let parametrized = self.param_fn.apply(abs_normalized);

			*value = parametrized / len;
		}
	}
}
