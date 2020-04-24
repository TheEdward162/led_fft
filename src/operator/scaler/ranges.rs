use crate::operator::SpectrumOperator;

pub struct RangeScaling {
	pub range: std::ops::Range<usize>,
	pub factor: crate::DataType
}
/// Scales bins by ranges.
pub struct RangeScaler {
	pub ranges: Vec<RangeScaling>
}
impl SpectrumOperator for RangeScaler {
	fn apply(&mut self, spectrum: &mut [crate::DataType]) {
		for range in self.ranges.iter() {
			for index in range.range.clone() {
				spectrum[index] *= range.factor;
			}
		}
	}
}
