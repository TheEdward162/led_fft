use crate::{
	fft_processor::FFTProcessor,
	window_buffer::WindowBuffer,
	DataType,
	BIN_SIZE,
	CHANNELS,
	SAMPLE_RATE,
	UPDATE_FRAMES,
	WINDOW_SIZE
};

use crate::{operator::SpectrumOperator, output::OutputHandler};

/// Application context.
pub struct Context {
	window: WindowBuffer<[DataType; CHANNELS]>,
	window_frame_counter: usize,

	fft: FFTProcessor,

	operators: Vec<Box<dyn SpectrumOperator>>,
	outputs: Vec<Box<dyn OutputHandler>>
}
impl Context {
	pub fn new(
		operators: Vec<Box<dyn SpectrumOperator>>,
		outputs: Vec<Box<dyn OutputHandler>>
	) -> Self {
		Context {
			window: WindowBuffer::new(),
			window_frame_counter: 0,

			fft: FFTProcessor::new(),

			operators,
			outputs
		}
	}

	pub fn process_input_buffer(&mut self, input: &[DataType]) {
		let mut samples = input.iter().peekable();
		loop {
			if samples.peek().is_none() {
				break
			}

			let mut frame: [DataType; CHANNELS] = [0.0; CHANNELS];
			for i in 0 .. CHANNELS {
				frame[i] = *samples.next().expect("Expected sample");
			}

			self.window.push(frame);
			self.window_frame_counter += 1;

			if self.window_frame_counter == UPDATE_FRAMES {
				self.window_frame_counter = 0;

				self.recalculate();
				self.output();
			}
		}
	}

	fn recalculate(&mut self) {
		self.fft.process(self.window.buffer().iter().map(|frame| {
			let mut avg = 0.0;
			for i in 0 .. CHANNELS {
				avg += frame[i];
			}
			avg /= CHANNELS as DataType;

			avg
		}));

		self.fft.calculate_spectrum_bins();
	}

	/// Calculates frequency of a column.
	fn column_frequency(column: usize) -> DataType {
		column as DataType * SAMPLE_RATE as DataType / WINDOW_SIZE as DataType
	}

	/// Calculates frequency range of a bin.
	fn spectrum_bin_frequency(bin: usize) -> [DataType; 2] {
		let column_begin = (bin + 1) * BIN_SIZE;
		let column_end = (bin + 2) * BIN_SIZE;

		[
			Self::column_frequency(column_begin),
			Self::column_frequency(column_end)
		]
	}

	fn output(&mut self) {
		let mut spectrum_bins = self.fft.spectrum_bins().to_vec();

		for operator in self.operators.iter_mut() {
			operator.apply(&mut spectrum_bins);
		}

		for output in self.outputs.iter_mut() {
			output.handle_output(&spectrum_bins);
		}
	}

	// fn print_top_volume(&self) {
	// 	let mut top_freq = 0.0;
	// 	let mut top_freq_volume = 0.0;

	// 	for (i, volume) in self.fft.output().skip(1).take(WINDOW_SIZE / 2 - 1).enumerate() {
	// 	    if volume >= top_freq_volume {
	// 	        top_freq_volume = volume;
	// 	        top_freq = Self::column_frequency(i + 1);
	// 	    }
	// 	}
	// 	print!("Top frequency {: >8.2} Hz at volume {: >5.2}\r", top_freq, top_freq_volume);
	// }
}
