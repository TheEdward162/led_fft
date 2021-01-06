pub trait SpectrumOperator<const SPECTRUM_BINS: usize> {
	/// Applies the spectrum operator by mutating the spectrum slice.
	fn apply(&mut self, spectrum: &mut [crate::config::DataType; SPECTRUM_BINS]);
}
impl<const SPECTRUM_BINS: usize> SpectrumOperator<SPECTRUM_BINS> for () {
	fn apply(&mut self, _spectrum: &mut [crate::config::DataType; SPECTRUM_BINS]) {}
}