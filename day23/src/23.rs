use hashbrown::HashSet;
use itertools::Itertools;
use std::{
	cmp::{Ordering, Reverse},
	collections::BinaryHeap,
	fmt::Display,
};

pub fn pt1(input: &str) -> anyhow::Result<usize> {
	let mut state = State::new(2);
	let mut visited = HashSet::new();
	let mut queue = BinaryHeap::new();

	input
		.lines()
		.skip(2)
		.flat_map(|line| line.chars())
		.filter(|&c| c == 'A' || c == 'B' || c == 'C' || c == 'D')
		.map(|c| match c {
			'A' => Occupant::A,
			'B' => Occupant::B,
			'C' => Occupant::C,
			'D' => Occupant::D,
			_ => unreachable!(),
		})
		.enumerate()
		.for_each(|(i, c)| {
			state.rooms[i % 4][if i >= 4 { 1 } else { 0 }] = c;
		});

	queue.push(state);

	while let Some(state) = queue.pop() {
		if !visited.insert(state.id()) {
			continue;
		}

		if state.is_solution() {
			return Ok(state.cost.0);
		}

		for new_state in state.moves() {
			queue.push(new_state);
		}
	}

	Err(anyhow::anyhow!("No solution found"))
}

pub fn pt2(input: &str) -> anyhow::Result<usize> {
	let mut state = State::new(4);
	let mut visited = HashSet::new();
	let mut queue = BinaryHeap::new();

	input
		.lines()
		.skip(2)
		.flat_map(|line| line.chars())
		.filter(|&c| c == 'A' || c == 'B' || c == 'C' || c == 'D')
		.map(|c| match c {
			'A' => Occupant::A,
			'B' => Occupant::B,
			'C' => Occupant::C,
			'D' => Occupant::D,
			_ => unreachable!(),
		})
		.enumerate()
		.for_each(|(i, c)| {
			state.rooms[i % 4][if i >= 4 { 3 } else { 0 }] = c;
		});

	state.rooms[0][1] = Occupant::D;
	state.rooms[1][1] = Occupant::C;
	state.rooms[2][1] = Occupant::B;
	state.rooms[3][1] = Occupant::A;
	state.rooms[0][2] = Occupant::D;
	state.rooms[1][2] = Occupant::B;
	state.rooms[2][2] = Occupant::A;
	state.rooms[3][2] = Occupant::C;

	queue.push(state);

	while let Some(state) = queue.pop() {
		if !visited.insert(state.id()) {
			continue;
		}

		if state.is_solution() {
			return Ok(state.cost.0);
		}

		for new_state in state.moves() {
			queue.push(new_state);
		}
	}

	Err(anyhow::anyhow!("No solution found"))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Occupant {
	A,
	B,
	C,
	D,
	Empty,
}

impl Display for Occupant {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Occupant::A => write!(f, "A"),
			Occupant::B => write!(f, "B"),
			Occupant::C => write!(f, "C"),
			Occupant::D => write!(f, "D"),
			Occupant::Empty => write!(f, "."),
		}
	}
}

#[derive(Clone, Eq, PartialEq)]
struct State {
	cost: Reverse<usize>,
	hallway: [Occupant; 11],
	rooms: [Vec<Occupant>; 4],
}

impl std::fmt::Debug for State {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "\n{}", self.hallway.iter().join(""))?;

		for i in 0..2 {
			write!(f, "\n  ")?;
			for j in 0..4 {
				write!(f, "{} ", self.rooms[j][i])?;
			}
		}

		write!(f, "\ncost: {}", self.cost.0)
	}
}

impl PartialOrd for State {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for State {
	fn cmp(&self, other: &Self) -> Ordering {
		self.cost.cmp(&other.cost)
	}
}

impl State {
	fn new(n: usize) -> Self {
		Self {
			cost: Reverse(0),
			hallway: [
				Occupant::Empty,
				Occupant::Empty,
				Occupant::Empty,
				Occupant::Empty,
				Occupant::Empty,
				Occupant::Empty,
				Occupant::Empty,
				Occupant::Empty,
				Occupant::Empty,
				Occupant::Empty,
				Occupant::Empty,
			],
			rooms: [
				vec![Occupant::Empty; n],
				vec![Occupant::Empty; n],
				vec![Occupant::Empty; n],
				vec![Occupant::Empty; n],
			],
		}
	}

