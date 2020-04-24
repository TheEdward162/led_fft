use termion::color as tc;

use super::OutputHandler;
use crate::DataType;

use crate::config::{BLUE_DEFAULT_INFO, GREEN_DEFAULT_INFO, RED_DEFAULT_INFO};

pub struct TextOutputHandler {
	top_color: crate::util::TopColorCounter
}
impl TextOutputHandler {
	pub fn new() -> Self {
		TextOutputHandler {
			top_color: crate::util::TopColorCounter::new()
		}
	}
}
impl OutputHandler for TextOutputHandler {
	fn handle_output(&mut self, spectrum: &[crate::DataType]) {
		const HEIGHT: usize = 20;
		const BIN_LIMIT_MULT: DataType = 0.5;
		const COLOR_LIMIT_MULT: DataType = 1.0; // 0.125;
		const COLOR_MAX_VALUE: DataType = 255.0;

		let red_param = RED_DEFAULT_INFO.parametrized_value(spectrum);
		let green_param = GREEN_DEFAULT_INFO.parametrized_value(spectrum);
		let blue_param = BLUE_DEFAULT_INFO.parametrized_value(spectrum);

		let red_val = RED_DEFAULT_INFO.compute_value(spectrum);
		let green_val = GREEN_DEFAULT_INFO.compute_value(spectrum);
		let blue_val = BLUE_DEFAULT_INFO.compute_value(spectrum);
		let top_colors = self.top_color.update([red_val, green_val, blue_val]);

		for row in 0 .. HEIGHT {
			let limit: DataType = (HEIGHT - row) as DataType / (HEIGHT as DataType);

			print!("[");
			for (index, bin) in spectrum.iter().enumerate() {
				macro_rules! color_column {
					($config: expr, $color: ident, $var: expr) => {
						if $config.bin_range.contains(&index) {
							if $var >= limit {
								print!("{}", tc::Bg(tc::$color));
								}
							}
						if $config.bin_range.end == index {
							print!("{}", tc::Bg(tc::Reset));
							}
					};
				}

				color_column!(RED_DEFAULT_INFO, Red, red_param);
				color_column!(GREEN_DEFAULT_INFO, Green, green_param);
				color_column!(BLUE_DEFAULT_INFO, Blue, blue_param);

				if *bin >= limit * BIN_LIMIT_MULT {
					print!("x");
				} else {
					print!(" ");
				}
			}
			print!("|");

			macro_rules! color_value_column {
				(
					$val: expr, $color: ident
				) => {
					print!("{}", tc::Fg(tc::$color));
					if $val /*as DataType / COLOR_MAX_VALUE */ >= limit * COLOR_LIMIT_MULT {
						print!("x");
					} else {
						print!(" ");
						}
				};
			}
			// color_value_column!(red_val, Red);
			// color_value_column!(green_val, Green);
			// color_value_column!(blue_val, Blue);

			color_value_column!(top_colors[0], Red);
			color_value_column!(top_colors[1], Green);
			color_value_column!(top_colors[2], Blue);

			print!("{}", tc::Fg(tc::Reset));

			println!("]");
		}
		print!("\u{1B}[{}A", HEIGHT);
	}
}
