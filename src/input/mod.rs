#[cfg(feature = "backend_pulseaudio")]
pub mod pulse;

#[cfg(feature = "backend_cpal")]
pub mod cpal;

pub trait SoundSource {
	fn init(channels: u16, sample_rate: u32, maybe_device_index: Option<usize>) -> Result<Self, ()>
	where
		Self: Sized;

	fn run(&mut self, context: crate::context::Context);
}
