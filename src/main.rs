use cpal::traits::*;

use rustfft::FFTplanner;
use rustfft::algorithm::Radix4;
use rustfft::num_complex::Complex;
use rustfft::num_traits::Zero;

const RATE: u32 = 48000;
const CHANNELS: usize = 1;
const WINDOW_SIZE: usize = 2048;
const BIN_WIDTH: usize = 20;
const BINS: usize = WINDOW_SIZE / BIN_WIDTH;

// fn analyze_channel(plan: &Plan, data: &[[f32; 2]], channel: usize) -> Vec<f32> {
//     let mut input = Vec::with_capacity(data.len());
//     for x in data {
//         input.push(x[channel] as f64);
//     }
	
//     dft::transform(&mut input, &plan);
//     let output = dft::unpack(&input);
	
//     let mut result = Vec::with_capacity(data.len());
//     for ref c in output {
//         result.push(c.norm() as f32);
//     }
//     result
// }

fn main() {
	let host = cpal::default_host();
	let event_loop = host.event_loop();

	println!("Devices:");
	for d in host.devices().unwrap() {
		println!("{:?}", d.name());
	}
	let device = host.devices().unwrap().nth(0).unwrap();
	println!("Using device with name {:?}", device.name());
	println!();

	println!("Formats:");
	for f in device.supported_input_formats().unwrap() {
		println!("{:?}", f);
	}
	let format = device.supported_input_formats().unwrap().nth(1).expect("Could not pick input format.").with_max_sample_rate();
	println!("Using format {:?}", format);
	println!();

	let input_stream_id = event_loop.build_input_stream(&device, &format).expect("Could not build stream.");
	event_loop.play_stream(input_stream_id.clone()).expect("Could not play stream.");

	let mut window_buffer: [[f32; CHANNELS]; WINDOW_SIZE] = [[0.0; CHANNELS]; WINDOW_SIZE];
	let mut window_buffer_index: usize = 0;

	let mut fft_input: [Complex<f32>; WINDOW_SIZE] = [Complex::zero(); WINDOW_SIZE];
	let mut fft_output: [Complex<f32>; WINDOW_SIZE] = [Complex::zero(); WINDOW_SIZE];

	let mut planner = FFTplanner::new(false);
	let fft = planner.plan_fft(WINDOW_SIZE);

	let mut fft_max: f32 = 0.0;

	event_loop.run(move |id, result| {
		let data = match result {
			Ok(data) => data,
			Err(err) => {
				eprintln!("An error occurred on stream {:?}: {}", id, err);
				return;
			}
		};

		match data {
			cpal::StreamData::Input { buffer: cpal::UnknownTypeInputBuffer::F32(buffer) } => {
				assert_eq!(id, input_stream_id);

				let mut samples = buffer.iter().peekable();
				loop {
					if samples.peek().is_none() {
						break
					}

					let mut frame: [f32; CHANNELS] = [0.0; CHANNELS];
					for i in 0 .. CHANNELS {
						frame[i] = *samples.next().expect("Expected sample.");
					}
					
					window_buffer[window_buffer_index] = frame;
					window_buffer_index = (window_buffer_index + 1) % WINDOW_SIZE;
				}
			},
			_ => panic!("expecting f32 input data"),
		}

		for (i, frame) in window_buffer.iter().enumerate() {
			let avg_frame = {
				let mut avg = 0.0;
				for x in 0 .. CHANNELS {
					avg += frame[x];
				}
				avg /= CHANNELS as f32;

				avg
			};
			fft_input[i] = Complex::new(avg_frame, 0.0);
		}

		fft.process(&mut fft_input, &mut fft_output);

		// let mut top_freq = 0.0;
		// let mut top_freq_volume = 0.0;
		// for (i, volume) in fft_output[0 .. fft_output.len() / 2].iter().enumerate() {
		//     if volume.re >= top_freq_volume {
		//         top_freq = (i + 1) as f32 * RATE as f32 / fft_output.len() as f32;
		//         top_freq_volume = volume.re;
		//     }
		// }
		// print!("Top: {} Hz at volume {}.                \r", top_freq, top_freq_volume);

		let mut spectrum: [f32; BINS / 2 - 1] = [0.0; BINS / 2 - 1];
		fft_max *= 0.95; // Dampen old max

		for column in 1 .. BINS / 2 {
			let column_begin = column * BIN_WIDTH;
			let column_end = (column + 1) * BIN_WIDTH;

			let sum: f32 = fft_output[column_begin .. column_end].iter().map(|c| c.re).sum();

			spectrum[column - 1] = sum;
			fft_max = fft_max.max(sum);
		}

		// print!("{}\r", spectrum);
		// for i in 0 .. BINS / 2 - 1 - 20 {
		// 	let string = format!("{:.2}", spectrum[i]);
		// 	print!("{: >5} ", string);
		// }
		// print!("\r");

		for x in 0 .. 5 {
			print!("[");

			for spec in spectrum.iter() {
				let limit: f32 = 20.0 * (5 - x) as f32 / 5.0;
				if *spec > limit {
					print!("x");
				} else {
					print!(" ");
				}
				// print!("{}", limit);
			}

			println!("]");
		}
		print!("\u{1B}[5A");
	});
}
