use rustfft::{algorithm::Radix4, num_complex::Complex, num_traits::Zero, FFT};

use crate::{DataType, BIN_SIZE, SPECTRUM_BINS, WINDOW_SIZE};

pub struct FFTProcessor {
	input: [Complex<DataType>; WINDOW_SIZE],
	output: [Complex<DataType>; WINDOW_SIZE],

	fft: Radix4<DataType>,

	spectrum_bins: [DataType; SPECTRUM_BINS]
}
impl FFTProcessor {
	pub fn new() -> Self {
		FFTProcessor {
			input: [Complex::zero(); WINDOW_SIZE],
			output: [Complex::zero(); WINDOW_SIZE],

			fft: Radix4::new(WINDOW_SIZE, false),

			spectrum_bins: [Default::default(); SPECTRUM_BINS]
		}
	}

	pub fn process(&mut self, data: impl ExactSizeIterator<Item = DataType>) {
		assert_eq!(data.len(), WINDOW_SIZE);

		for (i, d) in data.enumerate() {
			self.input[i] = Complex::new(d, 0.0);
		}

		self.fft.process(&mut self.input, &mut self.output);
	}

	pub fn output<'a>(&'a self) -> impl ExactSizeIterator<Item = DataType> + 'a {
		self.output.iter().map(Complex::norm)
	}

	pub fn calculate_spectrum_bins(&mut self) {
		for bin in 0 .. SPECTRUM_BINS {
			let column_begin = (bin + 1) * BIN_SIZE;
			let column_end = (bin + 2) * BIN_SIZE;

			let bin_value: DataType = self.output[column_begin .. column_end]
				.iter()
				.map(Complex::norm)
				.sum();

			self.spectrum_bins[bin] = bin_value;
		}
	}

	pub fn spectrum_bins(&self) -> &[DataType; SPECTRUM_BINS] {
		&self.spectrum_bins
	}
}
