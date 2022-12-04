use itertools::Itertools;

pub fn pt1(input: &str) -> anyhow::Result<usize> {
	Ok(input
		.lines()
		.map(|line| {
			let (a, b) = line.split(',').collect_tuple().unwrap();

			let (a1, a2) = a
				.split('-')
				.map(|n| n.parse::<usize>())
				.collect_tuple()
				.unwrap();
			let (b1, b2) = b
				.split('-')
				.map(|n| n.parse::<usize>())
				.collect_tuple()
				.unwrap();

			((a1.unwrap(), a2.unwrap()), (b1.unwrap(), b2.unwrap()))
		})
		.map(|((a1, a2), (b1, b2))| {
			if b2 - b1 > a2 - a1 {
				((b1, b2), (a1, a2))
			} else {
				((a1, a2), (b1, b2))
			}
		})
		.filter(|((a1, a2), (b1, b2))| a1 <= b1 && a2 >= b2)
		.count())
}

pub fn pt2(input: &str) -> anyhow::Result<usize> {
	Ok(input
		.lines()
		.map(|line| {
			let (a, b) = line.split(',').collect_tuple().unwrap();

			let (a1, a2) = a
				.split('-')
				.map(|n| n.parse::<usize>())
				.collect_tuple()
				.unwrap();
			let (b1, b2) = b
				.split('-')
				.map(|n| n.parse::<usize>())
				.collect_tuple()
				.unwrap();

			((a1.unwrap(), a2.unwrap()), (b1.unwrap(), b2.unwrap()))
		})
		.filter(|((a1, a2), (b1, b2))| {
			(a1 <= b1 && a2 >= b1)
				|| (a1 <= b2 && a2 >= b2)
				|| (b1 <= a1 && b2 >= a1)
				|| (b1 <= a2 && b2 >= a2)
		})
		.count())
}

advent::problem!(
	r#"
		2-4,6-8
		2-3,4-5
		5-7,7-9
		2-8,3-7
		6-6,4-6
		2-6,4-8
    "#,
	2,
	4,
);
