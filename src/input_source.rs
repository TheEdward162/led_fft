use cpal::{traits::*, Host, SampleFormat};

/// Lists input devices and formats that fit given parameters.
///
/// Since `Device` is neither `Copy` nor `Clone`, we return the index into the `Devices` iterator and hope for the best.
pub fn list_fitting_input_devices(host: &Host, channels: u16, sample_rate: u32, sample_format: SampleFormat) -> Vec<usize> {
	let mut result = Vec::new();

	log::info!("Host {:?} devices:", host.id());
	let mut devices = match host.devices() {
		Err(e) => {
			log::warn!("Could not read devices: {:?}", e);
			return result
		}
		Ok(d) => d
	};

	for (i, device) in devices.enumerate() {
		log::info!("\t{}) {:?}:", i, device.name().expect("Could not retrieve device name"));

		let mut formats = match device.supported_input_formats() {
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