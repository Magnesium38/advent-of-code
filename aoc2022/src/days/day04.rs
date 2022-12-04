use itertools::Itertools;

pub fn pt1(input: &str) -> anyhow::Result<usize> {
	Ok(input
		.lines()
		.map(|line| {
			line.split([',', '-'])
				.map(|n| n.parse::<usize>().expect("unable to parse input"))
				.collect_tuple()
				.expect("expected four elements")
		})
		.map(|(a1, a2, b1, b2)| {
			if b2 - b1 > a2 - a1 {
				(b1, b2, a1, a2)
			} else {
				(a1, a2, b1, b2)
			}
		})
		.filter(|(a1, a2, b1, b2)| a1 <= b1 && a2 >= b2)
		.count())
}

pub fn pt2(input: &str) -> anyhow::Result<usize> {
	Ok(input
		.lines()
		.map(|line| {
			line.split([',', '-'])
				.map(|n| n.parse::<usize>().expect("unable to parse input"))
				.collect_tuple()
				.expect("expected four elements")
		})
		.filter(|(a1, a2, b1, b2)| {
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
