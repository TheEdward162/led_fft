use cpal::{Host, Device, StreamConfig, traits::{HostTrait, DeviceTrait, StreamTrait}, SampleFormat, SampleRate, BufferSize, BuildStreamError, PlayStreamError};

use crate::config;
use super::{SoundSource, SoundSink, DeviceIndex, SoundSourceError};

const DATA_TYPE: cpal::SampleFormat = cpal::SampleFormat::F32;

impl From<BuildStreamError> for SoundSourceError {
	fn from(err: BuildStreamError) -> Self {
		SoundSourceError::specific(err)
	}
}
impl From<PlayStreamError> for SoundSourceError {
	fn from(err: PlayStreamError) -> Self {
		SoundSourceError::specific(err)
	}
}

pub struct CpalSoundSource {
	device: Device,
	config: StreamConfig
}
impl CpalSoundSource {
	/// Lists input devices and formats that fit given parameters.
	///
	/// Since `Device` is neither `Copy` nor `Clone`, we return the index into the `Devices` iterator and hope for the best.
	fn list_fitting_input_devices(
		host: &Host,
		channels: u16,
		sample_rate: u32,
		sample_format: SampleFormat
	) -> Vec<usize> {
		let mut result = Vec::new();

		log::info!("Host {:?} devices:", host.id());
		let devices = match host.devices() {
			Err(e) => {
				log::warn!("Could not read devices: {:?}", e);
				return result
			}
			Ok(d) => d
		};

		for (i, device) in devices.enumerate() {
			log::info!(
				"\t{}) {:?}:",
				i,
				device.name().expect("Could not retrieve device name")
			);

			let configs = match device.supported_input_configs() {
				Err(e) => {
					log::warn!("Could not read formats: {:?}", e);
					return result
				}
				Ok(f) => f
			};
			for config in configs {
				log::info!(
					"\t\t{}ch ({} - {}) {:?}",
					config.channels(),
					config.min_sample_rate().0,
					config.max_sample_rate().0,
					config.sample_format()
				);
				if config.channels() == channels
					&& config.sample_format() == sample_format
					&& config.min_sample_rate().0 <= sample_rate
					&& config.max_sample_rate().0 >= sample_rate
				{
					result.push(i)
				}
			}
		}

		result
	}
}
impl SoundSource for CpalSoundSource {
	fn init(
		channels: u16,
		sample_rate: u32,
		device_index: DeviceIndex
	) -> Result<Self, SoundSourceError> {
		let mut device_index = match device_index {
			DeviceIndex::Unspecified => 0,
			DeviceIndex::Nth(n) => n
		};

		let host = cpal::default_host();

		let fitting_device_indices = CpalSoundSource::list_fitting_input_devices(
			&host,
			channels,
			sample_rate,
			DATA_TYPE
		);
		if fitting_device_indices.len() == 0 {
			return Err(SoundSourceError::NoFittingDeviceFound);
		}

		log::debug!("Fitting device indices: {:?}", fitting_device_indices);

		if device_index >= fitting_device_indices.len() {
			log::warn!(
				"Index {} is bigger than the number of fitting devices ({}). Using 0.",
				device_index,
				fitting_device_indices.len()
			);
			device_index = 0;
		}

		let device = host.devices().unwrap().nth(fitting_device_indices[device_index]).unwrap();
		let config = cpal::StreamConfig {
			channels,
			sample_rate: SampleRate(sample_rate),
			// TODO: Check that this is in the supported range
			buffer_size: BufferSize::Fixed(config::UPDATE_FRAMES as u32)
		};
		log::info!(
			"Using device \"{}\": {:?}",
			device.name().unwrap(),
			config
		);

		Ok(
			CpalSoundSource {
				device,
				config
			}
		)
	}

	fn run(&mut self, mut sink: impl SoundSink + Send + 'static) -> Result<!, SoundSourceError> {
		let stream = self.device.build_input_stream(
			&self.config,
			move |data, _info| {
				sink.process_input(data.iter().copied());
			},
			|error| {
				log::error!("cpal stream error: {}", error);
			}
		)?;
		stream.play()?;

		loop {}
	}
}
