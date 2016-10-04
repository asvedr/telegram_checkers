use field::*;
use solve::*;
use ai::*;
use std::io;
use std::fs::File;
use std::io::Write;

pub fn redraw(f : &Field, i : usize, j : usize, grabbed : bool, steps : Option<&Vec<(usize,usize)>>, cross : Option<(usize,usize)>) {
	let def = Vec::new();
	let ptr = match steps {
				Some(p) => p,
				_ => &def
			};
	println!("{}",f.show_crd(i,j,grabbed,ptr,cross));
}

pub fn redraw_from(f : &Field, i : usize, j : usize, cross : Option<(usize,usize)>) {
	let mut steps = Vec::new();
	{
		let (ptr,_) = match f.get_ch(j,i) {
						None => panic!(),
						Some(a) => a
					};
		for item in moves(ptr, f) {
			let (x,y,_) = item;
			steps.push((x,y))
		}
	}
	redraw(f, i, j, true, Some(&steps), cross)
}

// если есть ходы где бить, то только эти фигуры и разрешить!
fn available_chess(f : &Field, color : Color) -> Vec<(usize,usize)> {
	let cache = f.cache_field();
	let arr : &[Checker;12] = match color {Color::White => &*f.white, _ => &*f.black};
	let mut exist_kills = false;
	let mut res = vec![];
	for c in arr.iter() {
		if c.live {
			let steps = all_moves(c.x, c.y, color.to_isize(), c.king, &cache);
			if steps.len() > 0 {
				match steps[0] {
					(_,_,ref v) =>
						if exist_kills {
							if v.len() > 0
								{res.push((c.x,c.y))}
						}
						else {
							if v.len() == 0
								{res.push((c.x, c.y))}
							else{
								exist_kills = true;
								res.clear();
								res.push((c.x,c.y));
							}
						}
				}
			}
		}
	}
	res
}

pub fn user_step(f : &mut Field, color : Color) {
	let mut i = 0;
	let mut j = 0;
	let inp = io::stdin();
	let mut line = String::new();
	let available = available_chess(f, color.clone());
	'bigloop : loop {
		line.clear();
		redraw(f,i,j,false,None,None);
		let _ = inp.read_line(&mut line);
		let chars : Vec<char> = line.chars().collect();
		for c in chars {
			match c {
				'\n' => (),
				'w' if i > 0 => i -= 1,
				's' if i < 7 => i += 1,
				'a' if j > 0 => j -= 1,
				'd' if j < 7 => j += 1,
				' ' =>
					match f.get_ch(j,i) {
						Some((_,_)) => {
							for item in available.iter() {
								match *item {
									(ref x, ref y) =>
										if *x == j && *y == i
											{break 'bigloop}
								}
							}
							println!("bad cell")
						},
							//if c == color.to_isize()
								//{break 'bigloop}
							//else
								//{println!("bad cell")},
						_ => println!("bad cell")
					},
				_ => println!("bad command")
			}
		}
	}
	let grab_i = i;
	let grab_j = j;
	let steps;
	let mut s_to_show = Vec::new();
	{
		let (ptr,_) = f.get_ch(j,i).unwrap();
		steps = moves(ptr, f);
		for item in steps.iter() {
			let (ref x, ref y, _) = *item;
			s_to_show.push((*x,*y));
		}
	}
	loop {
		line.clear();
		redraw(f,i,j,true,Some(&s_to_show),None);
		println!("{:?}", steps);
		let _ = inp.read_line(&mut line);
		let chars : Vec<char> = line.chars().collect();
		for c in chars {
			match c {
				'\n' => (),
				'w' if i > 0 => i -= 1,
				's' if i < 7 => i += 1,
				'a' if j > 0 => j -= 1,
				'd' if j < 7 => j += 1,
				' ' => {
					for n in 0 .. steps.len() {
						let (ref x, ref y, ref k) = steps[n];
						if i == *y && j == *x {
							println!("mv(user) {},{} => {},{}", grab_j, grab_i, j, i);
							for item in k.iter() {
								let (ref x, ref y) = *item;
								println!("kill(user) {},{}", *x, *y);
								f.kill(*x,*y);
							}
							match f.get_ch_mut(grab_j, grab_i){
								Some((ch,_)) => {
									ch.x = j;
									ch.y = i;
									match color {
										Color::White if ch.y == 0 => ch.king = true,
										Color::Black if ch.y == 7 => ch.king = true,
										_ => ()
									}
								},
								_ => panic!()
							}
							return ();
						}
					}
					println!("bad cell")
				},
				_ => println!("bad command")
			}
		}
	}
}

pub fn ai_step(fld : &mut Field, c : Color, depth : usize, heuristic : isize) {
	match next_move(fld, c.clone(), depth, heuristic) {
		Some((x,y,nx,ny,killed)) => {
			println!("mv(ai) {},{} => {},{}", x, y, nx ,ny);
			for item in killed.iter() {
				let (ref x, ref y) = *item;
				println!("kill(ai) {},{}", *x, *y);
				fld.kill(*x,*y);
			}
			match fld.get_ch_mut(x,y) {
				Some((ch,_)) => {
					ch.x = nx;
					ch.y = ny;
					match c {
						Color::White if ch.y == 0 => ch.king = true,
						Color::Black if ch.y == 7 => ch.king = true,
						_ => ()
					}
				},
				_ => panic!()
			}
		},
		_ => println!("no moves for {:?}", c)
	}
}

pub fn single_play(color : Color, depth : usize, heuristic : isize, mut logfile : Option<File>) {
	let mut fld = Field::new();
	let c = capacity(&fld) * color.to_isize();
	println!("{}\n>> {}",fld.show(),c);
	fn log(file : &mut Option<File>, f : &Field, text : &str) {
		match *file {
			None => (),
			Some(ref mut file) => {
				let _ = file.write_fmt(format_args!("{}\n{}\n",text,f.show()));
				let _ = file.flush();
			}
		}
	}
	log(&mut logfile, &fld, "init");
	loop {
		if c >= WIN_PRICE {
			println!("YOU WIN");
			break;
		}
		if c <= - WIN_PRICE {
			println!("YOU LOOSE");
			break;
		}
		match color {
			Color::White => {
				user_step(&mut fld, color.clone());
				log(&mut logfile, &fld, "user, white");
				ai_step(&mut fld, color.neg(), depth, heuristic);
				log(&mut logfile, &fld, "ai, black");
			},
			Color::Black => {
				ai_step(&mut fld, color.neg(), depth, heuristic);
				log(&mut logfile, &fld, "ai, white");
				user_step(&mut fld, color.clone());
				log(&mut logfile, &fld, "user, black");
			}
		}
	}
}
