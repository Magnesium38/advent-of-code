use std::collections::HashSet;

use itertools::Itertools;

pub fn pt1(input: &str) -> anyhow::Result<usize> {
	Ok(input
		.chars()
		.enumerate()
		.tuple_windows()
		.map(|((i, a), (_, b), (_, c), (_, d))| -> (_, HashSet<_>) {
			(i + 4, HashSet::from_iter([a, b, c, d]))
		})
		.find(|(_, set)| set.len() == 4)
		.unwrap()
		.0)
}

pub fn pt2(input: &str) -> anyhow::Result<usize> {
	Ok(input
		.as_bytes()
		.windows(14)
		.map(|window| -> HashSet<_> { HashSet::from_iter(window) })
		.enumerate()
		.find(|(_, set)| set.len() == 14)
		.unwrap()
		.0 + 14)
}

advent::problem!(
	r#"
		mjqjpqmgbljsphdztnvjfqwrcgsmlb
    "#,
	7,
	19,
);
