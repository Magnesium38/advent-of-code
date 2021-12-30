use itertools::Itertools;

pub fn pt1(input: &str) -> anyhow::Result<isize> {
	let mut grid: advent::Grid<Option<Cucumber>> = input
		.lines()
		.map(|line| {
			line.chars().map(|c| match c {
				'>' => Some(Cucumber::Right),
				'v' => Some(Cucumber::Down),
				'.' => None,
				_ => panic!("invalid character"),
			})
		})
		.into();

	let width = grid.width as isize;
	let height = grid.height as isize;
	for i in 1.. {
		let right_facing = grid
			.iter()
			.filter(|&(_, c)| c == &Some(Cucumber::Right))
			.collect_vec();
		let mut new_grid = grid.clone();
		let mut moved = false;

		for ((x, y), c) in right_facing {
			let new_x = (x + 1) % width;

			if grid.get(new_x, y).unwrap().is_some() {
				continue;
			}

			new_grid.insert(x, y, None);
			new_grid.insert(new_x, y, *c);
			moved = true;
		}

		grid = new_grid;
		let down_facing = grid
			.iter()
			.filter(|&(_, c)| c == &Some(Cucumber::Down))
			.collect_vec();
		let mut new_grid = grid.clone();
		for ((x, y), c) in down_facing {
			let new_y = (y + 1) % height;

			if grid.get(x, new_y).unwrap().is_some() {
				continue;
			}

			new_grid.insert(x, y, None);
			new_grid.insert(x, new_y, *c);
			moved = true;
		}

		if !moved {
			return Ok(i);
		}

		grid = new_grid;
	}

	unreachable!();
}

pub fn pt2(_input: &str) -> anyhow::Result<isize> {
	Ok(0)
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Cucumber {
	Right,
	Down,
}

advent::problem!(
	r#"
		v...>>.vv>
		.vv>>.vv..
		>>.>v>...v
		>>v>>.>.v.
		v>v.vv.v..
		>.>>..v...
		.vv..>.>v.
		v.v..>>v.v
		....v..v.>
	"#,
	58,
	0,
);
