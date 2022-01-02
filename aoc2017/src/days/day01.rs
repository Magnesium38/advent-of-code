use itertools::Itertools;

pub fn pt1(input: &str) -> anyhow::Result<isize> {
	Ok(input
		.chars()
		.chain(input.chars().take(1))
		.tuple_windows()
		.map(|(a, b)| {
			if a == b {
				a.to_digit(10).unwrap() as isize
			} else {
				0
			}
		})
		.sum())
}

pub fn pt2(input: &str) -> anyhow::Result<isize> {
	Ok(input
		.chars()
		.zip(input.chars().cycle().skip(input.len() / 2))
		.map(|(a, b)| {
			if a == b {
				a.to_digit(10).unwrap() as isize
			} else {
				0
			}
		})
		.sum())
}

advent::problem!(
	r#"
		123123
    "#,
	0,
	12,
);
