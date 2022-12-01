use itertools::Itertools;

pub fn pt1(input: &str) -> anyhow::Result<isize> {
	Ok(input
		.lines()
		.map(|s| s.parse::<isize>())
		.group_by(|s| s.is_ok())
		.into_iter()
		.map(|group| group.1.map(|el| el.unwrap_or(0)).sum::<isize>())
		.max()
		.unwrap())
}

pub fn pt2(input: &str) -> anyhow::Result<isize> {
	Ok(input
		.lines()
		.map(|s| s.parse::<isize>())
		.group_by(|s| s.is_ok())
		.into_iter()
		.map(|group| group.1.map(|el| el.unwrap_or(0)).sum::<isize>())
		.sorted()
		.rev()
		.take(3)
		.sum())
}

advent::problem!(
	r#"
		1000
		2000
		3000

		4000

		5000
		6000

		7000
		8000
		9000

		10000
    "#,
	24000,
	45000,
);
