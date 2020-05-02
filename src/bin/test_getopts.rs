use getopts::{Options, HasArg, Occur};

use std::env;

fn main() {
	let args = env::args().collect::<Vec<_>>();

	let mut opts = Options::new();
	opts.opt(
		"m",
		"message",
		"a simple message",
		"MESSAGE",
		HasArg::Maybe,
		Occur::Optional,
	);
	opts.optflag("h", "help", "show this help menu");

	let matches = match opts.parse(&args[1..]) {
		Ok(m) => m,
		Err(f) => panic!(f.to_string()),
	};

	if matches.opt_present("h") {
		println!("{}", opts.usage(""));
	}

	println!("-m given? {}", matches.opt_present("m"));
	println!("value of -m: {:?}", matches.opt_str("m"));
}
