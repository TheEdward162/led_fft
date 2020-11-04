use crate::config::DataType;

pub enum ParamFn {
	Linear,
	LinearInverse,
	Quadratic,
	SquareRoot,
	Logarithmic,
	Exponential
}
impl ParamFn {
	pub fn apply(&self, value: DataType) -> DataType {
		match self {
			ParamFn::Linear => value,
			ParamFn::LinearInverse => 1.0 - value,
			ParamFn::Quadratic => value * value,
			ParamFn::SquareRoot => value.sqrt(),
			ParamFn::Logarithmic => (value + 1.0).ln(),
			ParamFn::Exponential => value.exp() - 1.0
		}
	}
}
