use std::collections::HashSet;

use itertools::Itertools;

fn find_marker(input: &str, window_size: usize) -> usize {
	input
		.as_bytes()
		.windows(window_size)
		.map(|window| -> HashSet<_> { HashSet::from_iter(window) })
		.find_position(|set| set.len() == window_size)
		.map(|(position, _)| position + window_size)
		.unwrap()
}

pub fn pt1(input: &str) -> anyhow::Result<usize> {
	Ok(find_marker(input, 4))
}

pub fn pt2(input: &str) -> anyhow::Result<usize> {
	Ok(find_marker(input, 14))
}

advent::problem!(
	r#"
		mjqjpqmgbljsphdztnvjfqwrcgsmlb
    "#,
	7,
	19,
);
