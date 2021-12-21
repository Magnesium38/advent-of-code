use hashbrown::HashMap;
use itertools::iproduct;

pub fn pt1(input: &str) -> anyhow::Result<usize> {
	let mut game = parse_input(input);

	while let Some((player_one_score, player_two_score)) = game.next() {
		if player_one_score >= 1000 {
			return Ok(player_two_score * game.rolls);
		}

		if player_two_score >= 1000 {
			return Ok(player_one_score * game.rolls);
		}
	}

	Err(anyhow::anyhow!("No winner"))
}

pub fn pt2(input: &str) -> anyhow::Result<usize> {
	let game = parse_input(input);

	let (player_one_wins, player_two_wins) =
		dirac_game(&mut HashMap::new(), (0, 0), game.positions, true);

	Ok(player_one_wins.max(player_two_wins))
}

fn dirac_game(
	cache: &mut HashMap<((usize, usize), (usize, usize), bool), (usize, usize)>,
	(s1, s2): (usize, usize),
	(p1, p2): (usize, usize),
	player_one_turn: bool,
) -> (usize, usize) {
	if s1 >= 21 {
		return (1, 0);
	}
	if s2 >= 21 {
		return (0, 1);
	}
	if let Some(wins) = cache.get(&((s1, s2), (p1, p2), player_one_turn)) {
		return *wins;
	}

	let mut wins = (0, 0);
	for roll in iproduct!(1..=3, 1..=3, 1..=3).map(|(a, b, c)| a + b + c) {
		let (w1, w2) = if player_one_turn {
			let position = p1 + roll;
			let position = position - if position > 10 { 10 } else { 0 };

			dirac_game(cache, (s1 + position, s2), (position, p2), false)
		} else {
			let position = p2 + roll;
			let position = position - if position > 10 { 10 } else { 0 };

			dirac_game(cache, (s1, s2 + position), (p1, position), true)
		};

		wins.0 += w1;
		wins.1 += w2;
	}

	cache.insert(((s1, s2), (p1, p2), player_one_turn), wins);

	wins
}

fn parse_input(input: &str) -> GameState {
	let mut lines = input.lines();

	let player1 = lines
		.next()
		.unwrap()
		.chars()
		.nth(28)
		.unwrap()
		.to_digit(10)
		.unwrap() as usize;
	let player2 = lines
		.next()
		.unwrap()
		.chars()
		.nth(28)
		.unwrap()
		.to_digit(10)
		.unwrap() as usize;

	GameState::new(player1, player2)
}

#[derive(Clone, Copy)]
enum Turn {
	PlayerOne,
	PlayerTwo,
}

#[derive(Clone, Copy)]
struct GameState {
	die: usize,
	rolls: usize,
	scores: (usize, usize),
	positions: (usize, usize),
	turn: Turn,
}

impl GameState {
	fn new(player1: usize, player2: usize) -> Self {
		Self {
			die: 1,
			rolls: 0,
			scores: (0, 0),
			positions: (player1, player2),
			turn: Turn::PlayerOne,
		}
	}

	fn roll(&mut self) -> usize {
		let roll = self.die;

		self.rolls += 1;
		self.die = (self.die) % 100 + 1;

		roll
	}
}

impl Iterator for GameState {
	type Item = (usize, usize);

	fn next(&mut self) -> Option<Self::Item> {
		let roll = self.roll() + self.roll() + self.roll();
		match self.turn {
			Turn::PlayerOne => {
				self.positions.0 = (roll + self.positions.0 - 1) % 10 + 1;

				self.scores.0 += self.positions.0;

				self.turn = Turn::PlayerTwo;
			}
			Turn::PlayerTwo => {
				self.positions.1 = (roll + self.positions.1 - 1) % 10 + 1;

				self.scores.1 += self.positions.1;

				self.turn = Turn::PlayerOne;
			}
		}

		Some(self.scores)
	}
}

advent::problem!(
	r#"
		Player 1 starting position: 4
		Player 2 starting position: 8
	"#,
	739785,
	444356092776315,
);
