use cpal::{traits::*, Host, SampleFormat};

/// Lists input devices and formats that fit given parameters.
///
/// Since `Device` is neither `Copy` nor `Clone`, we return the index into the `Devices` iterator and hope for the best.
pub fn list_fitting_input_devices(host: &Host, channels: u16, sample_rate: u32, sample_format: SampleFormat) -> Vec<usize> {
	let mut result = Vec::new();

	log::info!("Host {:?} devices:", host.id());
	for (i, device) in host.devices().expect("Could not read devices").enumerate() {
		log::info!("\t{}) {:?}:", i, device.name().expect("Could not retrieve device name"));

		for format in device.supported_input_formats().expect("Could not read device input formats") {
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