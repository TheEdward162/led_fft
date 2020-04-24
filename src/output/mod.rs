use crate::DataType;

pub mod serial;
pub mod text;

pub trait OutputHandler {
	fn handle_output(&mut self, spectrum: &[crate::DataType]);
}

pub struct ColorOutputInfo {
	pub bin_range: std::ops::Range<usize>,
	pub base_value: u8,
	pub mult_value: DataType,
	pub param_fn: crate::parametrization::ParamFn
}
impl ColorOutputInfo {
	pub fn parametrized_value(&self, spectrum: &[DataType]) -> DataType {
		let sum: DataType = spectrum[self.bin_range.clone()].iter().sum();
		self.param_fn.apply(sum)
	}

	pub fn compute_value(&self, spectrum: &[DataType]) -> u8 {
		(self.parametrized_value(spectrum) * self.mult_value) as u8 + self.base_value
	}
}
