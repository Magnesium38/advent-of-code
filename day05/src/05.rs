use hashbrown::HashMap;
use itertools::Itertools;

pub fn pt1(input: &str) -> anyhow::Result<isize> {
	Ok(
		from_lines(parse_input(input).filter(|(x1, y1, x2, y2)| x1 == x2 || y1 == y2))
			.iter()
			.fold(
				0,
				|total, (_, &value)| if value > 1 { total + 1 } else { total },
			),
	)
}

pub fn pt2(input: &str) -> anyhow::Result<isize> {
	Ok(from_lines(parse_input(input)).iter().fold(
		0,
		|total, (_, &value)| if value > 1 { total + 1 } else { total },
	))
}

fn parse_input(input: &str) -> impl Iterator<Item = (isize, isize, isize, isize)> + '_ {
	input.lines().map(|line| {
		line.split(" -> ")
			.flat_map(|s| s.split(','))
			.map(|n| n.parse::<isize>().unwrap())
			.collect_tuple()
			.unwrap()
	})
}

fn from_lines<T: Iterator<Item = (isize, isize, isize, isize)>>(
	lines: T,
) -> HashMap<(isize, isize), isize> {
	let mut data = HashMap::new();

	for (x1, y1, x2, y2) in lines {
		let dx = (x2 - x1).signum();
		let dy = (y2 - y1).signum();

		let (mut x, mut y) = (x1, y1);
		while (x, y) != (x2 + dx, y2 + dy) {
			data.entry((x, y)).and_modify(|v| *v += 1).or_insert(1);

			x += dx;
			y += dy;
		}
	}

	data
}

advent::problem!(
	r#"
		0,9 -> 5,9
		8,0 -> 0,8
		9,4 -> 3,4
		2,2 -> 2,1
		7,0 -> 7,4
		6,4 -> 2,0
		0,9 -> 2,9
		3,4 -> 1,4
		0,0 -> 8,8
		5,5 -> 8,2
    "#,
	5,
	12,
);
