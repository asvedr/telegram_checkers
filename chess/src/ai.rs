use solve::*;
use field::*;
use rand::*;

pub struct TNode {
	x         : usize, // prev crd
	y         : usize, // prev crd
	nx        : usize, // this state crd
	ny        : usize, // this state crd
	killed    : Box<Vec<(usize,usize)>>,
	data      : Field,
	self_cap  : isize,
	max_on_way: Option<(isize,usize)>,
	//min_on_way: Option<(isize,usize)>,
	step_color: isize, // (white,1) (black,-1)
	childs    : Vec<Step>
}

impl TNode {
	pub fn max_val(&self) -> isize {
		match self.max_on_way {
			Some((a,_)) => a,
			_ => self.self_cap
		}
	}
	//pub fn min_val(&self) -> isize {
		//match self.min_on_way {
			//Some((a,_)) => a,
			//_ => - self.self_cap
		//}
	//}
	pub fn max_child(&self) -> Option<&TNode> {
		match self.max_on_way {
			Some((_,ref i)) => Some(&*self.childs[*i]),
			_ => None
		}
	}
	//pub fn min_child(&self) -> Option<&TNode> {
		//match self.min_on_way {
			//Some((_,ref i)) => Some(&*self.childs[*i]),
			//_ => None
		//}
	//}
	pub fn coords_from(&self) -> (usize, usize) {
		(self.x, self.y)
	}
	pub fn coords_to(&self) -> (usize,usize) {
		(self.nx, self.ny)
	}
}
/*
fn log_down(node : &TNode) {
	println!("log:\n{}", node.data.show());
	match node.max_child() {
		Some(link) => log_down(link),
		_ => ()
	}
}
*/
pub type Step = Box<TNode>;

#[derive(Clone,Debug)]
pub enum Color {
	White,
	Black
}

impl Color {
	pub fn to_isize(&self) -> isize {
		match *self {
			Color::White => 1,
			_ => -1
		}
	}
	pub fn from_isize(i : isize) -> Option<Color> {
		match i {
			1  => Some(Color::White),
			-1 => Some(Color::Black),
			_  => None
		}
	}
	pub fn neg(&self) -> Color {
		match *self {
			Color::White => Color::Black,
			_ => Color::White
		}
	}
}

//                                                                                                  x,    y,    new_x,new_y,killed[(x,y)]
pub fn next_move(fld : &Field, color : Color, max_depth : usize, heuristic_diff : isize) -> Option<(usize,usize,usize,usize,Box<Vec<(usize,usize)>>)> {
	fn rec_down(fld : &Field, color : isize, depth : usize, heuristic_diff : isize, max_white : isize, max_black : isize) ->
		Vec<Step>
	{
		let mut exist_kills = false;
		let mut res = Vec::new();
		let my = if color == 1 {&*fld.white} else {&*fld.black};
		let cache = fld.cache_field();
		for i in 0 .. 12 {
			let ch = &my[i];
			if ch.live {
				let mut steps = all_moves(ch.x, ch.y, color, ch.king, &cache);
				// нужно проверить, если есть ходы где надо бить: оставить только их
				if exist_kills {
					if steps.len() > 0 && (match steps[0]{(_,_,ref v) => v.len() == 0}) {
						// надо бить, а у этой шашки таких ходов нету
						steps.clear()
					}
				}
				else {
					if steps.len() > 0 {
						match steps[0] {
							(_,_,ref v) => if v.len() > 0 {
								/* был обнаружен ПЕРВЫЙ ход, где надо бить
								 * значит в векторе только ходы без битья
								 * значит их нужно удалить, поскольку если можно бить - обязан бить
								 */
								exist_kills = true;
								res.clear();
							}
						}
					}
				}
				for item in steps {
					let (nx, ny, killed) = item;
					let mut new_fld = fld.clone();
					{ // making new_fld
						let (ch_m, en) =
							if color == 1
								{(&mut new_fld.white[i], &mut *new_fld.black)}
							else
								{(&mut new_fld.black[i], &mut *new_fld.white)};
						ch_m.x = nx;
						ch_m.y = ny;
						match color {
							1  if ch_m.y == 0 => ch_m.king = true,
							-1 if ch_m.y == 7 => ch_m.king = true,
							_ => ()
						}
						for item in killed.iter() {
							let (cx,cy) = item.clone();
							'local : for i in 0 .. 12 {
								if en[i].live && en[i].x == cx && en[i].y == cy {
									en[i].live = false;
									break 'local;
								}
							}
						}
					}
					let cap = capacity(&new_fld) * color;
					let mut max_on_way = None;
					//let mut min_on_way = None;
					let mut worst_ch = 0;
					let mut childs = Vec::new();
					let max_cap = if color == 1 {max_white} else {max_black};
					if max_cap - cap > heuristic_diff || depth < 1 || cap >= WIN_PRICE || cap <= -WIN_PRICE
						{} // don't make childs
					else {
						let max_cap = if cap > max_cap {cap} else {max_cap};
						let (w,b) = if color == 1 {(max_cap,max_black)} else {(max_white,max_cap)};
						childs = rec_down(&new_fld, -color, depth - 1, heuristic_diff, w, b);
						if childs.len() > 0 {
							// search max for enemy
							let mut worst_cap = childs[0].max_val();
							for i in 0 .. childs.len(){ 
								if childs[i].max_val() > worst_cap {
									worst_ch = i;
									worst_cap = childs[i].max_val()
								}
							}
							//min_on_way = Some(- childs[worst_ch].max_val());
							//max_on_way = Some(- childs[worst_ch].min_val());
							max_on_way = Some(- worst_cap)
						}
					}
					//match (min_on_way.clone(), max_on_way.clone()) {
						//(Some(a),Some(b)) => if a != -b {panic!()},
						//_ => ()
					//}
					let node = Box::new(TNode {
											x         : ch.x,
											y         : ch.y,
											nx        : nx,
											ny        : ny,
											killed    : killed,
											data      : new_fld,
											self_cap  : cap,
											max_on_way: match max_on_way {
															Some(a) => Some((a, worst_ch)),
															_ => None
														},
											//min_on_way: match min_on_way {
															//Some(a) => Some((a, worst_ch)),
															//_ => None
														//},
											step_color: color,
											childs    : childs
										});
					res.push(node);
				}
			}
		}
		return res;
	}
	let color =
		match color {
			Color::White => 1,
			Color::Black => -1
		};
	let cur_cap = capacity(fld) * color;
	let (w,b) = if color == 1 {(cur_cap, -cur_cap)} else {(-cur_cap, cur_cap)};
	let mut vec = rec_down(fld, color, max_depth, heuristic_diff, w, b);
	//println!("{}",cur_cap);
	if vec.len() > 0 {
	/*
		let mut maxi = 0;
		for i in 0 .. vec.len() {
			if vec[i].max_val() > vec[maxi].max_val()
				{maxi = i}
		}
		let res = vec.remove(maxi);
		//log_down(&res);
		return Some((res.x, res.y, res.nx, res.ny, res.killed));
	*/
		let mut maxc = vec[0].max_val();
		for i in 0 .. vec.len() {
			if vec[i].max_val() > maxc
				{maxc = vec[i].max_val()}
		}
		let mut resl = vec![];
		loop {
			match vec.pop() {
				None => break,
				Some(item) =>
					if item.max_val() == maxc
						{resl.push(item)}
			}
		}
		let i = random::<usize>() % resl.len();
		let res = resl.remove(i);
		return Some((res.x, res.y, res.nx, res.ny, res.killed))
	}
	else {
		return None;
	}
}
