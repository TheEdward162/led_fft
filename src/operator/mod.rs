pub mod tracking_smoother;
pub mod bin_weights;
pub mod min_normalizer;
pub mod avg_normalizer;

use crate::config::DataType;

pub trait SpectrumOperator<const SPECTRUM_BINS: usize> {
	/// Applies the spectrum operator by mutating the spectrum slice.
	fn apply(&mut self, spectrum: &mut [DataType; SPECTRUM_BINS]);
}
impl<const SPECTRUM_BINS: usize> SpectrumOperator<SPECTRUM_BINS> for () {
	fn apply(&mut self, _spectrum: &mut [DataType; SPECTRUM_BINS]) {}
}

macro_rules! impl_spectrum_operator_tuple {
	(
		$(
			$idx: tt -> $T: ident,
		)+
	) => {
		impl<$($T: SpectrumOperator<SPECTRUM_BINS>,)+ const SPECTRUM_BINS: usize> SpectrumOperator<SPECTRUM_BINS> for ($($T,)+) {
			fn apply(&mut self, spectrum: &mut [DataType; SPECTRUM_BINS]) {
				$(
					self.$idx.apply(spectrum);
				)+
			}
		}
	};
}
impl_spectrum_operator_tuple! {
	0 -> A,
}
impl_spectrum_operator_tuple! {
	0 -> A,
	1 -> B,
}
impl_spectrum_operator_tuple! {
	0 -> A,
	1 -> B,
	2 -> C,
}
impl_spectrum_operator_tuple! {
	0 -> A,
	1 -> B,
	2 -> C,
	3 -> D,
}
impl_spectrum_operator_tuple! {
	0 -> A,
	1 -> B,
	2 -> C,
	3 -> D,
	4 -> E,
}
impl_spectrum_operator_tuple! {
	0 -> A,
	1 -> B,
	2 -> C,
	3 -> D,
	4 -> E,
	5 -> F,
}