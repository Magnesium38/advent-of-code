use std::ops::Sub;

#[derive(PartialEq, PartialOrd, Copy, Clone)]
enum Rps {
	Rock,
	Paper,
	Scissors,
}

impl Sub for Rps {
	type Output = isize;

	fn sub(self, rhs: Self) -> Self::Output {
		if self == rhs {
			return 3;
		}

		if self == Rps::Rock {
			return if rhs == Rps::Paper { 6 } else { 0 };
		}
		if self == Rps::Paper {
			return if rhs == Rps::Scissors { 6 } else { 0 };
		}
		if self == Rps::Scissors {
			return if rhs == Rps::Rock { 6 } else { 0 };
		}

		unreachable!()
	}
}

pub fn pt1(input: &str) -> anyhow::Result<isize> {
	Ok(input
		.lines()
		.map(|s| {
			(
				match s.chars().next().unwrap() {
					'A' => Rps::Rock,
					'B' => Rps::Paper,
					'C' => Rps::Scissors,
					_ => unreachable!(),
				},
				match s.chars().skip(2).next().unwrap() {
					'X' => Rps::Rock,
					'Y' => Rps::Paper,
					'Z' => Rps::Scissors,
					_ => unreachable!(),
				},
			)
		})
		.map(|(a, b)| {
			(a - b)
				+ match b {
					Rps::Rock => 1,
					Rps::Paper => 2,
					Rps::Scissors => 3,
				}
		})
		.sum())
}

pub fn pt2(input: &str) -> anyhow::Result<isize> {
	Ok(input
		.lines()
		.map(|s| {
			let opponent = match s.chars().next().unwrap() {
				'A' => Rps::Rock,
				'B' => Rps::Paper,
				'C' => Rps::Scissors,
				_ => unreachable!(),
			};

			(
				opponent,
				match s.chars().skip(2).next().unwrap() {
					'X' => match opponent {
						Rps::Rock => Rps::Scissors,
						Rps::Paper => Rps::Rock,
						Rps::Scissors => Rps::Paper,
					},
					'Y' => match opponent {
						Rps::Rock => Rps::Rock,
						Rps::Paper => Rps::Paper,
						Rps::Scissors => Rps::Scissors,
					},
					'Z' => match opponent {
						Rps::Rock => Rps::Paper,
						Rps::Paper => Rps::Scissors,
						Rps::Scissors => Rps::Rock,
					},
					_ => unreachable!(),
				},
			)
		})
		.map(|(a, b)| {
			(a - b)
				+ match b {
					Rps::Rock => 1,
					Rps::Paper => 2,
					Rps::Scissors => 3,
				}
		})
		.sum())
}

advent::problem!(
	r#"
		A Y
		B X
		C Z
    "#,
	15,
	12,
);
