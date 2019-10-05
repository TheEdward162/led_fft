use std::io::Write;

pub const PACKET_LENGTH: usize = 3;
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