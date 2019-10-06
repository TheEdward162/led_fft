# LED FFT in Rust

This Rust application processes audio input through the `rustfft` library and generates a histogram of the spectrum.
Then it uses that histogram to choose led colors and intensities to send through the serial port.

An arduino program listening on the other end read the serial port and controls the physical LEDs through MOSFETs.