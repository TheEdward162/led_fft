// macro_rules! operator_chain {
// 	(
// 		$struct_name: ident
// 	) => {
// 		pub $struct_name {

// 		}
// 		impl<const BINS: usize> SpectrumOperator<BINS> for $struct_name {

// 		}
// 	};
// };

// pub mod filter;
// pub mod normalizer;
// pub mod scaler;
// pub mod smoother;

// pub use filter::HighPassFilter;
// pub use normalizer::{AbsNormalizer, AvgNormalizer};
// pub use scaler::{BinScaler, RangeScaler, UniformScaler};
// pub use smoother::SpectrumSmoother;

pub trait SpectrumOperator<const SPECTRUM_BINS: usize> {
	/// Applies the spectrum operator by mutating the spectrum slice.
	fn apply(&mut self, spectrum: &mut [crate::config::DataType; SPECTRUM_BINS]);
}
impl<const SPECTRUM_BINS: usize> SpectrumOperator<SPECTRUM_BINS> for () {
	fn apply(&mut self, _spectrum: &mut [crate::config::DataType; SPECTRUM_BINS]) {}
}