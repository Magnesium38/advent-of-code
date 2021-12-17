use itertools::Itertools;

fn pt1(input: &str) -> anyhow::Result<isize> {
	let ((x_min, x_max), (y_min, y_max)) = get_bounds(input)?;

	let mut starting_x_velocity = 0;
	while (starting_x_velocity + 1) * (starting_x_velocity + 2) / 2 < x_max {
		starting_x_velocity += 1;
	}

	(y_min..=y_min.abs().max(y_max))
		.flat_map(|dy| is_valid_shot(starting_x_velocity, dy, x_min, x_max, y_min, y_max))
		.max()
		.ok_or(anyhow::anyhow!("no valid shots"))
}

fn pt2(input: &str) -> anyhow::Result<usize> {
	let ((x_min, x_max), (y_min, y_max)) = get_bounds(input)?;

	let mut min_starting_x_velocity = 0;
	while (min_starting_x_velocity) * (min_starting_x_velocity + 1) / 2 < x_min {
		min_starting_x_velocity += 1;
	}

	let mut valid = Vec::new();
	(min_starting_x_velocity..=x_max)
		.cartesian_product(y_min..=y_min.abs().max(y_max))
		.for_each(|(dx, dy)| {
			if is_valid_shot(dx, dy, x_min, x_max, y_min, y_max).is_some() {
				valid.push((dx, dy));
			}
		});

	valid.sort_unstable();
	valid.dedup();

	Ok(valid.len())
}

fn get_bounds(input: &str) -> anyhow::Result<((isize, isize), (isize, isize))> {
	let (_, input) = input.split_once(": ").unwrap();
	let (x_range, y_range) = input.split_once(", ").unwrap();
	let (_, x_range) = x_range.split_once("=").unwrap();
	let (_, y_range) = y_range.split_once("=").unwrap();
	let (x_min, x_max) = x_range.split_once("..").unwrap();
	let (y_min, y_max) = y_range.split_once("..").unwrap();

	Ok((
		(x_min.parse::<isize>()?, x_max.parse::<isize>()?),
		(y_min.parse::<isize>()?, y_max.parse::<isize>()?),
	))
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
