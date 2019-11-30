use crate::DataType;
use super::OutputHandler;

pub struct TextOutputHandler {
}
impl TextOutputHandler {
	pub fn new() -> Self {
		TextOutputHandler {
		}
	}
}
impl OutputHandler for TextOutputHandler {
	fn handle_output(
		&mut self,
		spectrum: &[crate::DataType]
	) {
		const HEIGHT: usize = 13;

		// for bin in 0 .. SPECTRUM_BINS {
		// 	let frequency = Self::spectrum_bin_frequency(bin);

		// 	print!("{:.0}-{:.0} ", frequency[0], frequency[1]);
		// }
		for row in 0 .. HEIGHT {
			print!("[");

			for bin in spectrum {
				let limit: f32 = (HEIGHT - row) as DataType / (HEIGHT as DataType);
				
				if *bin >= limit {
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