use itertools::Itertools;

fn pt1(input: &str) -> anyhow::Result<isize> {
	count_decreases(input.lines().map(|line| line.parse().unwrap()))
}

fn pt2(input: &str) -> anyhow::Result<isize> {
	count_decreases(
		input
			.lines()
			.map(|line| line.parse().unwrap())
			.tuple_windows::<(isize, isize, isize)>()
			.map(|(a, b, c)| a + b + c),
	)
}

fn count_decreases<I: Iterator<Item = isize>>(input: I) -> anyhow::Result<isize> {
	Ok(input
		.tuple_windows::<(isize, isize)>()
		.map(|(a, b)| if a < b { 1 } else { 0 })
		.sum())
}

advent::problem!(
	r#"
        199
        200
        208
        210
        200
        207
        240
        269
        260
        263
    "#,
	7,
	5,
);
