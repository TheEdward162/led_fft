use crate::operator::SpectrumOperator;

/// Scales each bin using the same factor.
pub struct UniformScaler {
	pub factor: crate::DataType
}
impl SpectrumOperator for UniformScaler {
	fn apply(&mut self, spectrum: &mut [crate::DataType]) {
		for value in spectrum.iter_mut() {
			*value *= self.factor;
		}
	}
}
