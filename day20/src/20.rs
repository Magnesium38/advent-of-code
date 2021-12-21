use itertools::Itertools;

pub fn pt1(input: &str) -> anyhow::Result<usize> {
	let (algorithm, mut image) = parse_input(input);

	let toggle = *algorithm.first().unwrap();
	for _ in 0..1 {
		image = enhance(image, &algorithm, false);
		image = enhance(image, &algorithm, toggle);
	}

	Ok(image.iter().filter(|(_, &v)| v).count())
}

pub fn pt2(input: &str) -> anyhow::Result<usize> {
	let (algorithm, mut image) = parse_input(input);

	let toggle = *algorithm.first().unwrap();
	for _ in 0..25 {
		image = enhance(image, &algorithm, false);
		image = enhance(image, &algorithm, toggle);
	}

	Ok(image.iter().filter(|(_, &v)| v).count())
}

fn parse_input(input: &str) -> (Vec<bool>, advent::Grid<bool>) {
	let (algorithm, input) = input.split_once("\n\n").unwrap();

	(
		algorithm
			.chars()
			.map(|c| match c {
				'.' => false,
				'#' => true,
				_ => panic!("invalid character '{}'", c),
			})
			.collect_vec(),
		input
			.lines()
			.map(|line| {
				line.chars().map(|c| match c {
					'.' => false,
					'#' => true,
					_ => panic!("invalid character"),
				})
			})
			.into(),
	)
}

fn enhance(
	mut map: advent::Grid<bool>,
	algorithm: &[bool],
	default: bool,
) -> advent::Grid<bool> {
	map.add_row(0, default);
	map.add_row(map.height, default);
	map.add_column(0, default);
	map.add_column(map.width, default);

	let mut new_grid = advent::Grid::new_with_size(map.width, map.height);

	for x in 0..(map.width as isize) {
		for y in 0..(map.height as isize) {
			let mut bit: usize = 0;

			for (dx, dy) in &[
				(-1, -1),
				(0, -1),
				(1, -1),
				(-1, 0),
				(0, 0),
				(1, 0),
				(-1, 1),
				(0, 1),
				(1, 1),
			] {
				bit = (bit << 1)
					+ match map.get(x + dx, y + dy) {
						Some(true) => 1,
						None if default => 1,
						_ => 0,
					};
			}

			new_grid.insert(x, y, algorithm[bit]);
		}
	}

	new_grid
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
