use itertools::Itertools;
use std::{collections::HashMap, str::FromStr};

pub fn pt1(input: &str) -> anyhow::Result<isize> {
	Ok(from_lines(
		input
			.lines()
			.map(|line| line.parse().unwrap())
			.filter(|line: &Line| line.is_horizontal() || line.is_vertical()),
	)
	.iter()
	.fold(
		0,
		|total, (_, &value)| if value > 1 { total + 1 } else { total },
	))
}

pub fn pt2(input: &str) -> anyhow::Result<isize> {
	Ok(from_lines(input.lines().map(|line| line.parse().unwrap()))
		.iter()
		.fold(
			0,
			|total, (_, &value)| if value > 1 { total + 1 } else { total },
		))
}

struct Line((isize, isize), (isize, isize));

impl Line {
	const fn is_horizontal(&self) -> bool {
		self.0 .0 == self.1 .0
	}

	const fn is_vertical(&self) -> bool {
		self.0 .1 == self.1 .1
	}
}

impl FromStr for Line {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> anyhow::Result<Self> {
		fn to_point(s: Option<&str>) -> anyhow::Result<(isize, isize)> {
			s.ok_or(anyhow::anyhow!("invalid input"))?
				.split(',')
				.map(|s| s.parse().unwrap())
				.collect_tuple()
				.ok_or(anyhow::anyhow!("invalid point"))
		}
		
		let mut iter = s.split(" -> ");

		Ok(Self(to_point(iter.next())?, to_point(iter.next())?))
	}
}

impl IntoIterator for Line {
	type Item = (isize, isize);
	type IntoIter = std::vec::IntoIter<(isize, isize)>;

	fn into_iter(self) -> Self::IntoIter {
		let mut iter = Vec::new();

		let (x1, y1) = self.0;
		let (x2, y2) = self.1;
		let (dx, dy) = (x2 - x1, y2 - y1);

		for i in 0..=dx.abs().max(dy.abs()) {
			iter.push((x1 + i * dx.signum(), y1 + i * dy.signum()));
		}

		iter.into_iter()
	}
}

fn from_lines<T: Iterator<Item = Line>>(lines: T) -> HashMap<(isize, isize), isize> {
	let mut data = HashMap::new();

	for line in lines {
		for point in line {
			let value = data.entry(point).or_insert(0);
			*value += 1;
		}
	}

	data
}

advent::problem!(
	r#"
		0,9 -> 5,9
		8,0 -> 0,8
		9,4 -> 3,4
		2,2 -> 2,1
		7,0 -> 7,4
		6,4 -> 2,0
		0,9 -> 2,9
		3,4 -> 1,4
		0,0 -> 8,8
		5,5 -> 8,2
    "#,
	5,
	12,
);
