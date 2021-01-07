use crate::{
	util::window_buffer::WindowBuffer,
	config::DataType,
	sound::SoundSink
};

use crate::{operator::SpectrumOperator, output::OutputHandler};

/// Application context.
///
/// Holds all the buffers and objects that live per-application.
pub struct Context<
	Operator: SpectrumOperator<SPECTRUM_BINS>, Output: OutputHandler<SPECTRUM_BINS>,
	const WINDOW_SIZE: usize, const SPECTRUM_BINS: usize, const UPDATE_FRAMES: usize
> {
	window_envelope: [DataType; WINDOW_SIZE],
	/// The window buffer that holds the last `WINDOW_SIZE` frames.
	window: WindowBuffer<DataType, WINDOW_SIZE>,
	/// Counter used to keep track of how many frames have been inserted since the last recalculation and output.
	window_frame_counter: usize,

	fft: fft::FftProcessor<WINDOW_SIZE, SPECTRUM_BINS>,

	/// Operator applied at each update.
	operator: Operator,
	/// Output run at each update.
	output: Output
}
impl<
	Operator: SpectrumOperator<SPECTRUM_BINS>, Output: OutputHandler<SPECTRUM_BINS>,
	const WINDOW_SIZE: usize, const SPECTRUM_BINS: usize, const UPDATE_FRAMES: usize
> Context<Operator, Output, WINDOW_SIZE, SPECTRUM_BINS, UPDATE_FRAMES> {
	pub fn new(
		operator: Operator,
		output: Output
	) -> Self {
		// Precalculate the envelope
		let mut window_envelope = [DataType::default(); WINDOW_SIZE];
		window_envelope.iter_mut().enumerate().for_each(
			|(index, env)| *env = crate::config::window_envelope_function::<WINDOW_SIZE>(index)
		);

		Context {
			window_envelope,
			window: WindowBuffer::new(),
			window_frame_counter: 0,

			fft: fft::FftProcessor::new(),

			operator,
			output
		}
	}

	pub fn process_input(&mut self, input: impl Iterator<Item = DataType>) {
		input.for_each(
			|frame| {
				self.window.push(frame);
				self.window_frame_counter += 1;

				if self.window_frame_counter == UPDATE_FRAMES {
					self.window_frame_counter = 0;

					self.recalculate();
					self.output();
				}
			}
		);
	}

	fn recalculate(&mut self) {
		self.fft.process(
			self.window.iter_ordered().zip(self.window_envelope.iter()).map(
				|(frame, envelope)| frame * envelope
			)
		);
	}

	/// Applies operator to the last output and runs the output handler.
	///
	/// This method copies the spectrum bins buffer and can be called repeatedly.
	fn output(&mut self) {
		let mut spectrum_bins = *self.fft.spectrum_bins();

		self.operator.apply(&mut spectrum_bins);
		self.output.handle_output(&spectrum_bins);
	}
}
impl<
	Operator: SpectrumOperator<SPECTRUM_BINS>, Output: OutputHandler<SPECTRUM_BINS>,
	const WINDOW_SIZE: usize, const SPECTRUM_BINS: usize, const UPDATE_FRAMES: usize
> SoundSink for Context<Operator, Output, WINDOW_SIZE, SPECTRUM_BINS, UPDATE_FRAMES> {
	fn process_input(&mut self, input: impl Iterator<Item = crate::config::DataType>) {
		self.process_input(input)
	}
}

mod fft {
	use rustfft::{algorithm::Radix4, num_complex::Complex, num_traits::Zero, Fft, FftDirection};

	use crate::config::{DataType, WINDOW_EVELOPE_VARW};

	/// Wraps the fft implementation nad buffers.
	pub struct FftProcessor<const WINDOW_SIZE: usize, const SPECTRUM_BINS: usize> {
		data: [Complex<DataType>; WINDOW_SIZE],
		scratch: [Complex<DataType>; WINDOW_SIZE],
		fft: Radix4<DataType>,
		spectrum_bins: [DataType; SPECTRUM_BINS]
	}
	impl<const WINDOW_SIZE: usize, const SPECTRUM_BINS: usize> FftProcessor<WINDOW_SIZE, SPECTRUM_BINS> {
		const BIN_SIZE: usize = WINDOW_SIZE / ((SPECTRUM_BINS + 1) * 2);

		pub fn new() -> Self {
			FftProcessor {
				data: [Complex::zero(); WINDOW_SIZE],
				scratch: [Complex::zero(); WINDOW_SIZE],
				fft: Radix4::new(WINDOW_SIZE, FftDirection::Forward),
				spectrum_bins: [Default::default(); SPECTRUM_BINS]
			}
		}

		pub fn process(&mut self, data: impl Iterator<Item = DataType>) {
			debug_assert_eq!(data.size_hint(), (WINDOW_SIZE, Some(WINDOW_SIZE)));

			// Fill in the input array by the data.
			self.data.iter_mut().zip(data).for_each(
				|(input, dat)| *input = Complex::new(dat, DataType::default())
			);

			self.fft.process_with_scratch(&mut self.data, &mut self.scratch);

			// Process output using magic and compute spectral bins.
			self.data.chunks(Self::BIN_SIZE).zip(self.spectrum_bins.iter_mut()).for_each(
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
}