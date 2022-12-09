use hashbrown::HashSet;

fn move_tail(rope: &mut [(isize, isize)]) {
	let (head, tail) = if let [head, tail] = rope {
		(head, tail)
	} else {
		unreachable!("only a single tail can be moved at a time")
	};

	*tail = match ((head.0 - tail.0), (head.1 - tail.1)) {
		(-1..=1, -1..=1) => *tail,
		(dx @ -2, -1..=1) | (dx @ 2, -1..=1) => (head.0 - dx.signum(), head.1),
		(-1..=1, dy @ -2) | (-1..=1, dy @ 2) => (head.0, head.1 - dy.signum()),
		(dx, dy) => (head.0 - dx.signum(), head.1 - dy.signum()),
	};
}

fn move_head(
	pair: &mut [(isize, isize)],
	visited: &mut HashSet<(isize, isize)>,
	direction: char,
	amount: usize,
) {
	let delta = match direction {
		'U' => (0, 1),
		'D' => (0, -1),
		'L' => (-1, 0),
		'R' => (1, 0),
		_ => unreachable!("any other characters would be invalid input"),
	};

	for _ in 0..amount {
		pair[0] = (pair[0].0 + delta.0, pair[0].1 + delta.1);
		let tail_before = *pair
			.last()
			.expect("the pair should be at least two elements and thus should have a last");

		for i in 0..pair.len() - 1 {
			move_tail(&mut pair[i..=i + 1]);
		}

		let tail_after = *pair
			.last()
			.expect("the pair should be at least two elements and thus should have a last");
		if tail_after != tail_before {
			visited.insert(tail_after);
		}
	}
}

fn calculate_visited(input: &str, rope: &mut [(isize, isize)]) -> usize {
	let mut visited = HashSet::new();
	visited.insert(rope[0]);

	input
		.lines()
		.map(|line| {
			line.split_once(' ')
				.expect("any other shape is invalid input")
		})
		.map(|(direction, amount)| {
			(
				direction
					.chars()
					.next()
					.expect("expected the direction as a single character"),
				amount.parse().expect("expected a numeric amount"),
			)
		})
		.for_each(|(direction, amount): (char, usize)| {
			move_head(rope, &mut visited, direction, amount);
		});

	visited.len()
}

pub fn pt1(input: &str) -> anyhow::Result<usize> {
	Ok(calculate_visited(input, &mut [(0, 0), (0, 0)]))
}

pub fn pt2(input: &str) -> anyhow::Result<usize> {
	Ok(calculate_visited(
		input,
		&mut [
			(0, 0),
			(0, 0),
			(0, 0),
			(0, 0),
			(0, 0),
			(0, 0),
			(0, 0),
			(0, 0),
			(0, 0),
			(0, 0),
		],
	))
}

advent::problem!(
	r#"
		R 5
		U 8
		L 8
		D 3
		R 17
		D 10
		L 25
		U 20
	"#,
	88,
	36,
);
