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

	let mut max_y = None;

	for starting_y_velocity in 0..500 {
		let mut x_velocity = starting_x_velocity;
		let mut y_velocity = starting_y_velocity;
		let mut max_y_for_shot = isize::MIN;
		let mut position = (0, 0);

		loop {
			if position.0 > x_max || position.1 < y_min {
				break;
			}

			position = (position.0 + x_velocity, position.1 + y_velocity);

			x_velocity += -x_velocity.signum();
			y_velocity -= 1;

			max_y_for_shot = max_y_for_shot.max(position.1);

			if position.0 >= x_min
				&& position.1 >= y_min
				&& position.0 <= x_max
				&& position.1 <= y_max
			{
				match max_y {
					None => {
						max_y = Some(max_y_for_shot);
					}
					Some(current_best) => {
						if max_y_for_shot > current_best {
							max_y = Some(max_y_for_shot);
						}
					}
				}
			}
		}

		if let Some(max_y) = max_y {
			if max_y_for_shot < max_y {
				break;
			}
		}
	}

	max_y.ok_or(anyhow::anyhow!("No max y found"))
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

advent::problem!(
	r#"
		target area: x=20..30, y=-10..-5
	"#,
	45,
	112,
);
