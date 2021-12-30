use hashbrown::HashSet;
use itertools::Itertools;
use std::{
	cmp::{Ordering, Reverse},
	collections::BinaryHeap,
	fmt::Display,
	hash::Hash,
};

pub fn pt1(input: &str) -> anyhow::Result<usize> {
	solve(parse::<2>(input))
}

pub fn pt2(input: &str) -> anyhow::Result<usize> {
	let mut initial_state = parse::<4>(input);

	initial_state.rooms.iter_mut().for_each(|room| {
		room.cells[3] = room.cells[1];
	});

	initial_state.rooms[0].cells[1] = Some(Amphipod::Desert);
	initial_state.rooms[1].cells[1] = Some(Amphipod::Copper);
	initial_state.rooms[2].cells[1] = Some(Amphipod::Bronze);
	initial_state.rooms[3].cells[1] = Some(Amphipod::Amber);
	initial_state.rooms[0].cells[2] = Some(Amphipod::Desert);
	initial_state.rooms[1].cells[2] = Some(Amphipod::Bronze);
	initial_state.rooms[2].cells[2] = Some(Amphipod::Amber);
	initial_state.rooms[3].cells[2] = Some(Amphipod::Copper);

	solve(initial_state)
}

fn parse<const N: usize>(input: &str) -> State<N> {
	let input = input
		.lines()
		.skip(2)
		.map(|line| {
			line.chars()
				.filter(|c| matches!(c, 'A'..='D'))
				.map(Amphipod::parse)
				.collect_vec()
		})
		.collect_vec();

	let mut rooms = [Room {
		id: Amphipod::Amber,
		cells: [None; N],
	}; 4];

	for (i, room) in rooms.iter_mut().enumerate() {
		room.id = match i {
			0 => Amphipod::Amber,
			1 => Amphipod::Bronze,
			2 => Amphipod::Copper,
			3 => Amphipod::Desert,
			_ => unreachable!(),
		};

		for (j, v) in input.iter().enumerate().take(2) {
			room.cells[j] = Some(v[i]);
		}
	}

	State {
		cost: 0,
		rooms,
		hallway: [None; 11],
	}
}

