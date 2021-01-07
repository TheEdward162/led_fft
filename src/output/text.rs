use std::io::Write;

use super::OutputHandler;
use crate::config::DataType;

pub struct TextOutputHandler {
	up_threshold: f64,
	low_threshold: f64
}
impl TextOutputHandler {
	pub const fn new(
		low_threshold: f64,
		up_threshold: f64
	) -> Self {
		// debug_assert!(low_threshold < up_threshold);

		TextOutputHandler {
			low_threshold,
			up_threshold
		}
	}

	fn draw_bars<const SPECTRUM_BINS: usize>(&self, stream: &mut impl Write, spectrum: &[DataType; SPECTRUM_BINS]) -> Result<usize, std::io::Error> {
		const ROW_PREFIX: &'static str = "[";
		const ROW_POSTFIX: &'static str = "]\n";
		const CELL_EMPTY: &'static str = " ";
		const CELL_FULL: &'static str = "x";

		const HEIGHT: usize = 30;
		// const WIDTH: usize = SPECTRUM_BINS;

		let val_diff = self.up_threshold - self.low_threshold;
		let half_step: f64 = val_diff / HEIGHT as f64 / 2.0f64;

		for row in 0 .. HEIGHT {
			let cell_limit: f64 = (HEIGHT - row) as f64 / HEIGHT as f64 * val_diff + self.low_threshold - half_step;

			write!(stream, "{}", ROW_PREFIX)?;
			for column in 0 .. SPECTRUM_BINS {
				let bin_index = (column as f64 / SPECTRUM_BINS as f64 * SPECTRUM_BINS as f64) as usize;

				if spectrum[bin_index] as f64 <= cell_limit {
					write!(stream, "{}", CELL_EMPTY)?;
				} else {
					write!(stream, "{}", CELL_FULL)?;
				}
			}
			write!(stream, "{}", ROW_POSTFIX)?;
		}

		Ok(HEIGHT)
	}

	fn output_info<const SPECTRUM_BINS: usize>(&self, stream: &mut impl Write, spectrum: &[DataType; SPECTRUM_BINS]) -> Result<usize, std::io::Error> {
		let (min, max) = {
			let mut min = spectrum[0];
			let mut max = spectrum[0];

			for &v in spectrum {
				if v < min {
					min = v;
				}
				if v > max {
					max = v;
				}
			}

			(min, max)
		};

		write!(stream, "min: {:>8.2}, max: {:>8.2}\n", min, max)?;

		Ok(1)
	}
}
impl<const SPECTRUM_BINS: usize> OutputHandler<SPECTRUM_BINS> for TextOutputHandler {
	fn handle_output(&mut self, spectrum: &[DataType; SPECTRUM_BINS]) {
		let stream = std::io::stdout();
		let mut stream = stream.lock();

		let result = self.draw_bars(&mut stream, spectrum).and_then(
			|h1| self.output_info(&mut stream, spectrum).map(|h2| h1 + h2)
		).and_then(
			|height| write!(stream, "\u{1B}[{}A", height)
		);

		if let Err(err) = result {
			log::error!("Could not write to stdout: {}", err);
		}
	}
}
