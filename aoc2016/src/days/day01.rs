use hashbrown::HashSet;

pub fn pt1(input: &str) -> anyhow::Result<isize> {
	let (mut x, mut y) = (0, 0);
	let mut direction = 0;

	for input in input.split(", ") {
		let (dir, dist) = input.split_at(1);
		let dist = dist.parse::<isize>()?;

		direction = (direction
			+ match dir {
				"R" => 1,
				"L" => 3,
				_ => unreachable!(),
			}) % 4;

		match direction {
			0 => y += dist,
			1 => x += dist,
			2 => y -= dist,
			3 => x -= dist,
			_ => unreachable!(),
		}
	}

	Ok(x.abs() + y.abs())
}

pub fn pt2(input: &str) -> anyhow::Result<isize> {
	let (mut x, mut y): (isize, isize) = (0, 0);
	let mut direction = 0;
	let mut visited = HashSet::new();

	for input in input.split(", ") {
		let (dir, dist) = input.split_at(1);
		let dist = dist.parse::<isize>()?;

		direction = (direction
			+ match dir {
				"R" => 1,
				"L" => 3,
				_ => unreachable!(),
			}) % 4;

		for _ in 0..dist {
			match direction {
				0 => y += 1,
				1 => x += 1,
				2 => y -= 1,
				3 => x -= 1,
				_ => unreachable!(),
			}

			if !visited.insert((x, y)) {
				return Ok(x.abs() + y.abs());
			}
		}
	}

	Err(anyhow::anyhow!("No duplicate found"))
}

advent::problem!(
	r#"
		R8, R4, R4, R8
    "#,
	8,
	4,
);
