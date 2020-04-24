use crate::operator::SpectrumOperator;

/// Scales each bin individually.
pub struct BinScaler {
	pub factors: [crate::DataType; crate::SPECTRUM_BINS]
}
impl SpectrumOperator for BinScaler {
	fn apply(&mut self, spectrum: &mut [crate::DataType]) {
		for (index, value) in spectrum.iter_mut().enumerate() {
			*value *= self.factors[index];
		}
	}
}
