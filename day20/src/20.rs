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

	print_map(&image);

	image = enhance(image, &algorithm, false);
	print_map(&image);
	image = enhance(image, &algorithm, true);
	print_map(&image);

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

	for i in 0..25 {
		dbg!(i);

		image = enhance(image, &algorithm, false);
		image = enhance(image, &algorithm, true);
	}

	Ok(image.iter().filter(|(_, &v)| v).count())
}

fn print_map(map: &HashMap<(isize, isize), bool>) {
	let min_x = map.keys().map(|&(x, _)| x).min().unwrap();
	let max_x = map.keys().map(|&(x, _)| x).max().unwrap();
	let min_y = map.keys().map(|&(_, y)| y).min().unwrap();
	let max_y = map.keys().map(|&(_, y)| y).max().unwrap();

	let mut output = String::new();

	for y in min_y - 1..=max_y + 1 {
		for x in min_x - 1..=max_x + 1 {
			output.push(if *map.get(&(y, x)).unwrap_or(&false) {
				'#'
			} else {
				'.'
			});
		}

		output.push('\n');
	}

	println!("\n{}\n", output);
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

	for x in min_x - 1..=max_x + 1 {
		for y in min_y - 1..=max_y + 1 {
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
