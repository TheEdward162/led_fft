use rustfft::{algorithm::Radix4, num_complex::Complex, num_traits::Zero, FFT};

use crate::config::{DataType, WINDOW_EVELOPE_VARW};

pub struct FFTProcessor<const WINDOW_SIZE: usize, const SPECTRUM_BINS: usize> {
	input: [Complex<DataType>; WINDOW_SIZE],
	output: [Complex<DataType>; WINDOW_SIZE],
	fft: Radix4<DataType>,
	spectrum_bins: [DataType; SPECTRUM_BINS]
}
impl<const WINDOW_SIZE: usize, const SPECTRUM_BINS: usize> FFTProcessor<WINDOW_SIZE, SPECTRUM_BINS> {
	const BIN_SIZE: usize = WINDOW_SIZE / ((SPECTRUM_BINS + 1) * 2);

	pub fn new() -> Self {
		FFTProcessor {
			input: [Complex::zero(); WINDOW_SIZE],
			output: [Complex::zero(); WINDOW_SIZE],
			fft: Radix4::new(WINDOW_SIZE, false),
			spectrum_bins: [Default::default(); SPECTRUM_BINS]
		}
	}

	pub fn process(&mut self, data: impl Iterator<Item = DataType>) {
		debug_assert_eq!(data.size_hint(), (WINDOW_SIZE, Some(WINDOW_SIZE)));

		// Fill in the input array by the data.
		self.input.iter_mut().zip(data).for_each(
			|(input, dat)| *input = Complex::new(dat, DataType::default())
		);

		self.fft.process(&mut self.input, &mut self.output);

		// Process output using magic and compute spectral bins.
		self.output.chunks(Self::BIN_SIZE).zip(self.spectrum_bins.iter_mut()).for_each(
			|(chunk, bin)| {
				*bin = chunk.iter().map(|n| {
					let normed = n.norm() / WINDOW_SIZE as DataType;
					(normed.abs().powi(2) / WINDOW_EVELOPE_VARW).log10()
				}).sum();
			}
		);
	}

	pub fn spectrum_bins(&self) -> &[DataType; SPECTRUM_BINS] {
		&self.spectrum_bins
	}
}