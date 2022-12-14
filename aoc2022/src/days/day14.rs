use hashbrown::HashMap;
use itertools::Itertools;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Coordinate {
	x: usize,
	y: usize,
}

impl From<&str> for Coordinate {
	fn from(input: &str) -> Self {
		let (x, y) = input.split_once(',').unwrap();

		Self {
			x: x.parse().unwrap(),
			y: y.parse().unwrap(),
		}
	}
}

impl Coordinate {
	fn new(x: usize, y: usize) -> Self {
		Self { x, y }
	}
}

#[derive(Debug)]
enum Node {
	Rock,
	Sand,
}

fn parse_grid(input: &str) -> (HashMap<Coordinate, Node>, usize) {
	let mut grid = HashMap::new();
	let mut max_y = 0;

	input.lines().for_each(|line| {
		line.split(" -> ")
			.map(Coordinate::from)
			.tuple_windows()
			.for_each(|(p1, p2)| {
				for x in p1.x.min(p2.x)..=p1.x.max(p2.x) {
					for y in p1.y.min(p2.y)..=p1.y.max(p2.y) {
						grid.insert(Coordinate::new(x, y), Node::Rock);
					}
				}

				max_y = max_y.max(p1.y).max(p2.y);
			});
	});

	(grid, max_y + 2)
}

fn drop_sand(
	grid: &mut HashMap<Coordinate, Node>,
	max_y: usize,
	mut coords: Coordinate,
) -> Option<Coordinate> {
	for y in coords.y..=max_y {
		coords.y = y;

		if grid.get(&coords).is_some() {
			if coords.y == 0 {
				return None;
			}

			coords.x -= 1;
			if grid.get(&coords).is_some() {
				coords.x += 2;
				if grid.get(&coords).is_some() {
					coords.x -= 1;
					coords.y -= 1;

					grid.insert(coords, Node::Sand);

					return Some(coords);
				}
			}

			return drop_sand(grid, max_y, coords);
		}
	}

	None
}

fn fill_grid(grid: &mut HashMap<Coordinate, Node>, max_y: usize) -> usize {
	loop {
		if matches!(drop_sand(grid, max_y, Coordinate::new(500, 0)), None) {
			break;
		}
	}

	grid.values()
		.filter(|node| matches!(node, Node::Sand))
		.count()
}

pub fn pt1(input: &str) -> anyhow::Result<usize> {
	let (mut grid, max_y) = parse_grid(input);

	Ok(fill_grid(&mut grid, max_y))
}

pub fn pt2(input: &str) -> anyhow::Result<usize> {
	let (mut grid, max_y) = parse_grid(input);

	for x in (500 - max_y)..=(500 + max_y) {
		grid.insert(Coordinate::new(x, max_y), Node::Rock);
	}

	Ok(fill_grid(&mut grid, max_y))
}

advent::problem!(
	r#"
		498,4 -> 498,6 -> 496,6
		503,4 -> 502,4 -> 502,9 -> 494,9
    "#,
	24,
	93,
);
