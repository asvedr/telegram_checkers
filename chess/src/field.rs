#[derive(Clone,Copy)]
pub struct Checker {
	pub x    : usize,
	pub y    : usize,
	pub king : bool,
	pub live : bool
}

pub struct Field {
	pub white      : Box<[Checker;12]>,
	pub black      : Box<[Checker;12]>,
//	pub step_white : bool
}

/* INIT STATE:

	0 .b.b.b.b
	1 b.b.b.b.
	2 .b.b.b.b
	3 ........
	4 ........
	5 w.w.w.w.
	6 .w.w.w.w
	7 w.w.w.w.

	  01234567
*/

// empty cell : 0
// cheker : 1 | (-1)  + whine, - back
// king   : 2 | (-2)


impl Clone for Field {
	fn clone(&self) -> Field {
		let mut white = Box::new([Checker{x:0, y:0, king: false, live: true};12]);
		let mut black = Box::new([Checker{x:0, y:0, king: false, live: true};12]);
		for i in 0 .. 12 {
			white[i] = self.white[i].clone();
			black[i] = self.black[i].clone();
		}
		Field{
//			step_white : self.step_white,
			white : white,
			black : black
		}
	}
}

pub fn clone_cache(f : &[[isize;8];8]) -> Box<[[isize;8];8]> {
	let mut res = Box::new([[0;8];8]);
	for i in 0 .. 8 {
		for j in 0 .. 8 {
			res[i][j] = f[i][j]
		}
	}
	return res;
}

