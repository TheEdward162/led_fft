#![feature(min_const_generics)]
#![feature(never_type)]

mod sound;
mod operator;
mod output;
mod util;
mod core;

use sound::SoundSource;
use crate::core::{Context, config};

fn main() {
	edwardium_logger::Logger::new(
		[
			edwardium_logger::targets::stderr::StderrTarget::new(
				log::Level::Trace
			)
		],
		std::time::Instant::now()
	).init_boxed().expect("Could not initalize logger");

	log::info!(
		"Config:\n\tWINDOW_SIZE: {}\n\tUPDATE_FRAMES: {}\n\tSPECTRUM_BINS: {}",
		config::WINDOW_SIZE,
		config::UPDATE_FRAMES,
		config::SPECTRUM_BINS
	);

	let cli_config = core::config::CliConfig::parse(
		std::env::args().skip(1)
	);
	log::info!("{:?}", cli_config);

	let context: Context<
		_, _,
		{config::WINDOW_SIZE}, {config::SPECTRUM_BINS}, {config::UPDATE_FRAMES}
	> = Context::new(
		(
			// operator::tracking_smoother::TrackingSmootherOperator::new(10.0),
			operator::min_normalizer::MinNormalizerOperator,
			// operator::bin_weights::BinWeightsOperator::interpolate_stops(
			// 	[(0, 0.25), (8, 0.5), (31, 1.0), (47, 1.0), (62, 1.0)].iter().map(|v| *v),
			// 	|x| x
			// )
		),
		output::text::TextOutputHandler::new(-300.0, 300.0)
		// output::text::TextOutputHandler::new(0.0, 5.0)
	);

	// TODO: Allow choosing which backend to use

	#[cfg(feature = "backend_cpal")]
	{
		log::info!("Using cpal backend");
		let mut sound_source = sound::cpal::CpalSoundSource::init(
			cli_config.channels, cli_config.sample_rate, cli_config.device_index
		).unwrap();

		log::info!("Entering loop...");
		sound_source.run(context).expect("Error in source loop");
	}

	#[cfg(feature = "backend_pulseaudio")]
	{
		log::info!("Using pulseaudio backend");
		let mut sound_source =
			sound::pulse::PulseaudioSoundSource::init(
				cli_config.channels, cli_config.sample_rate, cli_config.device_index
			).unwrap();

		log::info!("Entering loop...");
		sound_source.run(context).expect("Error in source loop");
	}
}
