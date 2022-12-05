use itertools::Itertools;

struct Shipyard {
	stacks: Vec<Vec<char>>,
}

impl Shipyard {
	fn new(input: &str) -> Self {
		let mut stacks: Vec<_> = (0..input.lines().next().unwrap().len() / 4 + 1)
			.map(|_| vec![])
			.collect();

		input.lines().for_each(|line| {
			line.chars()
				.enumerate()
				.filter(|(_, c)| c.is_alphabetic())
				.for_each(|(i, c)| stacks[i / 4].push(c))
		});

		stacks.iter_mut().for_each(|stack| stack.reverse());

		Shipyard { stacks }
	}

	fn move_crate(&mut self, (amount, from, to): (usize, usize, usize)) {
		for _ in 0..amount {
			if let Some(c) = self.stacks[from - 1].pop() {
				self.stacks[to - 1].push(c);
			} else {
				break;
			}
		}
	}

	fn bulk_move_crates(&mut self, (amount, from, to): (usize, usize, usize)) {
		let mut tmp = Vec::with_capacity(amount);

		for _ in 0..amount {
			if let Some(c) = self.stacks[from - 1].pop() {
				tmp.push(c);
			} else {
				break;
			}
		}

		tmp.reverse();

		self.stacks[to - 1].extend(tmp);
	}

	fn read_message(self) -> String {
		self.stacks.iter().map(|v| v.last().unwrap()).collect()
	}
}

pub fn pt1(input: &str) -> anyhow::Result<String> {
	let (initial, moves) = input.split_once("\n\n").unwrap();
	let mut shipyard = Shipyard::new(initial);

	moves
		.lines()
		.map(|line| line.split_ascii_whitespace().collect_tuple().unwrap())
		.map(|(_, amount, _, from, _, to)| {
			(
				amount.parse().unwrap(),
				from.parse().unwrap(),
				to.parse().unwrap(),
			)
		})
		.for_each(|input| shipyard.move_crate(input));

	Ok(shipyard.read_message())
}

pub fn pt2(input: &str) -> anyhow::Result<String> {
	let (initial, moves) = input.split_once("\n\n").unwrap();
	let mut shipyard = Shipyard::new(initial);

	moves
		.lines()
		.map(|line| line.split_ascii_whitespace().collect_tuple().unwrap())
		.map(|(_, amount, _, from, _, to)| {
			(
				amount.parse().unwrap(),
				from.parse().unwrap(),
				to.parse().unwrap(),
			)
		})
		.for_each(|input| shipyard.bulk_move_crates(input));

	Ok(shipyard.read_message())
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = r#"
		    [D]    
		[N] [C]    
		[Z] [M] [P]
		 1   2   3 

		move 1 from 2 to 1
		move 3 from 1 to 3
		move 2 from 2 to 1
		move 1 from 1 to 2
	"#;

	fn prepare_input<'a>(input: &'a str) -> String {
		input
			.trim_matches(|c: char| c.is_whitespace() && c != ' ')
			.lines()
			.map(|line| line.trim_matches(|c: char| c.is_whitespace() && c != ' '))
			.collect::<Vec<_>>()
			.join("\n")
	}

	#[test]
	fn test_pt1() {
		assert_eq!(pt1(&prepare_input(INPUT)).unwrap(), String::from("CMZ"));
	}

	#[test]
	fn test_pt2() {
		assert_eq!(pt2(&prepare_input(INPUT)).unwrap(), String::from("MCD"));
	}
}