fn solve<const N: usize>(initial_state: State<N>) -> anyhow::Result<usize> {
	let mut visited = HashSet::new();
	let mut queue = BinaryHeap::new();

	visited.insert(initial_state);
	queue.push(Reverse(initial_state));

	while let Some(Reverse(state)) = queue.pop() {
		if state.is_solved() {
			return Ok(state.cost);
		}

		for next_state in state.next_moves() {
			if visited.insert(next_state) {
				queue.push(Reverse(next_state));
			}
		}
	}

	Err(anyhow::anyhow!("No solution found"))
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum Amphipod {
	Amber,
	Bronze,
	Copper,
	Desert,
}

impl Amphipod {
	fn cost(&self) -> usize {
		match self {
			Amphipod::Amber => 1,
			Amphipod::Bronze => 10,
			Amphipod::Copper => 100,
			Amphipod::Desert => 1000,
		}
	}

	fn destination(&self) -> usize {
		match self {
			Amphipod::Amber => 0,
			Amphipod::Bronze => 1,
			Amphipod::Copper => 2,
			Amphipod::Desert => 3,
		}
	}

	fn parse(c: char) -> Amphipod {
		match c {
			'A' => Amphipod::Amber,
			'B' => Amphipod::Bronze,
			'C' => Amphipod::Copper,
			'D' => Amphipod::Desert,
			_ => panic!("unexpected character: {}", c),
		}
	}
}

impl Display for Amphipod {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Amphipod::Amber => write!(f, "A"),
			Amphipod::Bronze => write!(f, "B"),
			Amphipod::Copper => write!(f, "C"),
			Amphipod::Desert => write!(f, "D"),
		}
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct State<const N: usize> {
	cost: usize,
	hallway: [Option<Amphipod>; 11],
	rooms: [Room<N>; 4],
}

impl<const N: usize> PartialOrd for State<N> {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl<const N: usize> Ord for State<N> {
	fn cmp(&self, other: &Self) -> Ordering {
		self.cost.cmp(&other.cost)
	}
}

impl<const N: usize> State<N> {
	fn is_solved(&self) -> bool {
		self.rooms.iter().all(|room| room.is_solved())
	}

	fn cost_to_move_to_position(
		&self,
		from: usize,
		to: usize,
		amphipod: Amphipod,
	) -> Option<usize> {
		if from == to {
			return None;
		}

		let (i1, i2) = if from < to {
			(from + 1, to)
		} else {
			(to, from - 1)
		};

		if self.hallway[i1..=i2].iter().all(|cell| cell.is_none()) {
			Some((i2 - i1 + 1) * amphipod.cost())
		} else {
			None
		}
	}

	fn cost_to_move(&self, from: usize, amphipod: Amphipod) -> Option<usize> {
		let to = 2 * amphipod.destination() + 2;

		self.cost_to_move_to_position(from, to, amphipod)
	}

	fn next_moves(&self) -> Vec<Self> {
		let mut moves = vec![];

		// Attempt to move hallway Amphipods into rooms
		for hallway_index in 0..self.hallway.len() {
			if let Some(amphipod) = self.hallway[hallway_index] {
				if self.rooms[amphipod.destination()].accepts(amphipod) {
					if let Some(hallway_cost) = self.cost_to_move(hallway_index, amphipod) {
						let mut next_state = *self;
						let room_cost = next_state.rooms[amphipod.destination()].insert(amphipod);
						next_state.hallway[hallway_index] = None;
						next_state.cost += hallway_cost + room_cost;
						moves.push(next_state);
					}
				}
			}
		}

		// Attempt to move room Amphipods
		for room_id in 0..4 {
			if let Some((amphipod, room_position, exit_cost)) = self.rooms[room_id].get_next() {
				let room_index = 2 * room_id + 2;

				// Try see if they can move directly to their destination
				// but fall back on just into the hallway instead.
				if self.rooms[amphipod.destination()].accepts(amphipod) {
					if let Some(hallway_cost) = self.cost_to_move(room_index, amphipod) {
						let mut next_state = *self;
						let room_cost = next_state.rooms[amphipod.destination()].insert(amphipod);
						next_state.rooms[room_id].cells[room_position] = None;
						next_state.cost += hallway_cost + room_cost + exit_cost;
						moves.push(next_state);
					}
				} else {
					// for destination_index in [10] {
					for destination_index in [0, 1, 3, 5, 7, 9, 10] {
						if let Some(hallway_cost) =
							self.cost_to_move_to_position(room_index, destination_index, amphipod)
						{
							let mut next_state = *self;

							next_state.hallway[destination_index] = Some(amphipod);
							next_state.rooms[room_id].cells[room_position] = None;
							next_state.cost += hallway_cost + exit_cost;

							moves.push(next_state);
						}
					}
				}
			}
		}

		moves
	}
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Room<const N: usize> {
	id: Amphipod,
	cells: [Option<Amphipod>; N],
}

impl<const N: usize> Room<N> {
	fn is_solved(&self) -> bool {
		self.cells.iter().all(|cell| cell == &Some(self.id))
	}

	fn accepts(&self, amphipod: Amphipod) -> bool {
		self.id == amphipod
			&& self
				.cells
				.iter()
				.all(|&cell| cell.is_none() || cell == Some(self.id))
	}

	fn insert(&mut self, amphipod: Amphipod) -> usize {
		self.cells
			.iter_mut()
			.enumerate()
			.rev()
			.find(|(_, cell)| cell.is_none())
			.map(|(index, cell)| {
				*cell = Some(amphipod);

				(index + 1) * amphipod.cost()
			})
			.unwrap()
	}

	fn get_next(&self) -> Option<(Amphipod, usize, usize)> {
		if let Some((position, Some(amphipod))) =
			self.cells.iter().find_position(|cell| cell.is_some())
		{
			let exit_cost = (position + 1) * amphipod.cost();

			if self
				.cells
				.iter()
				.skip(position)
				.all(|&cell| cell == Some(self.id))
			{
				return None;
			}

			return Some((*amphipod, position, exit_cost));
		}

		None
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

#[cfg(test)]
mod addtional_tests {
	use super::*;

	fn make_input(input: &str) -> String {
		let mut s = String::from("\n\n");
		s.push_str(&input[0..4]);
		s.push('\n');
		s.push_str(&input[4..8]);

		s
	}

	#[test]
	fn states_can_be_determined_if_solved() {
		let a = parse::<2>(&make_input("ABCDABCD"));
		let b = parse::<2>(&make_input("AACDBBCD"));

		assert!(a.is_solved());
		assert!(!b.is_solved());
	}

	#[test]
	fn rooms_can_insert_amphipods() {
		let mut room = Room {
			id: Amphipod::Amber,
			cells: [None, None],
		};

		assert!(room.accepts(Amphipod::Amber));
		assert!(!room.accepts(Amphipod::Bronze));
		assert!(!room.accepts(Amphipod::Copper));
		assert!(!room.accepts(Amphipod::Desert));
		assert_eq!(2, room.insert(Amphipod::Amber));
		assert_eq!([None, Some(Amphipod::Amber)], room.cells);
		assert_eq!(1, room.insert(Amphipod::Amber));
		assert_eq!([Some(Amphipod::Amber), Some(Amphipod::Amber)], room.cells);
	}

	#[test]
	fn rooms_only_accept_if_they_have_been_emptied_of_invalid_ones() {
		let room = Room {
			id: Amphipod::Bronze,
			cells: [None, None, Some(Amphipod::Bronze), Some(Amphipod::Desert)],
		};

		assert!(!room.accepts(Amphipod::Amber));
		assert!(!room.accepts(Amphipod::Bronze));
		assert!(!room.accepts(Amphipod::Copper));
		assert!(!room.accepts(Amphipod::Desert));
	}

	#[test]
	fn amphipods_can_move_to_a_room() {
		let state = parse::<2>(&make_input("ABCDABCD"));

		for (amphipod, index, expected_cost) in [
			(Amphipod::Amber, 0, 2),
			(Amphipod::Amber, 10, 8),
			(Amphipod::Bronze, 0, 40),
			(Amphipod::Bronze, 10, 60),
			(Amphipod::Copper, 0, 600),
			(Amphipod::Copper, 10, 400),
			(Amphipod::Desert, 0, 8000),
			(Amphipod::Desert, 10, 2000),
		] {
			let mut state = state.clone();
			state.hallway[index] = Some(amphipod);

			assert_eq!(Some(expected_cost), state.cost_to_move(index, amphipod));
		}
	}

	#[test]
	fn a_blocked_hallway_returns_none_for_cost() {
		let state = parse::<2>(&make_input("ABCDABCD"));

		for (amphipod, index) in [
			(Amphipod::Amber, 10),
			(Amphipod::Bronze, 10),
			(Amphipod::Copper, 0),
			(Amphipod::Desert, 0),
		] {
			let mut state = state.clone();
			state.hallway[index] = Some(amphipod);
			state.hallway[5] = Some(amphipod);

			assert_eq!(None, state.cost_to_move(index, amphipod));
		}
	}

	#[test]
	fn a_room_can_say_what_amphipod_would_leave_it_next() {
		let mut state = parse::<2>(&make_input("ABCDDACB"));
		state.rooms[2].cells[0] = None;
		state.rooms[3].cells[0] = None;

		for (index, expected) in [
			(0, Some((Amphipod::Amber, 0, 1))),
			(1, Some((Amphipod::Bronze, 0, 10))),
			(2, None),
			(3, Some((Amphipod::Bronze, 1, 20))),
		] {
			assert_eq!(expected, state.rooms[index].get_next());
		}
	}

	#[test]
	fn amphipods_in_their_destination_will_leave_if_they_are_blocking_others() {
		let state = parse::<2>(&make_input("ABCDBCDA"));

		for (index, expected) in [
			(0, Some((Amphipod::Amber, 0, 1))),
			(1, Some((Amphipod::Bronze, 0, 10))),
			(2, Some((Amphipod::Copper, 0, 100))),
			(3, Some((Amphipod::Desert, 0, 1000))),
		] {
			assert_eq!(expected, state.rooms[index].get_next());
		}
	}

	#[test]
	fn stuck_state_move_test() {
		let mut state = parse::<2>(&make_input("ABCDABCA"));
		state.rooms[0].cells[0] = None;
		state.hallway[10] = Some(Amphipod::Desert);

		let moves = state.next_moves();

		assert_eq!(6, moves.len());
	}
}
