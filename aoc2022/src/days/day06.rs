use std::collections::HashSet;

pub fn pt1(input: &str) -> anyhow::Result<usize> {
	Ok(input
		.as_bytes()
		.windows(4)
		.map(|window| -> HashSet<_> { HashSet::from_iter(window) })
		.enumerate()
		.find(|(_, set)| set.len() == 4)
		.unwrap()
		.0 + 4)
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
