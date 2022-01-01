use itertools::Itertools;

pub fn pt1(input: &str) -> anyhow::Result<usize> {
	Ok(input
		.lines()
		.map(|line| {
			let (x, y, z) = line
				.split_whitespace()
				.map(|word| word.parse::<isize>().unwrap())
				.sorted()
				.collect_tuple()
				.unwrap();

			x + y > z
		})
		.filter(|b| *b)
		.count())
}

pub fn pt2(input: &str) -> anyhow::Result<isize> {
	Ok(input
		.lines()
		.tuples::<(_, _, _)>()
		.map(|(a, b, c)| {
			let (x1, x2, x3) = a
				.split_whitespace()
				.map(|word| word.parse::<isize>().unwrap())
				.collect_tuple()
				.unwrap();

			let (y1, y2, y3) = b
				.split_whitespace()
				.map(|word| word.parse::<isize>().unwrap())
				.collect_tuple()
				.unwrap();

			let (z1, z2, z3) = c
				.split_whitespace()
				.map(|word| word.parse::<isize>().unwrap())
				.collect_tuple()
				.unwrap();

			(x1 + y1 > z1 && y1 + z1 > x1 && z1 + x1 > y1) as isize
				+ (x2 + y2 > z2 && y2 + z2 > x2 && z2 + x2 > y2) as isize
				+ (x3 + y3 > z3 && y3 + z3 > x3 && z3 + x3 > y3) as isize
		})
		.sum())
}

advent::problem!(
	r#"
		101 301 501
		102 302 502
		903 303 503
		201 401 601
		202 402 602
		203 403 603
    "#,
	3,
	5,
);