	fn is_solution(&self) -> bool {
		self.rooms[0].iter().all(|&c| c == Occupant::A)
			&& self.rooms[1].iter().all(|&c| c == Occupant::B)
			&& self.rooms[2].iter().all(|&c| c == Occupant::C)
			&& self.rooms[3].iter().all(|&c| c == Occupant::D)
	}

	fn id(&self) -> Vec<Occupant> {
		self.hallway
			.iter()
			.chain(self.rooms.iter().flat_map(|r| r.iter()))
			.copied()
			.collect()
	}

	fn moves(&self) -> Vec<Self> {
		let mut moves = Vec::new();

		for (i, o) in self
			.hallway
			.iter()
			.enumerate()
			.filter(|(_, &c)| c != Occupant::Empty)
		{
			let room_index = match o {
				Occupant::A => 0,
				Occupant::B => 1,
				Occupant::C => 2,
				Occupant::D => 3,
				_ => unreachable!(),
			};

			if let Some(room_position) = self.rooms[room_index]
				.iter()
				.positions(|&c| c == Occupant::Empty)
				.last()
			{
				let ri = 2 + room_index * 2;
				let (mut i1, mut i2) = (ri.min(i), ri.max(i));
				if i1 == i {
					i1 += 1;
				} else {
					i2 -= 1;
				}
				let can_move = self.hallway[i1..=i2].iter().all(|&c| c == Occupant::Empty);

				if can_move {
					let move_cost = i2 - i1 + 2 + room_position;

					let mut new_state = self.clone();
					new_state.cost = Reverse(
						self.cost.0
							+ match o {
								Occupant::A => move_cost,
								Occupant::B => move_cost * 10,
								Occupant::C => move_cost * 100,
								Occupant::D => move_cost * 1000,
								_ => unreachable!(),
							},
					);

					new_state.hallway[i] = Occupant::Empty;
					new_state.rooms[room_index][room_position] = *o;

					moves.push(new_state);
				}
			}
		}

		for (room_index, (room_position, o)) in self
			.rooms
			.iter()
			.enumerate()
			.filter(|(room_index, room)| !match room_index {
				0 => room
					.iter()
					.all(|&c| c == Occupant::A || c == Occupant::Empty),
				1 => room
					.iter()
					.all(|&c| c == Occupant::B || c == Occupant::Empty),
				2 => room
					.iter()
					.all(|&c| c == Occupant::C || c == Occupant::Empty),
				3 => room
					.iter()
					.all(|&c| c == Occupant::D || c == Occupant::Empty),
				_ => unreachable!(),
			})
			.map(|(room_index, room)| {
				(
					room_index,
					room.iter().find_position(|&c| c != &Occupant::Empty),
				)
			})
			.filter(|(_, o)| o.is_some())
			.map(|(room_index, o)| (room_index, o.unwrap()))
		{
			for index in [0, 1, 3, 5, 7, 9, 10] {
				let ri = 2 + room_index * 2;
				let (i1, i2) = (ri.min(index), ri.max(index));
				let can_move = self.hallway[i1..=i2].iter().all(|&c| c == Occupant::Empty);

				if can_move {
					let move_cost = i2 - i1 + room_position + 1;

					let mut new_state = self.clone();
					new_state.cost = Reverse(
						self.cost.0
							+ match o {
								Occupant::A => move_cost,
								Occupant::B => move_cost * 10,
								Occupant::C => move_cost * 100,
								Occupant::D => move_cost * 1000,
								_ => unreachable!(),
							},
					);
					new_state.hallway[index] = *o;
					new_state.rooms[room_index][room_position] = Occupant::Empty;

					moves.push(new_state);
				}
			}
		}

		moves
	}
}

advent::problem!(
	r#"
		#############
		#...........#
		###B#C#B#D###
		  #A#D#C#A#
		  #########
	"#,
	12521,
	44169,
);
