use itertools::Itertools;

fn temp(s: &str) -> u64 {
	let mut n = 0;

	for c in s.as_bytes() {
		n |= 1
			<< match c {
				b'a'..=b'z' => (c - b'a') + 1,
				b'A'..=b'Z' => (c - b'A') + 27,
				_ => unreachable!(),
			};
	}

	n
}

pub fn pt1(input: &str) -> anyhow::Result<u32> {
	Ok(input
		.lines()
		.map(|line| {
			let (a, b) = line.split_at(line.len() / 2);

			(temp(a) & temp(b)).trailing_zeros()
		})
		.sum())
}

pub fn pt2(input: &str) -> anyhow::Result<u32> {
	Ok(input
		.lines()
		.chunks(3)
		.into_iter()
		.map(|chunk| {
			let (a, b, c) = chunk.collect_tuple().unwrap();

			(temp(a) & temp(b) & temp(c)).trailing_zeros()
		})
		.sum())
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
