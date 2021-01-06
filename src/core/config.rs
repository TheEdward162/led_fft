// use crate::{output::ColorOutputInfo, parametrization::ParamFn};
use crate::sound::DeviceIndex;

/// Data type to use in FFT and audio sampling.
///
/// TODO: Using anything other than `f32` will need some code changes.
pub type DataType = f32;
/// Size of the window the FFT is run on.
pub const WINDOW_SIZE: usize = 2048;
/// After this many frames have been processed, the FFT is recalculated over the current window.
pub const UPDATE_FRAMES: usize = WINDOW_SIZE / 4;
/// Number of spectrum bins.
///
/// It should hold that `WINDOW_SIZE / ((SPECTRUM_BINS + 1) * 2)` is a whole number.
pub const SPECTRUM_BINS: usize = WINDOW_SIZE / 2 / 16 - 1;

pub fn window_envelope_function<const WINDOW_SIZE: usize>(index: usize) -> DataType {
	const PI: DataType = std::f64::consts::PI as DataType;
	
	0.5 * (
		1.0 - (
			2.0 * PI * (index as DataType) / (WINDOW_SIZE as DataType)
		).cos()
	)
}
	
pub const WINDOW_EVELOPE_VARW: DataType = 3.0 / 8.0;

#[derive(Debug)]
pub struct CliConfig {
	pub serial_port: Option<String>,
	pub device_index: crate::sound::DeviceIndex,
	pub channels: u16,
	pub sample_rate: u32
}
impl CliConfig {
	pub fn parse(mut input: impl Iterator<Item = String>) -> Self {
		let mut serial_port = None;
		let mut device_index = DeviceIndex::default();
		let mut channels = 2u16;
		let mut sample_rate = 44100u32;

		while let Some(value) = input.next() {
			match value.as_str() {
				"--port" | "-p" => {
					serial_port = Some(
						input.next().expect("--port flag must be followed by an argument")
					);
				}
				"--device" | "-d" => {
					let value = input.next().expect("--device flag must be followed by an argument");
					let index: usize = value.parse().expect("--device argument is not a valid number");

					device_index = DeviceIndex::Nth(index)
				}
				"--channels" | "-c" => {
					let value = input.next().expect("--channels flag must be followed by an argument");
					channels = value.parse().expect("--channels argument is not a valid number");
				}
				"--rate" | "-r" => {
					let value = input.next().expect("--rate flag must be followed by an argument");
					sample_rate = value.parse().expect("--rate argument is not a valid number");
				}
				v => {
					log::error!("{} argument not recognized", v);
				}
			}
		}

		CliConfig {
			serial_port,
			device_index,
			channels,
			sample_rate
		}
	}
}