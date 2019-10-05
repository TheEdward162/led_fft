use crate::{DataType, CHANNELS, UPDATE_FRAMES, SAMPLE_RATE, WINDOW_SIZE, SPECTRUM_BINS, BIN_SIZE};
use crate::window_buffer::WindowBuffer;
use crate::fft_processor::FFTProcessor;
use crate::led_serial::LEDSerial;

/// Application context.
pub struct Context {
	window: WindowBuffer<[DataType; CHANNELS]>,
	window_frame_counter: usize,

	fft: FFTProcessor,
	led: LEDSerial
}
impl Context {
	pub fn new(serial_port: &str) -> Self {
		Context {
			window: WindowBuffer::new(),
			window_frame_counter: 0,

			fft: FFTProcessor::new(),
			led: LEDSerial::new(serial_port).expect("Could not open serial port.")
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
				frame[i] = *samples.next().expect("Expected sample.");
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
		self.fft.process(
			self.window.buffer().iter().map(|frame| {
				let mut avg = 0.0;
				for i in 0 .. CHANNELS {
					avg += frame[i];
				}
				avg /= CHANNELS as DataType;

				avg
			})
		);

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
		self.output_serial();

		// self.print_top_volume();
		self.print_term_visualizer();
	}

	fn output_serial(&mut self) {
		let fft_max = self.fft.spectrum_bins().iter().copied().fold(0.0, f32::max);

		let fft_factor = (fft_max / 20.0f32 / 2.0f32).min(1.0).max(0.0);
		let red: u8 = (fft_factor * 128.0) as u8 + 32;

		self.led.update([red, 0, 0]).expect("Could not write to serial port.");
	}

	fn print_top_volume(&self) {
		let mut top_freq = 0.0;
		let mut top_freq_volume = 0.0;

		for (i, volume) in self.fft.output().skip(1).take(WINDOW_SIZE / 2 - 1).enumerate() {
		    if volume >= top_freq_volume {
		        top_freq_volume = volume;
		        top_freq = Self::column_frequency(i + 1);
		    }
		}
		print!("Top frequency {: >8.2} Hz at volume {: >5.2}\r", top_freq, top_freq_volume);
	}

	/// Simple visualization of the generated output.
	fn print_term_visualizer(&self) {
		const HEIGHT: usize = 13;
		const ARBITRARY_MAX: DataType = 20.0;

		// for bin in 0 .. SPECTRUM_BINS {
		// 	let frequency = Self::spectrum_bin_frequency(bin);

		// 	print!("{:.0}-{:.0} ", frequency[0], frequency[1]);
		// }
		for row in 0 .. HEIGHT {
			print!("[");

			for bin in self.fft.spectrum_bins().iter() {
				let limit: f32 = ARBITRARY_MAX * (HEIGHT - row) as DataType / (HEIGHT as DataType);
				if *bin > limit {
					print!("x");
				} else {
					print!(" ");
				}
			}

			println!("]");
		}
		print!("\u{1B}[{}A", HEIGHT);
	}
}