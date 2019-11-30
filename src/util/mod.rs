pub mod running_average;

pub mod normalizer;
pub mod scaler;
pub mod smoother;

pub use running_average::RunningAverage;
pub use normalizer::SpectrumNormalizer;
pub use scaler::SpectrumScaler;
pub use smoother::SpectrumSmoother;

pub trait SpectrumOperator {
	fn apply(&mut self, spectrum: &mut [crate::DataType]);
}