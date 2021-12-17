use hashbrown::HashSet;

fn pt1(input: &str) -> anyhow::Result<isize> {
	let (_, input) = input.split_once(": ").unwrap();
	let (x_range, y_range) = input.split_once(", ").unwrap();
	let (_, x_range) = x_range.split_once("=").unwrap();
	let (_, y_range) = y_range.split_once("=").unwrap();
	let (x_min, x_max) = x_range.split_once("..").unwrap();
	let (y_min, y_max) = y_range.split_once("..").unwrap();

	let x_min = x_min.parse::<isize>()?;
	let x_max = x_max.parse::<isize>()?;
	let y_min = y_min.parse::<isize>()?;
	let y_max = y_max.parse::<isize>()?;

	let mut starting_x_velocity = 0;
	while (starting_x_velocity + 1) * (starting_x_velocity + 2) / 2 < x_max {
		starting_x_velocity += 1;
	}

	(0..500)
		.map(|dy| is_valid_shot(starting_x_velocity, dy, x_min, x_max, y_min, y_max))
		.flatten()
		.max()
		.ok_or(anyhow::anyhow!("no valid shots"))
}

fn pt2(input: &str) -> anyhow::Result<usize> {
	let (_, input) = input.split_once(": ").unwrap();
	let (x_range, y_range) = input.split_once(", ").unwrap();
	let (_, x_range) = x_range.split_once("=").unwrap();
	let (_, y_range) = y_range.split_once("=").unwrap();
	let (x_min, x_max) = x_range.split_once("..").unwrap();
	let (y_min, y_max) = y_range.split_once("..").unwrap();

	let x_min = x_min.parse::<isize>()?;
	let x_max = x_max.parse::<isize>()?;
	let y_min = y_min.parse::<isize>()?;
	let y_max = y_max.parse::<isize>()?;

	let mut min_starting_x_velocity = 0;
	while (min_starting_x_velocity) * (min_starting_x_velocity + 1) / 2 < x_min {
		min_starting_x_velocity += 1;
	}
	let max_starting_x_velocity = x_max;

	let mut valid = HashSet::new();
	for starting_x_velocity in min_starting_x_velocity..=max_starting_x_velocity {
		for starting_y_velocity in y_min..10000 {
			let mut position = (0, 0);
			let mut x_velocity = starting_x_velocity;
			let mut y_velocity = starting_y_velocity;

			loop {
				if position.0 > x_max || position.1 < y_min {
					break;
				}

				position = (position.0 + x_velocity, position.1 + y_velocity);

				x_velocity += -x_velocity.signum();
				y_velocity -= 1;

				if position.0 >= x_min
					&& position.1 >= y_min
					&& position.0 <= x_max
					&& position.1 <= y_max
				{
					valid.insert((starting_x_velocity, starting_y_velocity));
					break;
				}
			}
		}
	}

	Ok(valid.len())
}

fn is_valid_shot(
	mut dx: isize,
	mut dy: isize,
	x_target_min: isize,
	x_target_max: isize,
	y_target_min: isize,
	y_target_max: isize,
) -> Option<isize> {
	let mut x = 0;
	let mut y = 0;
	let mut max_y = isize::MIN;

	loop {
		x += dx;
		y += dy;

		dx -= dx.signum();
		dy -= 1;

		if y > max_y {
			max_y = y;
		}

		match (
			x >= x_target_min && x <= x_target_max,
			y >= y_target_min && y <= y_target_max,
		) {
			(true, true) => return Some(max_y),
			(false, _) if dx == 0 => return None,
			(_, false) if dy < 0 && y < y_target_min => return None,
			_ => {}
		}
	}
}

advent::problem!(
	r#"
		target area: x=20..30, y=-10..-5
	"#,
	45,
	112,
);
