use field::*;

//pub const CHECKER_DIFF_PRICE : isize = 200;    // if white has 5 checkers and black has 3 then CHECKER_DIFF_PRICE * 2
pub const CHECKER_DIFF_PRICE : f32 = 2000.0;
pub const KING_PRICE         : isize = 500;    // for existing kings * king count
pub const NEAR_TO_KING_PRICE : isize = 10;     // for regular checker whick moveing to edge of board 
//pub const FREEDOM_PRICE      : isize = 1;      // if checker has no moved - subtract this price
pub const WIN_PRICE          : isize = 99999; // if situation is realy win

// colors : (1, white) (-1, black)



#[inline(always)]
fn check_free(i : isize, j : isize, cache : &[[isize;8];8]) -> bool {
	if i >= 0 && i < 8 && j >= 0 && j < 8
		{cache[i as usize][j as usize] == 0}
	else
		{false}
}
#[inline(always)]
fn check_in(i : isize, j : isize) -> bool {
	i >= 0 && i < 8 && j >= 0 && j < 8
}

#[inline]
fn simple_moves(x : usize, y : usize, color : isize, king : bool, cache : &[[isize;8];8]) -> Vec<(usize,usize)> {
	let mut res = Vec::new();
	res.reserve(15);
	let i = y as isize;
	let j = x as isize;
	if !king {
		if color == 1 { // white
			if check_free(i-1, j-1, cache) {res.push(((j-1) as usize,(i-1) as usize))}
			if check_free(i-1, j+1, cache) {res.push(((j+1) as usize,(i-1) as usize))}
		}
		else { // black
			if check_free(i+1, j-1, cache) {res.push(((j-1) as usize,(i+1) as usize))}
			if check_free(i+1, j+1, cache) {res.push(((j+1) as usize,(i+1) as usize))}
		}
	}
	else {
		let mut x = j + 1;
		let mut y = i + 1;
		while check_free(y,x,cache) {
			res.push((x as usize,y as usize));
			x += 1;
			y += 1;
		}
		let mut x = j + 1;
		let mut y = i - 1;
		while check_free(y,x,cache) {
			res.push((x as usize,y as usize));
			x += 1;
			y -= 1;
		}
		let mut x = j - 1;
		let mut y = i + 1;
		while check_free(y,x,cache) {
			res.push((x as usize,y as usize));
			x -= 1;
			y += 1;
		}
		let mut x = j - 1;
		let mut y = i - 1;
		while check_free(y,x,cache) {
			res.push((x as usize,y as usize));
			x -= 1;
			y -= 1;
		}
	}
	return res;
}

#[inline]
fn kill_moves(x : isize, y : isize, color : isize, king : bool, cache : &[[isize;8];8], acc : Option<&Vec<(usize,usize)>>, prev : Option<(isize,isize)>) ->
		Vec<(usize,usize,Box<Vec<(usize,usize)>>)> {
	if king {
		let mut res = vec![];
		for item in [(1,-1),(-1,-1),(-1,1),(1,1)].iter() {
			let (px,py) = *item;
			match prev {
				Some((ref px1, ref py1)) =>
					if *px1 == - px && *py1 == - py
						{continue},
				_ => ()
			}
			let mut mul = 1;
			'while_l : while check_in(x + px * (mul + 1), y + py * (mul + 1)) { // в этом цикле идем в одном направлении
				let c = cache[(y + py * mul) as usize][(x + px * mul) as usize];
				if c != 0 {
					if c.signum() != color { // found enemy
						// making way for next step
						let mut cells = vec![];
						let enemy_y = (y + py * mul) as usize;
						let enemy_x = (x + px * mul) as usize;
						mul += 1;
						'local : while check_in(x + px * mul, y + py * mul) {
							if cache[(y + py * mul) as usize][(x + px * mul) as usize] != 0
								{break 'local}
							else
								{cells.push((x + px * mul, y + py * mul))}
							mul += 1;
						}
						if cells.len() == 0
							{break 'while_l}
						// now try new step from every elem in 'cells' array
						let mut killed = match acc {
								Some(p) => Box::new(p.clone()),
								_  => Box::new(vec![])
						};
						killed.push((enemy_x, enemy_y));
						let mut cache = clone_cache(cache);
						cache[y as usize][x as usize] = 0;
						cache[enemy_y][enemy_x] = 0;
						let mut flag = false; // does king need make one more kill
						for i in 0 .. cells.len() {
							let (x,y) = cells[i].clone();
							for item in kill_moves(x, y, color, king, &cache, Some(&killed), Some((px,py))) {
								res.push(item);
								flag = true;
							}
						}
						if !flag {
							for item in cells {
								let (x,y) = item;
								res.push((x as usize, y as usize, killed.clone()))
							}
						}
					}
					break 'while_l // поскольку преграда найдена, дальше идти в этом направлении не имеет смысла
				};
				mul += 1;
			}
		}
		return res;
	}
	else {
		let mut res = vec![];
		for item in [(1,-1),(-1,-1),(-1,1),(1,1)].iter() {
			let (px,py) = *item;
			if check_in(x + px * 2, y + py * 2){
				let c = cache[(y + py) as usize][(x + px) as usize];
				let dst = cache[(y + py * 2) as usize][(x + px * 2) as usize];
				if c != 0 && c.signum() != color && dst == 0 {
					let mut killed = match acc {
						Some(p) => Box::new(p.clone()),
						_ => Box::new(vec![])
					};
					killed.push(((x + px) as usize, (y + py) as usize));
					let mut cache = clone_cache(cache);
					cache[y as usize][x as usize] = 0;
					cache[(y + py) as usize][(x + px) as usize] = 0;
					cache[(y + py * 2) as usize][(x + px * 2) as usize] = color;
					let mut flag = true;
					for item in kill_moves(x + px * 2, y + py * 2, color, king, &cache, Some(&killed), None) {
						res.push(item);
						flag = false;
					}
					if flag
						{res.push(((x + px * 2) as usize, (y + py * 2) as usize, killed))};
				}
			}
		}
		return res;
	}
}

