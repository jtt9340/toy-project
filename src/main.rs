use std::{env, process};
use std::io::{self, Write};

use getopts::{
	HasArg,
	Occur,
	Options,
};

mod text;
mod angle;
mod dragon;

const DEFAULT_PHRASE: &str = "A day in the life of a software engineer";
const DEFAULT_ANGLE: f64 = 90.0;

/// Used to emulate Python's prompt function
fn prompt(message: &str) -> io::Result<String> {
	print!("{}", message);
	io::stdout().flush()?;

	let mut input = String::new();
	io::stdin().read_line(&mut input)?;

	Ok(input)
}

fn get_phrase() -> String {
	match prompt("Enter a phrase: ") {
		Ok(s) => s,
		Err(e) => {
			eprintln!("Could not read user input :(. Here's why: {error}. Using the phrase \"{phrase}\"",
					  error = e, phrase = DEFAULT_PHRASE);
			DEFAULT_PHRASE.to_string()
		},
	}
}

fn get_angle() -> angle::Angle {
	loop {
		match prompt("Now enter an angle. Use \u{00B0} to indicate degrees and \"rad.\" to indicate radians: ") {
			// Reading strings from stdin preserves the newline, so we need to remove that with the .trim_end() method
			Ok(s) => match s.trim_end().parse::<angle::Angle>() {
				Ok(a) => return a,
				Err(e) => {
					eprintln!("{}. Try again.", e);
					continue
				}
			},
			Err(e) => {
				eprintln!("Could not read angle :(. Here's why: {error}. Using the angle {angle}\u{00B0}",
						  error = e, angle = DEFAULT_ANGLE);
				return angle::Angle::Degrees(DEFAULT_ANGLE)
			},
		};
	};
}

fn main() {
	// I guess this is necessary
	turtle::start();

	// Collect the command line arguments into a vec
	let args: Vec<String> = env::args().collect();

	// Specify the command line options
	let mut opts = Options::new();
	opts
		.opt(
			"t",
			"text",
			"autistify and shout a phrase",
			"PHRASE",
			HasArg::Maybe,
			Occur::Optional
		)
		.opt(
			"a",
			"angle",
			"convert between degrees and radians",
			"ANGLE",
			HasArg::Maybe,
			Occur::Optional
		)
		.optflag("d", "dragon", "draw a dragon")
	;

	// If no command line arguments are given
	if args.len() == 1 {
		let phrase = get_phrase();

		println!("{}", text::autistify(&*phrase));
		println!("{}", text::shout(&*phrase));

		return;
	}

	// Parse the command line arguments
	let matches = match opts.parse(&args[1..]) {
		Ok(m) => m,
		Err(f) => {
			// Print usage on bad command line arguments
			eprintln!("{}", f);
			let brief = format!("Usage: {} [options]", args[0].clone());
			print!("{}", opts.usage(&brief));
			process::exit(1);
		},
	};

	// autistify and shout
	if matches.opt_present("t") {
		let phrase = matches.opt_str("t").unwrap_or_else(get_phrase);

		println!("{}", text::autistify(&*phrase));
		println!("{}", text::shout(&*phrase));
	}

	// degrees and radians
	if matches.opt_present("a") {
		let angle = match matches.opt_get::<angle::Angle>("a") {
			Ok(Some(angle)) => angle,
			Ok(None) => get_angle(),
			Err(e) => {
				eprintln!("Could not read angle :(. Here's why: {error}.", error = e);
				get_angle()
			}
		};

		println!("The angle you entered is {}.", if angle.is_degrees() {
			angle.to_radians()
		} else {
			angle.to_degrees()
		});
	}

	// try to draw a dragon
	if matches.opt_present("d") {
		println!("Ooh, a dragon!");
		let mut turtle = dragon::Turtle::new();

		let drawing_config = turtle.drawing_mut();
		drawing_config.set_background_color("#112244");
		drawing_config.set_title("Ooh, a dragon!");
		drawing_config.enter_fullscreen();
		/*drawing_config.set_size(turtle::Size {
			width: 640,
			height: 480,
		});*/

		turtle.pen_up();
		turtle.backward(160.0);
		turtle.right(90.);
		turtle.forward(110.);
		turtle.pen_down();
		turtle.set_speed("faster");

		dragon::dragon(&mut turtle, -90.0, 11, 0., 255.);

		turtle.hide();
	}
}
