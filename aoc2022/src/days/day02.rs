use itertools::Itertools;

#[derive(Clone, Copy)]
enum Rps {
	Rock,
	Paper,
	Scissors,
}

fn score((opponent, player): (Rps, Rps)) -> isize {
	(match player {
		Rps::Rock => 1,
		Rps::Paper => 2,
		Rps::Scissors => 3,
	}) + (match (opponent, player) {
		(Rps::Rock, Rps::Rock) => 3,
		(Rps::Rock, Rps::Scissors) => 0,
		(Rps::Rock, Rps::Paper) => 6,
		(Rps::Paper, Rps::Rock) => 0,
		(Rps::Paper, Rps::Scissors) => 6,
		(Rps::Paper, Rps::Paper) => 3,
		(Rps::Scissors, Rps::Rock) => 6,
		(Rps::Scissors, Rps::Scissors) => 3,
		(Rps::Scissors, Rps::Paper) => 0,
	})
}

pub fn pt1(input: &str) -> anyhow::Result<isize> {
	Ok(input
		.lines()
		.map(|s| match s.split(' ').collect_tuple().unwrap() {
			("A", "X") => (Rps::Rock, Rps::Rock),
			("A", "Y") => (Rps::Rock, Rps::Paper),
			("A", "Z") => (Rps::Rock, Rps::Scissors),
			("B", "X") => (Rps::Paper, Rps::Rock),
			("B", "Y") => (Rps::Paper, Rps::Paper),
			("B", "Z") => (Rps::Paper, Rps::Scissors),
			("C", "X") => (Rps::Scissors, Rps::Rock),
			("C", "Y") => (Rps::Scissors, Rps::Paper),
			("C", "Z") => (Rps::Scissors, Rps::Scissors),
			_ => unreachable!(),
		})
		.map(score)
		.sum())
}

pub fn pt2(input: &str) -> anyhow::Result<isize> {
	Ok(input
		.lines()
		.map(|s| match s.split(' ').collect_tuple().unwrap() {
			("A", "X") => (Rps::Rock, Rps::Scissors),
			("A", "Y") => (Rps::Rock, Rps::Rock),
			("A", "Z") => (Rps::Rock, Rps::Paper),
			("B", "X") => (Rps::Paper, Rps::Rock),
			("B", "Y") => (Rps::Paper, Rps::Paper),
			("B", "Z") => (Rps::Paper, Rps::Scissors),
			("C", "X") => (Rps::Scissors, Rps::Paper),
			("C", "Y") => (Rps::Scissors, Rps::Scissors),
			("C", "Z") => (Rps::Scissors, Rps::Rock),
			_ => unreachable!(),
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