#[inline(always)]
pub fn all_moves(x : usize, y : usize, color : isize, king : bool, fld : &[[isize;8];8]) ->
		Vec<(usize,usize,Box<Vec<(usize,usize)>>)> {
	let mut res = kill_moves(x as isize, y as isize, color, king, fld, None, None);
	if res.len() > 0
		{return res}
	for item in simple_moves(x, y, color, king, fld) {
		let (x,y) = item;
		res.push((x,y,Box::new(Vec::new())));
	}
	return res;
}

pub fn moves(ch : &Checker, fld : &Field) -> Vec<(usize,usize,Box<Vec<(usize,usize)>>)> {
	let cache = fld.cache_field();
	let (_,color) = fld.get_ch(ch.x, ch.y).unwrap();
	all_moves(ch.x, ch.y, color, ch.king, &cache)
}

/* for you, mr. White */
#[inline(always)]
pub fn capacity(fld : &Field) -> isize {
	let mut result;
	let mut white : Vec<&Checker> = Vec::new();
	let mut black : Vec<&Checker> = Vec::new();
	for i in 0 .. 12 {
		if fld.white[i].live
			{white.push(&fld.white[i])};
		if fld.black[i].live
			{black.push(&fld.black[i])}
	}
	// CHECKER DIFF
//	result = ((white.len() as isize) - (black.len() as isize)) * CHECKER_DIFF_PRICE;
	let wlen = white.len() as f32;
	let blen = black.len() as f32;
	result = ((wlen - blen) / wlen.max(blen) * CHECKER_DIFF_PRICE) as isize;
	let cache = fld.cache_field();
	// KINGS & NEAR_TO_KING & FREEDOM
	let mut white_moves = 0;
	for ch in white {
		if !ch.live
			{continue}
		// KING
		if ch.king
			{result += KING_PRICE}
		// NEAR
		else
			{result += (7 - (ch.y as isize)) * NEAR_TO_KING_PRICE}
		// FREEDOM
		let moves = all_moves/*simple*/(ch.x, ch.y, 1, ch.king, &*cache);
		white_moves += moves.len();
//		if moves.len() == 0
//			{result -= FREEDOM_PRICE}
		
	}
	let mut black_moves = 0;
	for ch in black {
		if !ch.live
			{continue}
		// KING
		if ch.king
			{result -= KING_PRICE}
		// NEAR
		else
			{result -= (ch.y as isize) * NEAR_TO_KING_PRICE}
		// FREEDOM
		let moves = all_moves/*simple*/(ch.x, ch.y, -1, ch.king, &*cache);
		black_moves += moves.len();
//		if moves.len() == 0
//			{result += FREEDOM_PRICE}
	}
	// TOTALY WIN/LOOSE CHECK
	if black_moves == 0 {
		return WIN_PRICE + 1;
	}
	else if white_moves == 0{
		return - (WIN_PRICE + 1);
	}
	else {
		return result;
	}
}

pub fn capacity_white(f : &Field) -> isize {capacity(f)}
pub fn capacity_black(f : &Field) -> isize {- capacity(f)}
