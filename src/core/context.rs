use crate::{
	core::fft_processor::FFTProcessor,
	util::window_buffer::WindowBuffer,
	config::DataType,
	sound::SoundSink
};

use crate::{operator::SpectrumOperator, output::OutputHandler};

/// Application context.
pub struct Context<
	Operator: SpectrumOperator<SPECTRUM_BINS>, Output: OutputHandler<SPECTRUM_BINS>,
	const WINDOW_SIZE: usize, const SPECTRUM_BINS: usize, const UPDATE_FRAMES: usize
> {
	window_envelope: [DataType; WINDOW_SIZE],
	/// The window buffer that holds the last `WINDOW_SIZE` frames.
	window: WindowBuffer<DataType, WINDOW_SIZE>,
	/// Counter used to keep track of how many frames have been inserted since the last recalculation and output.
	window_frame_counter: usize,

	fft: FFTProcessor<WINDOW_SIZE, SPECTRUM_BINS>,

	operator: Operator,
	outputs: Output
}
impl<
	Operator: SpectrumOperator<SPECTRUM_BINS>, Output: OutputHandler<SPECTRUM_BINS>,
	const WINDOW_SIZE: usize, const SPECTRUM_BINS: usize, const UPDATE_FRAMES: usize
> Context<Operator, Output, WINDOW_SIZE, SPECTRUM_BINS, UPDATE_FRAMES> {
	pub fn new(
		operator: Operator,
		outputs: Output
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

			fft: FFTProcessor::new(),

			operator,
			outputs
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

	// /// Calculates frequency of a column.
	// fn column_frequency(column: usize) -> DataType {
	// 	column as DataType * SAMPLE_RATE as DataType / WINDOW_SIZE as DataType
	// }

	// /// Calculates frequency range of a bin.
	// fn spectrum_bin_frequency(bin: usize) -> [DataType; 2] {
	// 	let column_begin = (bin + 1) * BIN_SIZE;
	// 	let column_end = (bin + 2) * BIN_SIZE;

	// 	[
	// 		Self::column_frequency(column_begin),
	// 		Self::column_frequency(column_end)
	// 	]
	// }

	fn output(&mut self) {
		let mut spectrum_bins = *self.fft.spectrum_bins();

		self.operator.apply(&mut spectrum_bins);
		self.outputs.handle_output(&spectrum_bins);
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