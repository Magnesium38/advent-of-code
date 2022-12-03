use itertools::Itertools;

fn encode(s: &str) -> u64 {
	s.as_bytes().iter().fold(0u64, |n, c| {
		1 << match c {
			b'a'..=b'z' => (c - b'a') + 1,
			b'A'..=b'Z' => (c - b'A') + 27,
			_ => unreachable!("invalid input character"),
		} | n
	})
}

fn find_common<'a, I: Iterator<Item = &'a str>>(iter: I) -> u32 {
	iter.map(encode)
		.reduce(|acc, el| acc & el)
		.expect("expected at least one element")
		.trailing_zeros()
}

pub fn pt1(input: &str) -> anyhow::Result<u32> {
	Ok(input
		.lines()
		.map(|line| {
			let (a, b) = line.split_at(line.len() / 2);

			find_common([a, b].iter().copied())
		})
		.sum())
}

pub fn pt2(input: &str) -> anyhow::Result<u32> {
	Ok(input.lines().chunks(3).into_iter().map(find_common).sum())
}

advent::problem!(
	r#"
		vJrwpWtwJgWrhcsFMMfFFhFp
		jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
		PmmdzqPrVvPwwTWBwg
		wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
		ttgJtRGJQctTZtZT
		CrZsJsPPZsGzwwsLwLmpwMDw
    "#,
	157,
	70,
);
