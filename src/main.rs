use std::env;

const CLOCKRATE: f32 = 1e6;
const TERM_RST: &'static str = "\x1B[0m";
const TERM_RED: &'static str = "\x1b[31m";
const TERM_GRN: &'static str = "\x1b[32m";
const TERM_YEL: &'static str = "\x1b[33m";


fn calc_baudrate(baudrate: f32, clockrate: f32, u2x: bool) {
	let mut ubrr: f32;
	let ubrr_a: f32;
	let ubrr_b: f32;

	let divisor: f32;

	let error: f32;
	let error_a: f32;
	let error_b: f32;

	let mut color = TERM_YEL;

	match u2x {
		true => divisor =  8.0,
		_    => divisor = 16.0,
	}

	ubrr = clockrate/divisor/baudrate - 1.0;
	ubrr_a = ubrr.floor();
	ubrr_b = ubrr.ceil();

	error_a = ((clockrate * 100.0 /(ubrr_a + 1.0))/divisor/baudrate) - 100.0;
	error_b = ((clockrate * 100.0 /(ubrr_b + 1.0))/divisor/baudrate) - 100.0;

	if error_a.abs() < error_b.abs() {
		error = (error_a * 10.0).round() / 10.0;
		ubrr = ubrr_a;
	} else {
		error = (error_b * 10.0).round() / 10.0;
		ubrr = ubrr_b;
	}
	if ubrr < 1.0 {
		return;
	}

	if error.abs() > 2.0 {
		color = TERM_RED;
	}

	if error.abs() < 0.2 {
		color = TERM_GRN;
	}

	println!("{} Baudrate: {:6}, U2X: {}, UBRR: 0x{:04x}, error: {:+.1}%{}", color, baudrate, u2x, ubrr as u16, error, TERM_RST);
}

fn main() {
	let mut baudrates: Vec<f32> = Vec::new();
	let mut clockrate: f32 = CLOCKRATE;

	baudrates.push(0.3e3);
	baudrates.push(0.6e3);
	baudrates.push(1.2e3);
	baudrates.push(2.4e3);
	baudrates.push(4.8e3);
	baudrates.push(9.6e3);
	baudrates.push(14.4e3);
	baudrates.push(19.2e3);
	baudrates.push(28.8e3);
	baudrates.push(38.4e3);
	baudrates.push(57.6e3);
	baudrates.push(76.8e3);
	baudrates.push(115.2e3);
	baudrates.push(230.4e3);
	baudrates.push(250.0e3);
	baudrates.push(1.0e6);

	let args: Vec<String> = env::args().collect();
	if args.len() > 1 {
		let num = args[1].parse::<f32>();
		match num {
			Ok(val) => clockrate = val,
			_       => println!("{}No valid clockrate given. Falling back to {}Hz{}", TERM_RED
,  CLOCKRATE, TERM_RST),
		}
	}

	println!("Clockrate: {}Hz", clockrate);


	for i in 0..baudrates.len() {
		calc_baudrate(baudrates[i], clockrate, false);
	}
	for i in 0..baudrates.len() {
		calc_baudrate(baudrates[i], clockrate, true);
	}
}
