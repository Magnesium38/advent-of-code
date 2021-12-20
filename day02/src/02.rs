use std::{num::ParseIntError, str::FromStr};

pub fn pt1(input: &str) -> anyhow::Result<isize> {
	let (x, y) = input
		.lines()
		.map(|line| line.parse().unwrap())
		.fold((0, 0), |(x, y), operation| match operation {
			Command::Forward(n) => (x + n, y),
			Command::Down(n) => (x, y + n),
		});

	Ok(x * y)
}

pub fn pt2(input: &str) -> anyhow::Result<isize> {
	let (x, y, _) =
		input
			.lines()
			.map(|line| line.parse().unwrap())
			.fold((0, 0, 0), |(x, y, aim), operation| match operation {
				Command::Forward(n) => (x + n, y + (aim * n), aim),
				Command::Down(n) => (x, y, aim + n),
			});

	Ok(x * y)
}

enum Command {
	Forward(isize),
	Down(isize),
}

impl FromStr for Command {
	type Err = ParseIntError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (command, distance) = {
			let mut iter = s.split_whitespace();
			(iter.next().unwrap(), iter.next().unwrap().parse()?)
		};

		Ok(match command {
			"forward" => Self::Forward(distance),
			"down" => Self::Down(distance),
			"up" => Self::Down(-distance),
			_ => panic!("Unknown direction: {}", command),
		})
	}
}

advent::problem!(
	r#"
        forward 5
        down 5
        forward 8
        up 3
        down 8
        forward 2
    "#,
	150,
	900,
);
