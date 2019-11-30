use cpal::{traits::*, Host, SampleFormat, EventLoop, StreamId};

use super::SoundSource;

const DATA_TYPE: cpal::SampleFormat = cpal::SampleFormat::F32;

pub struct CpalSoundSource {
	event_loop: EventLoop,
	stream_id: StreamId
}
impl CpalSoundSource {
	/// Lists input devices and formats that fit given parameters.
	///
	/// Since `Device` is neither `Copy` nor `Clone`, we return the index into the `Devices` iterator and hope for the best.
	fn list_fitting_input_devices(host: &Host, channels: u16, sample_rate: u32, sample_format: SampleFormat) -> Vec<usize> {
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
			log::info!("\t{}) {:?}:", i, device.name().expect("Could not retrieve device name"));

			let formats = match device.supported_input_formats() {
				Err(e) => {
					log::warn!("Could not read formats: {:?}", e);
					return result
				}
				Ok(f) => f
			};
			for format in formats {
				log::info!("\t\t{}ch ({} - {}) {:?}", format.channels, format.min_sample_rate.0, format.max_sample_rate.0, format.data_type);
				if 
					format.channels == channels
					&& format.data_type == sample_format
					&& format.min_sample_rate.0 <= sample_rate
					&& format.max_sample_rate.0 >= sample_rate 
				{
					result.push(
						i
					)
				}
			}
		}

		result
	}
}
impl SoundSource for CpalSoundSource {
	fn init(channels: u16, sample_rate: u32, maybe_device_index: Option<usize>) -> Result<Self, ()> {
		let mut device_index = maybe_device_index.unwrap_or(0);
		
		let host = cpal::default_host();
		let event_loop = host.event_loop();

		let stream_id = {
			let fitting_device_indices = CpalSoundSource::list_fitting_input_devices(&host, channels, sample_rate, DATA_TYPE);
			if fitting_device_indices.len() == 0 {
				log::error!("Could not find any fitting device.");
				panic!("Could not find any fitting device.");
			}

			log::debug!("Fitting device indices:");
			for index in fitting_device_indices.iter() {
				log::debug!("\t{}", index);
			}

			if device_index >= fitting_device_indices.len() {
				log::warn!("Index {} is bigger than number of fitting devices ({}). Using 0.", device_index, fitting_device_indices.len());
				device_index = 0;
			}

			let device = host.devices().unwrap().nth(fitting_device_indices[device_index]).unwrap();
			let format = cpal::Format {
				channels,
				sample_rate: cpal::SampleRate(sample_rate),
				data_type: DATA_TYPE
			};
			log::info!("Using device {} with format {:?}.", device.name().unwrap(), format);

			let input_stream_id = event_loop.build_input_stream(&device, &format).expect("Could not build stream");
			event_loop.play_stream(input_stream_id.clone()).expect("Could not play stream");

			input_stream_id
		};
		
		Ok(
			CpalSoundSource {
				event_loop,
				stream_id
			}
		)
	}

	fn run(&mut self, mut context: crate::context::Context) {
		self.event_loop.run(|id, result| {
			let data = match result {
				Ok(data) => data,
				Err(err) => {
					log::error!("An error occurred on stream {:?}: {}", id, err);
					return;
				}
			};

			match data {
				cpal::StreamData::Input { buffer: cpal::UnknownTypeInputBuffer::F32(buffer) } => {
					assert_eq!(id, self.stream_id);

					context.process_input_buffer(&buffer);
				},
				_ => panic!("expecting f32 input data"),
			}
		});
	}
}