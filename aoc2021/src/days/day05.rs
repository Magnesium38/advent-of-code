use hashbrown::HashMap;
use itertools::Itertools;

pub fn pt1(input: &str) -> anyhow::Result<usize> {
	let mut map = HashMap::new();

	Ok(parse_input(input)
		.filter(|(x1, y1, x2, y2)| x1 == x2 || y1 == y2)
		.map(|line| count_entries(line, &mut map))
		.sum())
}

pub fn pt2(input: &str) -> anyhow::Result<usize> {
	let mut map = HashMap::new();

	Ok(parse_input(input)
		.map(|line| count_entries(line, &mut map))
		.sum())
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

fn count_entries(
	(x1, y1, x2, y2): (isize, isize, isize, isize),
	map: &mut HashMap<(isize, isize), isize>,
) -> usize {
	let mut count = 0;
	let dx = (x2 - x1).signum();
	let dy = (y2 - y1).signum();

	let (mut x, mut y) = (x1, y1);
	while (x, y) != (x2 + dx, y2 + dy) {
		let entry = map.entry((x, y)).or_insert(0);
		*entry += 1;
		if *entry == 2 {
			count += 1;
		}

		x += dx;
		y += dy;
	}

	count
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
