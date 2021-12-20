use hashbrown::HashMap;
use itertools::Itertools;

pub fn pt1(input: &str) -> anyhow::Result<usize> {
	let (algorithm, input) = input.split_once("\n\n").unwrap();

	let mut image: HashMap<(isize, isize), bool> =
		HashMap::from_iter(input.lines().enumerate().flat_map(|(i, line)| {
			line.chars().enumerate().map(move |(j, c)| {
				(
					(i as isize, j as isize),
					match c {
						'.' => false,
						'#' => true,
						_ => panic!("invalid character"),
					},
				)
			})
		}));

	let algorithm = algorithm
		.chars()
		.map(|c| match c {
			'.' => false,
			'#' => true,
			_ => panic!("invalid character '{}'", c),
		})
		.collect_vec();

	let toggle = *algorithm.first().unwrap();
	for _ in 0..1 {
		image = enhance(image, &algorithm, toggle && false);
		image = enhance(image, &algorithm, toggle && true);
	}

	Ok(image.iter().filter(|(_, &v)| v).count())
}

pub fn pt2(input: &str) -> anyhow::Result<usize> {
	let (algorithm, input) = input.split_once("\n\n").unwrap();

	let mut image: HashMap<(isize, isize), bool> =
		HashMap::from_iter(input.lines().enumerate().flat_map(|(i, line)| {
			line.chars().enumerate().map(move |(j, c)| {
				(
					(i as isize, j as isize),
					match c {
						'.' => false,
						'#' => true,
						_ => panic!("invalid character"),
					},
				)
			})
		}));

	let algorithm = algorithm
		.chars()
		.map(|c| match c {
			'.' => false,
			'#' => true,
			_ => panic!("invalid character '{}'", c),
		})
		.collect_vec();

	let toggle = *algorithm.first().unwrap();
	for _ in 0..25 {
		image = enhance(image, &algorithm, toggle && false);
		image = enhance(image, &algorithm, toggle && true);
	}

	Ok(image.iter().filter(|(_, &v)| v).count())
}

fn enhance(
	map: HashMap<(isize, isize), bool>,
	algorithm: &Vec<bool>,
	default: bool,
) -> HashMap<(isize, isize), bool> {
	let mut new_map = HashMap::new();

	let min_x = map.keys().map(|&(x, _)| x).min().unwrap();
	let max_x = map.keys().map(|&(x, _)| x).max().unwrap();
	let min_y = map.keys().map(|&(_, y)| y).min().unwrap();
	let max_y = map.keys().map(|&(_, y)| y).max().unwrap();

	for x in min_x - 2..=max_x + 2 {
		for y in min_y - 2..=max_y + 2 {
			let mut bit: usize = 0;


			for (dx, dy) in &[
				(-1, -1),
				(-1, 0),
				(-1, 1),
				(0, -1),
				(0, 0),
				(0, 1),
				(1, -1),
				(1, 0),
				(1, 1),
			] {
				let new_pos = (x + dx, y + dy);

				bit = bit << 1;
				match map.get(&new_pos) {
					Some(true) => {
						bit += 1;
					}
					Some(false) => {}
					None if default => {
						bit += 1;
					}
					_ => {}
				}
			}

			new_map.insert((x, y), algorithm[bit]);
		}
	}

	new_map
}

advent::problem!(
	r#"
		..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

		#..#.
		#....
		##..#
		..#..
		..###
	"#,
	35,
	3351,
);
