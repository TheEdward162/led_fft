use std::io::Write;

use crate::{util::TopColorCounter, DataType};

use super::OutputHandler;

const PACKET_LENGTH: usize = 3;
const PACKET_END_MARKER: u8 = 0xFF;

/// Handler for writing LED data to serial port.
///
/// Needs a correct program on the reading side.
pub struct LedSerial {
	serial_port: serial::SystemPort,
	last_written: [u8; PACKET_LENGTH],
	top_color: TopColorCounter
}
impl LedSerial {
	pub fn new(port: &str) -> Result<Self, serial::core::Error> {
		log::info!("Opening port {}", port);
		let serial_port = serial::open(port)?;

		Ok(LedSerial {
			serial_port,
			last_written: [0; PACKET_LENGTH],
			top_color: TopColorCounter::new()
		})
	}

	/// Writes one packet with end marker appended.
	///
	/// Doesn't write if it is same as the last written packet.
	pub fn update(&mut self, packet: [u8; PACKET_LENGTH]) -> std::io::Result<()> {
		if packet == self.last_written {
			return Ok(())
		}

		log::trace!(
			"Sending packet [{}, {}, {}, {}]",
			packet[0],
			packet[1],
			packet[2],
			PACKET_END_MARKER
		);
		self.serial_port
			.write(&[packet[0], packet[1], packet[2], PACKET_END_MARKER])?;
		self.last_written = packet;

		Ok(())
	}
}
impl OutputHandler for LedSerial {
	fn handle_output(&mut self, spectrum: &[crate::DataType]) {
		let red = crate::config::RED_DEFAULT_INFO.compute_value(spectrum);
		let green = crate::config::GREEN_DEFAULT_INFO.compute_value(spectrum);
		let blue = crate::config::BLUE_DEFAULT_INFO.compute_value(spectrum);

		let color_mix = self.top_color.update([red, green, blue]);
		let all_color = red + green + blue;
		let result = [
			(color_mix[0] * red as DataType) as u8,
			(color_mix[1] * green as DataType) as u8,
			(color_mix[2] * blue as DataType) as u8
		];

		self.update(result).expect("Could not write to serial port");
	}
}
