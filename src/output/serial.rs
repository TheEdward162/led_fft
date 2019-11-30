use std::io::Write;

use super::OutputHandler;

const PACKET_LENGTH: usize = 3;
const PACKET_END_MARKER: u8 = 0xFF;

/// Handler for writing LED data to serial port.
///
/// Needs a correct program on the reading side.
pub struct LEDSerial {
	serial_port: serial::SystemPort,
	last_written: [u8; PACKET_LENGTH]
}
impl LEDSerial {
	pub fn new(port: &str) -> Result<Self, serial::core::Error> {
		log::info!("Opening port {}", port);
		let serial_port = serial::open(port)?;

		Ok(
			LEDSerial {
				serial_port,
				last_written: [0; PACKET_LENGTH]
			}
		)
	}

	/// Writes one packet with end marker appended.
	///
	/// Doesn't write if it is same as the last written packet.
	pub fn update(&mut self, packet: [u8; PACKET_LENGTH]) -> std::io::Result<()> {
		if packet == self.last_written {
			return Ok(())
		}
		
		log::trace!("Sending packet [{}, {}, {}, {}]", packet[0], packet[1], packet[2], PACKET_END_MARKER);
		self.serial_port.write(
			&[packet[0], packet[1], packet[2], PACKET_END_MARKER]
		)?;
		self.last_written = packet;

		Ok(())
	}
}
impl OutputHandler for LEDSerial {
	fn handle_output(
		&mut self,
		spectrum: &[crate::DataType]
	) {
		let red_norm = spectrum[3 .. 7].iter().fold(0.0, |acc, v| { acc + *v }) / 5.0;
		let red = (red_norm * 32.0 + 8.0) as u8;

		let green_norm = spectrum[15 .. 25].iter().fold(0.0, |acc, v| { acc + *v }) / 10.0;
		let green = (green_norm * 16.0 + 8.0) as u8;

		let blue_norm = spectrum[30 .. 40].iter().fold(0.0, |acc, v| { acc + *v }) / 10.0;
		let blue = 0; //(blue_norm * 16.0 + 8.0) as u8;

		self.update(
			[red, green, blue]
		).expect("Could not write to serial port");
	}
}