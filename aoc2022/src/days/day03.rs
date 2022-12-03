use itertools::Itertools;
use std::collections::HashSet;

pub fn pt1(input: &str) -> anyhow::Result<isize> {
	Ok(input
		.lines()
		.map(|line| {
			let (a, b) = line.split_at(line.len() / 2);

			let (a, b) = (
				a.as_bytes().iter().collect::<HashSet<_>>(),
				b.as_bytes().iter().collect::<HashSet<_>>(),
			);

			let c = a.intersection(&b).next().unwrap();
			match c {
				b'a'..=b'z' => isize::from((*c - b'a') + 1),
				b'A'..=b'Z' => isize::from((*c - b'A') + 27),
				_ => unreachable!(),
			}
		})
		.sum())
}

pub fn pt2(input: &str) -> anyhow::Result<isize> {
	Ok(input
		.lines()
		.chunks(3)
		.into_iter()
		.map(|chunk| {
			let (a, b, c) = chunk.collect_tuple().unwrap();

			let (a, b, c) = (
				a.as_bytes().iter().cloned().collect::<HashSet<u8>>(),
				b.as_bytes().iter().cloned().collect::<HashSet<u8>>(),
				c.as_bytes().iter().cloned().collect::<HashSet<u8>>(),
			);

			let ab = a.intersection(&b).cloned().collect::<HashSet<u8>>();
			let abc = ab.intersection(&c).next().unwrap();

			match abc {
				b'a'..=b'z' => isize::from((*abc - b'a') + 1),
				b'A'..=b'Z' => isize::from((*abc - b'A') + 27),
				_ => unreachable!(),
			}
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
