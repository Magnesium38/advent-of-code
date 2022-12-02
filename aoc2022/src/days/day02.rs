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
		.map(|s| match (s.as_bytes()[0], s.as_bytes()[2]) {
			(b'A', b'X') => (Rps::Rock, Rps::Rock),
			(b'A', b'Y') => (Rps::Rock, Rps::Paper),
			(b'A', b'Z') => (Rps::Rock, Rps::Scissors),
			(b'B', b'X') => (Rps::Paper, Rps::Rock),
			(b'B', b'Y') => (Rps::Paper, Rps::Paper),
			(b'B', b'Z') => (Rps::Paper, Rps::Scissors),
			(b'C', b'X') => (Rps::Scissors, Rps::Rock),
			(b'C', b'Y') => (Rps::Scissors, Rps::Paper),
			(b'C', b'Z') => (Rps::Scissors, Rps::Scissors),
			_ => unreachable!(),
		})
		.map(score)
		.sum())
}

pub fn pt2(input: &str) -> anyhow::Result<isize> {
	Ok(input
		.lines()
		.map(|s| match (s.as_bytes()[0], s.as_bytes()[2]) {
			(b'A', b'X') => (Rps::Rock, Rps::Scissors),
			(b'A', b'Y') => (Rps::Rock, Rps::Rock),
			(b'A', b'Z') => (Rps::Rock, Rps::Paper),
			(b'B', b'X') => (Rps::Paper, Rps::Rock),
			(b'B', b'Y') => (Rps::Paper, Rps::Paper),
			(b'B', b'Z') => (Rps::Paper, Rps::Scissors),
			(b'C', b'X') => (Rps::Scissors, Rps::Paper),
			(b'C', b'Y') => (Rps::Scissors, Rps::Scissors),
			(b'C', b'Z') => (Rps::Scissors, Rps::Rock),
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
