pub mod serial;
pub mod text;

pub trait OutputHandler {
	fn handle_output(
		&mut self,
		spectrum: &[crate::DataType]
	);
}