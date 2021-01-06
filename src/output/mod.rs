use crate::config::DataType;

// pub mod serial;
pub mod text;

pub trait OutputHandler<const SPECTRUM_BINS: usize> {
	/// Outputs the spectrum bins.
	fn handle_output(&mut self, spectrum: &[DataType; SPECTRUM_BINS]);
}
impl<const SPECTRUM_BINS: usize> OutputHandler<SPECTRUM_BINS> for () {
	fn handle_output(&mut self, _spectrum: &[DataType; SPECTRUM_BINS]) {}
}
impl<const SPECTRUM_BINS: usize> OutputHandler<SPECTRUM_BINS> for Vec<Box<dyn OutputHandler<SPECTRUM_BINS>>> {
	fn handle_output(&mut self, spectrum: &[DataType; SPECTRUM_BINS]) {
		for handler in self.iter_mut() {
			handler.handle_output(spectrum)
		}
	}
}

macro_rules! impl_output_handler_tuple {
	(
		$(
			$idx: tt -> $T: ident,
		)+
	) => {
		impl<$($T: OutputHandler<SPECTRUM_BINS>,)+ const SPECTRUM_BINS: usize> OutputHandler<SPECTRUM_BINS> for ($($T,)+) {
			fn handle_output(&mut self, spectrum: &[DataType; SPECTRUM_BINS]) {
				$(
					self.$idx.handle_output(spectrum);
				)+
			}
		}
	};
}
impl_output_handler_tuple! {
	0 -> A,
}
impl_output_handler_tuple! {
	0 -> A,
	1 -> B,
}
impl_output_handler_tuple! {
	0 -> A,
	1 -> B,
	2 -> C,
}
impl_output_handler_tuple! {
	0 -> A,
	1 -> B,
	2 -> C,
	3 -> D,
}
impl_output_handler_tuple! {
	0 -> A,
	1 -> B,
	2 -> C,
	3 -> D,
	4 -> E,
}
impl_output_handler_tuple! {
	0 -> A,
	1 -> B,
	2 -> C,
	3 -> D,
	4 -> E,
	5 -> F,
}