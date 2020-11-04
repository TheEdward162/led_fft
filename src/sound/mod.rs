#[cfg(feature = "backend_pulseaudio")]
pub mod pulse;

#[cfg(feature = "backend_cpal")]
pub mod cpal;

pub trait SoundSink: Sized {
	fn process_input(&mut self, input: impl Iterator<Item = crate::config::DataType>);
}

pub trait SoundSource: Sized {
	fn init(channels: u16, sample_rate: u32, device_index: DeviceIndex) -> Result<Self, SoundSourceError>;

	fn run(&mut self, sink: impl SoundSink + Send + 'static) -> Result<!, SoundSourceError>;
}

#[derive(Debug)]
pub enum DeviceIndex {
	/// Chooses device in an implementationally defined way
	/// (although mostly just takes the first one)
	Unspecified,
	/// Chooses nth device or falls back to `Unspecified` if out of range
	Nth(usize)
}
impl Default for DeviceIndex {
	fn default() -> Self {
		DeviceIndex::Unspecified
	}
}

#[derive(Debug)]
pub enum SoundSourceError {
	NoFittingDeviceFound,
	Specific(Box<dyn std::error::Error>)
}
impl SoundSourceError {
	pub fn specific(err: impl std::error::Error + 'static) -> Self {
		SoundSourceError::Specific(Box::new(err))
	}
}
impl std::error::Error for SoundSourceError {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		match self {
			SoundSourceError::Specific(err) => Some(err.as_ref()),
			_ => None
		}
	}
}
impl std::fmt::Display for SoundSourceError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			SoundSourceError::Specific(err) => write!(f, "Implementation specific error: {}", err),
			SoundSourceError::NoFittingDeviceFound => write!(f, "No fitting device found")
		}
	}
}