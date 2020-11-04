use libpulse_binding as pa;
use libpulse_simple_binding::Simple;

use super::{SoundSource, SoundSink, DeviceIndex, SoundSourceError};
use crate::core::config;

const DATA_TYPE: pa::sample::Format = pa::sample::SAMPLE_FLOAT32NE;

const APPLICATION_NAME: &'static str = "led_fft";
const STREAM_DESCRIPTION: &'static str = "led_fft thing";

#[derive(Debug)]
pub struct PulseaudioError(pa::error::PAErr);
impl std::error::Error for PulseaudioError {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		None
	}
}
impl std::fmt::Display for PulseaudioError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{} ({})", self.0, self.0.0)
	}
}
impl From<pa::error::PAErr> for SoundSourceError {
	fn from(err: pa::error::PAErr) -> Self {
		SoundSourceError::specific(
			PulseaudioError(err)
		)
	}
}

pub struct PulseaudioSoundSource {
	stream: Simple
}
impl PulseaudioSoundSource {}
impl SoundSource for PulseaudioSoundSource {
	fn init(
		channels: u16,
		sample_rate: u32,
		_device_index: DeviceIndex
	) -> Result<Self, SoundSourceError> {
		let sample_spec = pa::sample::Spec {
			format: DATA_TYPE,
			channels: channels as u8,
			rate: sample_rate
		};
		assert!(sample_spec.is_valid());

		let buffering_attributes = pa::def::BufferAttr {
			maxlength: std::u32::MAX,
			fragsize: (std::mem::size_of::<config::DataType>() * config::UPDATE_FRAMES) as u32,
			..Default::default()
		};


		let stream = Simple::new(
			None, // Use the default server
			APPLICATION_NAME,
			pa::stream::Direction::Record,
			None, // TODO: Use `device_index` here
			STREAM_DESCRIPTION,
			&sample_spec,
			None,
			Some(&buffering_attributes)
		)?;

		Ok(PulseaudioSoundSource { stream })
	}

	fn run(&mut self, mut sink: impl SoundSink + Send + 'static) -> Result<!, SoundSourceError> {
		let mut buffer = [config::DataType::default(); config::UPDATE_FRAMES];

		loop {
			// Safe for size because we just multiply the number of elements in `buffer` by the size of its element.
			// Safe for alignment because `u8` has alighment 1.
			let read_result = unsafe {
				let bytes_slice: &mut [u8] = std::slice::from_raw_parts_mut(
					buffer.as_mut_ptr() as *mut u8,
					std::mem::size_of::<config::DataType>() * buffer.len()
				);
				self.stream.read(bytes_slice)
			};
			read_result?;

			sink.process_input(buffer.iter().copied());
		}
	}
}
