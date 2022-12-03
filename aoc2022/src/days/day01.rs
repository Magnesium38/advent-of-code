use itertools::Itertools;

fn groups(input: &str) -> impl Iterator<Item = isize> + '_ {
	input
		.split("\n\n")
		.map(|group| group.lines().map(|s| s.parse().unwrap_or(0)).sum())
}

pub fn pt1(input: &str) -> anyhow::Result<isize> {
	Ok(groups(input).max().expect("at least one element was expected"))
}

pub fn pt2(input: &str) -> anyhow::Result<isize> {
	Ok(groups(input)
		.sorted_unstable_by(|a, b| b.cmp(a))
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
