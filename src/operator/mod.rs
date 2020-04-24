pub mod filter;
pub mod normalizer;
pub mod scaler;
pub mod smoother;

pub use filter::HighPassFilter;
pub use normalizer::{AbsNormalizer, AvgNormalizer};
pub use scaler::{BinScaler, RangeScaler, UniformScaler};
pub use smoother::SpectrumSmoother;

pub trait SpectrumOperator {
	fn apply(&mut self, spectrum: &mut [crate::DataType]);
}
