use itertools::Itertools;
use retain_mut::RetainMut;

fn pt1(input: &str) -> anyhow::Result<isize> {
	for state in Bingo::from(input.lines().filter(|line| !line.is_empty()))? {
		if let State::Win(n) = state {
			return Ok(n);
		}
	}

	Err(anyhow::anyhow!("ran out of numbers"))
}

fn pt2(input: &str) -> anyhow::Result<isize> {
	let mut last_win_score = None;

	for state in Bingo::from(input.lines().filter(|line| !line.is_empty()))? {
		if let State::Win(n) = state {
			last_win_score = Some(n);
		}
	}

	last_win_score.ok_or_else(|| anyhow::anyhow!("ran out of numbers"))
}

enum Cell {
	Called,
	Uncalled(isize),
}

struct Board {
	chunk: Vec<Vec<Cell>>,
}

impl Board {
	fn call(&mut self, n: isize) -> State {
		for row in &mut self.chunk {
			for cell in row.iter_mut() {
				if let Cell::Uncalled(m) = cell {
					if *m == n {
						*cell = Cell::Called;
					}
				}
			}
		}

		self.check_win(n)
	}

	fn calculate_score(&self) -> isize {
		let mut score = 0;

		for row in &self.chunk {
			for cell in row {
				if let Cell::Uncalled(n) = cell {
					score += n;
				}
			}
		}

		score
	}

	fn check_win(&self, last_number: isize) -> State {
		for row in &self.chunk {
			if row.iter().all(|c| match c {
				Cell::Called => true,
				Cell::Uncalled(_) => false,
			}) {
				return State::Win(self.calculate_score() * last_number);
			}
		}

		for col in 0..5 {
			let mut called_count = 0;
			for row in 0..5 {
				if let Cell::Called = self.chunk[row][col] {
					called_count += 1;
				}
			}

			if called_count == 5 {
				return State::Win(self.calculate_score() * last_number);
			}
		}

		State::Continue
	}
}

struct Bingo {
	numbers: Vec<isize>,
	boards: Vec<Board>,
}

impl Bingo {
	fn from<'a, I>(mut input: I) -> anyhow::Result<Self>
	where
		I: Iterator<Item = &'a str>,
	{
		let mut numbers = input
			.next()
			.ok_or_else(|| anyhow::anyhow!("No numbers"))?
			.split(',')
			.map(|s| s.parse::<isize>().unwrap())
			.collect::<Vec<_>>();
		numbers.reverse();

		let mut boards = Vec::new();

		for chunk in &input.chunks(5) {
			boards.push(Board {
				chunk: chunk
					.map(|line| {
						line.split(' ')
							.filter(|s| !s.is_empty())
							.map(|word| Cell::Uncalled(word.parse().unwrap()))
							.collect::<Vec<_>>()
					})
					.collect::<Vec<_>>(),
			});
		}

		Ok(Self { numbers, boards })
	}
}

enum State {
	Continue,
	Win(isize),
}

impl Iterator for Bingo {
	type Item = State;

	fn next(&mut self) -> Option<Self::Item> {
		let number = self.numbers.pop()?;

		let mut win = None;
		self.boards.retain_mut(|board| {
			let state = board.call(number);
			if let State::Win(n) = state {
				win = Some(State::Win(n));
			}

			matches!(state, State::Continue)
		});

		match win {
			None => Some(State::Continue),
			s => s,
		}
	}
}

advent::problem!(
	r#"
		7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

		22 13 17 11  0
		8  2 23  4 24
		21  9 14 16  7
		6 10  3 18  5
		1 12 20 15 19
		
		3 15  0  2 22
		9 18 13 17  5
		19  8  7 25 23
		20 11 10 24  4
		14 21 16 12  6
		
		14 21 17 24  4
		10 16 15  9 19
		18  8 23 26 20
		22 11 13  6  5
		2  0 12  3  7
	"#,
	4512,
	1924,
);
