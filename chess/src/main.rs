#![allow(dead_code)]
extern crate rand;

mod field;
mod solve;
mod ui;
mod ai;

use field::Field;
use ai::*;
use std::io;
use std::str::FromStr;


fn main() {
	let inp = io::stdin();
	let mut line = String::new();
	let _ = inp.read_line(&mut line);
	let cnt : Vec<&str> = line.split("\n").collect();
	let info = cnt[0];
	let mut args : Vec<&str> = info.split(" ").collect();
	let mut fld = Field::empty();
	let mut wi = 0;
	let mut bi = 0;
	let hard = match usize::from_str(args.remove(0)) {
		Ok(val) => val, _ => panic!()
	};
	let who = if args[0] == "w" {Color::White} else {Color::Black};
	for i in 0 .. ((args.len() - 1) / 4) {
		let n = i * 4 + 1;
		let color = if args[n] == "w" {Color::White} else {Color::Black};
		let x = match usize::from_str(args[n+1]){Ok(a) => a,_ => panic!()};
		let y = match usize::from_str(args[n+2]){Ok(a) => a,_ => panic!()};
		let king = args[n+3] == "k";
		match color {
			Color::White => {
				fld.white[wi].live = true;
				fld.white[wi].king = king;
				fld.white[wi].x = x;
				fld.white[wi].y = y;
				wi += 1;
			},
			Color::Black => {
				fld.black[bi].live = true;
				fld.black[bi].king = king;
				fld.black[bi].x = x;
				fld.black[bi].y = y;
				bi += 1;
			}
		}
	}
	//ai_step(&mut fld, who, 5, 1000);
	match ai::next_move(&fld, who, /*5*/hard, 1000) {
		None => println!("nil"),
		Some((x,y,nx,ny,killed)) => {
			println!("(({},{}),({},{}),{:?})", x, y, nx, ny, killed)
		}
	}
//	println!("{}",fld.show())
}

/*
fn main() {
	let logfile = File::create("log.txt").ok();
	single_play(Color::White, 5, 1000, logfile);
}

*/

/*
fn main() {
	let text = "\n\
#.#.#.#.\n\
.#.#.#.#\n\
#.#.#.b.\n\
.#.#.b.#\n\
#.#.w.#.\n\
.#.#.#.#\n\
#.#.#.#.\n\
.#.w.#.w";
	let mut fld = Field::read(text);
	ai_step(&mut fld, Color::White, 2, 1000);
	println!("{}",fld.show())
}
*/
