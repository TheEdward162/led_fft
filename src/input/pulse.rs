use libpulse_binding as pa;
use libpulse_simple_binding::Simple;

use super::SoundSource;

const DATA_TYPE: pa::sample::Format = pa::sample::SAMPLE_FLOAT32NE;

const APPLICATION_NAME: &'static str = "led_fft";
const STREAM_DESCRIPTION: &'static str = "led_fft thing";

pub struct PulseaudioSoundSource {
	stream: Simple
}
impl PulseaudioSoundSource {
	
}
impl SoundSource for PulseaudioSoundSource {
	fn init(channels: u16, sample_rate: u32, maybe_device_index: Option<usize>) -> Result<Self, ()> {
		let sample_spec = pa::sample::Spec {
			format: DATA_TYPE,
			channels: channels as u8,
			rate: sample_rate
		};
		assert!(sample_spec.is_valid());

		let buffering_attributes = pa::def::BufferAttr {
			maxlength: std::u32::MAX,
			fragsize: (
				std::mem::size_of::<crate::DataType>() / std::mem::size_of::<u8>() * crate::UPDATE_FRAMES
			) as u32,
			.. Default::default()
		};

		let stream = match Simple::new(
			None, // Use the default server
			APPLICATION_NAME,
			pa::stream::Direction::Record,
			None,
			STREAM_DESCRIPTION,
			&sample_spec,
			None,
			Some(&buffering_attributes)
		) {
			Ok(s) => s,
			Err(e) => {
				log::error!("{}", e);
				return Err(());
			}
		};

		Ok(
			PulseaudioSoundSource {
				stream
			}
		)
	}

	fn run(&mut self, mut context: crate::context::Context) {
		let mut buffer: [crate::DataType; crate::UPDATE_FRAMES] = [0.0; crate::UPDATE_FRAMES];
		
		loop {
			let read_result = unsafe {
				let buffer_pointer: *mut f32 = buffer.as_mut_ptr();
				let bytes_slice: &mut [u8] = std::slice::from_raw_parts_mut(
					buffer_pointer as *mut u8,
					std::mem::size_of::<crate::DataType>() / std::mem::size_of::<u8>() * crate::UPDATE_FRAMES
				);
				self.stream.read(
					bytes_slice
				)
			};
			match read_result {
				Ok(()) => (),
				Err(e) => {
					log::error!("{} ({:?})", e, e);
					break;
				}
			}

			context.process_input_buffer(&buffer);
		}
	}
}