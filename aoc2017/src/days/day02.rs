use itertools::{Itertools, MinMaxResult};

pub fn pt1(input: &str) -> anyhow::Result<isize> {
	Ok(input
		.lines()
		.map(|line| {
			if let MinMaxResult::MinMax(min, max) = line
				.split_whitespace()
				.map(|s| s.parse::<isize>().unwrap())
				.minmax()
			{
				max - min
			} else {
				unreachable!()
			}
		})
		.sum())
}

pub fn pt2(input: &str) -> anyhow::Result<isize> {
	Ok(input
		.lines()
		.map(|line| {
			let (a, b) = line
				.split_whitespace()
				.map(|s| s.parse::<isize>().unwrap())
				.tuple_combinations()
				.find(|(a, b)| a.max(b) % b.min(a) == 0)
				.unwrap();

			a.max(b) / b.min(a)
		})
		.sum())
}

advent::problem!(
	r#"
		5 9 2 8
		9 4 7 3
		3 8 6 5
    "#,
	18,
	9,
);
