//!
//! Arduino reader program.
//! The PWM digital outputs 3, 6 and 9 should be connected to MOSFETs
//! that drive the respective colors of the led strip.
//! See https://www.makeuseof.com/tag/connect-led-light-strips-arduino/
//!

#define SERIAL_BAUD 9600
#define INPUT_LENGTH 4
#define END_MARKER 0xFF

#define LED_RED 3
#define LED_GREEN 6
#define LED_BLUE 9

void setup() {
	pinMode(LED_RED, OUTPUT);
	pinMode(LED_GREEN, OUTPUT);
	pinMode(LED_BLUE, OUTPUT);

	Serial.begin(SERIAL_BAUD);
	while (!Serial) {
		; // wait for serial port to connect. Needed for native USB.
	}
}

void setColors(byte red, byte green, byte blue) {
	analogWrite(LED_RED, red);
	analogWrite(LED_GREEN, green);
	analogWrite(LED_BLUE, blue);
}

byte lastInput[INPUT_LENGTH] = { 0x00, 0x00, 0x00, 0x00 };
void pushInput(byte input) {
	lastInput[0] = lastInput[1];
	lastInput[1] = lastInput[2];
	lastInput[2] = lastInput[3];
	lastInput[3] = input;
}

void serialEvent() {
	register int readByte = Serial.read();
	while (readByte != -1) {
		pushInput((byte) readByte);
		if (readByte == END_MARKER) {
			setColors(
				lastInput[0],
				lastInput[1],
				lastInput[2]
			);
		}

		readByte = Serial.read();
	}
}

void loop() {
	delay(1);
}
