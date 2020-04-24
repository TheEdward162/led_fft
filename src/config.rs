// use serde::{Serialize, Deserialize};

// #[derive(Serialize, Deserialize)]
// pub struct Config {

// }

use crate::{output::ColorOutputInfo, parametrization::ParamFn};

pub const RED_DEFAULT_INFO: ColorOutputInfo = ColorOutputInfo {
	bin_range: 2 .. 9,
	base_value: 0,
	mult_value: 64.0,
	param_fn: ParamFn::Logarithmic
};
pub const GREEN_DEFAULT_INFO: ColorOutputInfo = ColorOutputInfo {
	bin_range: 14 .. 26,
	base_value: 0,
	mult_value: 64.0,
	param_fn: ParamFn::Exponential
};
pub const BLUE_DEFAULT_INFO: ColorOutputInfo = ColorOutputInfo {
	bin_range: 30 .. 40,
	base_value: 0,
	mult_value: 64.0,
	param_fn: ParamFn::SquareRoot
};
