mod window_buffer;
mod fft_processor;

mod input;
mod output;
mod util;

mod context;
use context::ColorFactor;

use input::SoundSource;

/// Data type to use in FFT and audio sampling.
///
/// Using anything other than `f32` will need some code changes.
type DataType = f32;

/// Channels of the input.
const CHANNELS: usize = 2;
/// Sample rate of the input.
const SAMPLE_RATE: u32 = 44100;

/// Size of the window the FFT is run on.
const WINDOW_SIZE: usize = 2048;
/// After this many frames have been processed, the FFT is recalculated over the current window. 
const UPDATE_FRAMES: usize = WINDOW_SIZE / 4;
/// Size of one bin.
const BIN_SIZE: usize = 20;
/// Number of bins.
const BINS: usize = WINDOW_SIZE / BIN_SIZE;
/// Number of spectrum bins.
const SPECTRUM_BINS: usize = BINS / 2 - 1;

const DEFAULT_SERIAL_PORT: &'static str = "/dev/ttyUSB0";
const DEFAULT_DEVICE_INDEX: usize = 0;

fn main() {
	edwardium_logger::init(
		vec![
			edwardium_logger::StdoutTarget::new(
				log::Level::Trace, Default::default()
			)
		]
	).expect("Could not initialize logger");

	let args: Vec<String> = std::env::args().collect();
	
	let serial;
	let index: usize;
	if args.len() <= 1 {
		serial = DEFAULT_SERIAL_PORT;
		index = DEFAULT_DEVICE_INDEX;
	} else if args.len() <= 2 {
		serial = &args[1];
		index = DEFAULT_DEVICE_INDEX;
	} else {
		serial = &args[1];
		index = args[2].parse::<usize>().unwrap_or(DEFAULT_DEVICE_INDEX);
	}

	let context = context::Context::new(
		vec![
			Box::new(
				util::SpectrumSmoother::new(10)
			) as Box<_>,
			Box::new(
				util::SpectrumScaler::new_interpolated(
					// vec![0.3, 0.7, 1.0, 1.0, 3.5, 4.0]
					vec![1.0, 4.0, 10.0]
				)
			) as Box<_>,
			Box::new(util::SpectrumNormalizer::new(10)) as Box<_>
		],
		vec![
			Box::new(output::serial::LEDSerial::new(serial).expect("Could not open serial port")) as Box<_>,
			Box::new(output::text::TextOutputHandler::new()) as Box<_>
		]
	);
	
	// TODO: Allow choosing which backend to use

	#[cfg(feature = "backend_cpal")]
	{
		log::info!("Using cpal backend");
		let mut sound_source = input::cpal::CpalSoundSource::init(
			CHANNELS as u16,
			SAMPLE_RATE,
			Some(index)
		).unwrap();

		log::info!("Entering loop...");
		sound_source.run(context);
	}

	#[cfg(feature = "backend_pulseaudio")]
	{
		log::info!("Using pulseaudio backend");
		let mut sound_source = input::pulse::PulseaudioSoundSource::init(
			CHANNELS as u16,
			SAMPLE_RATE,
			Some(index)
		).unwrap();

		log::info!("Entering loop...");
		sound_source.run(context);
	}
}
