use std::ops::Sub;

#[derive(PartialEq, Copy, Clone)]
enum Rps {
	Rock,
	Paper,
	Scissors,
}

impl Sub for Rps {
	type Output = isize;

	fn sub(self, rhs: Self) -> Self::Output {
		match (self, rhs) {
			(Rps::Rock, Rps::Rock) => 3,
			(Rps::Rock, Rps::Scissors) => 0,
			(Rps::Rock, Rps::Paper) => 6,
			(Rps::Paper, Rps::Rock) => 0,
			(Rps::Paper, Rps::Scissors) => 6,
			(Rps::Paper, Rps::Paper) => 3,
			(Rps::Scissors, Rps::Rock) => 6,
			(Rps::Scissors, Rps::Scissors) => 3,
			(Rps::Scissors, Rps::Paper) => 0,
		}
	}
}

fn score((opponent, player): (Rps, Rps)) -> isize {
	(opponent - player)
		+ match player {
			Rps::Rock => 1,
			Rps::Paper => 2,
			Rps::Scissors => 3,
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
				match s.chars().nth(2).unwrap() {
					'X' => Rps::Rock,
					'Y' => Rps::Paper,
					'Z' => Rps::Scissors,
					_ => unreachable!(),
				},
			)
		})
		.map(score)
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
				match s.chars().nth(2).unwrap() {
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
		.map(score)
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
