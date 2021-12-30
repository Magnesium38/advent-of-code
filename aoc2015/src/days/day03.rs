use hashbrown::HashMap;

pub fn pt1(input: &str) -> anyhow::Result<usize> {
	let mut grid: HashMap<(isize, isize), usize> = HashMap::new();
	let mut position = (0, 0);

	grid.insert((0, 0), 1);
	input.chars().for_each(|c| {
		match c {
			'^' => position.1 -= 1,
			'v' => position.1 += 1,
			'<' => position.0 -= 1,
			'>' => position.0 += 1,
			_ => unreachable!(),
		};

		*grid.entry(position).or_insert(0) += 1;
	});

	Ok(grid.len())
}

pub fn pt2(input: &str) -> anyhow::Result<usize> {
	let mut grid: HashMap<(isize, isize), usize> = HashMap::new();
	let mut santa_position = (0, 0);
	let mut robo_position = (0, 0);

	grid.insert((0, 0), 1);
	input.chars().enumerate().for_each(|(i, c)| {
		match (i % 2, c) {
			(0, '^') => santa_position.1 -= 1,
			(0, 'v') => santa_position.1 += 1,
			(0, '<') => santa_position.0 -= 1,
			(0, '>') => santa_position.0 += 1,
			(1, '^') => robo_position.1 -= 1,
			(1, 'v') => robo_position.1 += 1,
			(1, '<') => robo_position.0 -= 1,
			(1, '>') => robo_position.0 += 1,
			_ => unreachable!(),
		};

		if i % 2 == 0 {
			*grid.entry(santa_position).or_insert(0) += 1;
		} else {
			*grid.entry(robo_position).or_insert(0) += 1;
		}
	});

	Ok(grid.len())
}

advent::problem!(
	r#"
		^v^v^v^v^v
    "#,
	2,
	11,
);