impl Field {
	pub fn empty() -> Field {
		Field {
			white : Box::new([Checker{x:0, y:0, king: false, live: false};12]),
			black : Box::new([Checker{x:0, y:0, king: false, live: false};12])
		}
	}
	pub fn new() -> Field {
		let mut white = Box::new([Checker{x:0, y:0, king: false, live: true};12]);
		let mut black = Box::new([Checker{x:0, y:0, king: false, live: true};12]);
		fn mk_line(arr : &mut [Checker;12], from : usize, y : usize, even : bool) {
			for i in 0 .. 4 {
				let ch = &mut arr[from + i];
				ch.y = y;
				ch.x = if even {i * 2} else {(i + 1) * 2 - 1};
			}
		}
		mk_line(&mut *white, 0, 5, false);
		mk_line(&mut *white, 4, 6, true);
		mk_line(&mut *white, 8, 7, false);
		mk_line(&mut *black, 0, 0, true);
		mk_line(&mut *black, 4, 1, false);
		mk_line(&mut *black, 8, 2, true);
		Field{
			white: white,
			black: black,
//			step_white: true
		}
	}
	pub fn read(text : &str) -> Field {
		let mut white = Box::new([Checker{x:0, y:0, king: false, live: false};12]);
		let mut black = Box::new([Checker{x:0, y:0, king: false, live: false};12]);
		let mut lines : Vec<&str> = vec![];
		for line in text.split("\n") {
			if line.len() > 2 
				{lines.push(line)}
		};
		let mut white_n = 0;
		let mut black_n = 0;
		for i in 0 .. 8 {
			println!("'{}'",lines[i]);
			let line : Vec<char> = lines[i].chars().collect();
			for j in 0 .. 8 {
				match line[j] {
					'W'=> {
						white[white_n].live = true;
						white[white_n].king = true;
						white[white_n].x = j;
						white[white_n].y = i;
						white_n += 1;
					},
					'w'=> {
						white[white_n].live = true;
						white[white_n].x = j;
						white[white_n].y = i;
						white_n += 1;
					},
					'b'=> {
						black[black_n].live = true;
						black[black_n].x = j;
						black[black_n].y = i;
						black_n += 1;
					},
					'B'=> {
						black[black_n].live = true;
						black[black_n].king = true;
						black[black_n].x = j;
						black[black_n].y = i;
						black_n += 1;
					},
					_  => ()
				}
			}
		}
		Field{
			white: white,
			black: black,
//			step_white: true
		}
	}
	pub fn cache_field(&self) -> Box<[[isize;8];8]> {
		let mut res = Box::new([[0;8];8]);
		for i in 0 .. 12 {
			let w = &self.white[i];
			if w.live {
				res[w.y][w.x] = if w.king {2} else {1};
			}
			let b = &self.black[i];
			if b.live {
				res[b.y][b.x] = if b.king {-2} else {-1};
			}
		}
		return res;
	}
	pub fn show(&self) -> String {
		let cache = self.cache_field();
		let mut res = String::new();
		for i in 0 .. 8 {
			let mut line = String::new();
			for j in 0 .. 8 {
				let sh = 
					match cache[i][j] {
						0 => if (i + j) % 2 == 0 {"."} else {" "},
						1 => "w",
						2 => "W",
						-1=> "b",
						-2=> "B",
						_ => panic!()
					};
				line = format!("{}{}", line, sh);
			}
			res = format!("{}{}\n", res, line)
		}
		return res;
	}
	pub fn show_crd(&self, ci : usize, cj : usize, grabbed : bool, mark : &Vec<(usize,usize)>, cross : Option<(usize,usize)>) -> String {
		let cache = self.cache_field();
		let mut res = String::new();
		let hlim = |i : isize, j : isize| {
			if i == ci as isize {
				if j == cj as isize
					{if grabbed {")"} else {"<"}}
				else if j == (cj as isize) - 1
					{if grabbed {"("} else {">"}}
				else {"|"}
			}
			else {"|"}
		};
		let vlim = |i : isize, j : isize| {
			if j == cj as isize {
				if i == ci as isize {"+^"}
				else if i == (ci as isize) - 1 {"+v"}
				else {"+-"}
			}
			else {"+-"}
		};
		let i = -1;
		for j in 0 .. 8 {
			res = format!("{}{}",res,vlim(i, j as isize));
		}
		res = format!("{}\n",res);
		for i in 0 .. 8 {
			let mut line = format!("{}",hlim(i as isize,-1));
			let mut suff_line = String::new();
			for j in 0 .. 8 {
				let mut flag = false;
				let mut crossf = false;
				match cross {
					Some((ref x, ref y)) => crossf = *x == j && *y == i,
					_ => ()
				}
				if !crossf {
					for item in mark {
						let (j1,i1) = *item;
						flag = flag || (i == i1 && j == j1)
					}
				}
				let sh = 
					match cache[i][j] {
						_ if crossf => "x",
						_ if flag => "@",
						0 => if (i + j) % 2 == 0 {"."} else {" "},
						1 => "w",
						2 => "W",
						-1=> "b",
						-2=> "B",
						_ => panic!()
					};
				line = format!("{}{}{}", line, sh, hlim(i as isize, j as isize));
				suff_line = format!("{}{}", suff_line, vlim(i as isize, j as isize));
			}
			res = format!("{}{}\n{}\n", res, line, suff_line)
		}
		return res;
	}
	/*pub fn step_ind(&mut self, i : usize, x : usize, y : usize) {
		let f = if self.step_white {&mut *self.white} else {&mut *self.black};
		assert!(f[i].live);
		f[i].x = x;
		f[i].y = y;
		self.step_white = !self.step_white;
	}
	pub fn step_crd(&mut self, x : usize, y : usize, nx : usize, ny : usize) {
		let (f,nf) = if self.step_white {(&mut *self.white, &mut *self.black)}
					 else {(&mut *self.black, &mut *self.white)};
		for i in 0 .. 12 {
			if f[i].live && f[i].x == x && f[i].y == y {
				f[i].x = nx;
				f[i].y = ny;
				self.step_white = !self.step_white;
				return();
			}
			if nf[i].live && nf[i].x == x && nf[i].y == y
				{panic!()}
		}
		panic!();
	}
	*/
	pub fn get_ch(&self, x : usize, y : usize) -> Option<(&Checker, isize)> {
		for i in 0 .. 12 {
			let ch : &Checker = &self.white[i];
			if ch.live && ch.x == x && ch.y == y
				{return Some((ch,1))}
			let ch : &Checker = &self.black[i];
			if ch.live && ch.x == x && ch.y == y
				{return Some((ch,-1))}
		}
		return None;
	}
	pub fn get_ch_mut(&mut self, x : usize, y : usize) -> Option<(&mut Checker, isize)> {
		for i in 0 .. 12 {
			if self.white[i].live && self.white[i].x == x && self.white[i].y == y {
				let mut ch : &mut Checker = &mut self.white[i];
				return Some((ch,1))
			}
			if self.black[i].live && self.black[i].x == x && self.black[i].y == y {
				let mut ch : &mut Checker = &mut self.black[i];
				return Some((ch,-1))
			}
		}
		return None;
	}
	pub fn kill(&mut self, x : usize, y : usize) {
		for i in 0 .. 12 {
			let ch = &mut self.white[i];
			if ch.live && ch.x == x && ch.y == y {
				ch.live = false;
				return();
			}
			let ch = &mut self.black[i];
			if ch.live && ch.x == x && ch.y == y {
				ch.live = false;
				return();
			}
		}
		panic!("checker for kill not found");
	}
	/*pub fn step(&mut self) {
		self.step_white = !self.step_white;
	}
	*/
}
