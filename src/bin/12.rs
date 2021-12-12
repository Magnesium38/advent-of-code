use std::collections::HashMap;

fn pt1(input: &str) -> anyhow::Result<usize> {
	Ok(count_paths(
		&Cave::from(0, true),
		&build_graph(input),
		&mut Vec::with_capacity(25),
		true,
	))
}

fn pt2(input: &str) -> anyhow::Result<usize> {
	Ok(count_paths(
		&Cave::from(0, true),
		&build_graph(input),
		&mut Vec::with_capacity(25),
		false,
	))
}

const START: u8 = 0;
const END: u8 = 1;

fn count_paths(
	cave: &Cave,
	mapping: &HashMap<u8, Vec<Cave>>,
	visited: &mut Vec<u8>,
	mut has_double_visited: bool,
) -> usize {
	if cave.id == END {
		return 1;
	}

	if cave.is_small && has_double_visited && visited.contains(&cave.id) {
		return 0;
	}

	if cave.is_small && visited.contains(&cave.id) {
		has_double_visited = true;
	}

	visited.push(cave.id);

	let result = mapping[&cave.id]
		.iter()
		.map(|next_point| count_paths(next_point, mapping, visited, has_double_visited))
		.sum();

	visited.pop();

	result
}

fn build_graph(input: &str) -> HashMap<u8, Vec<Cave>> {
	let mut mapping = HashMap::new();
	let mut ids = HashMap::new();
	ids.insert("start", START);
	ids.insert("end", END);
	let mut id: u8 = 1;

	input.lines().for_each(|line| {
		let (start, end) = line.split_once('-').unwrap();

		let start_id = *ids.entry(start).or_insert_with(|| {
			id += 1;
			id
		});
		let end_id = *ids.entry(end).or_insert_with(|| {
			id += 1;
			id
		});

		if start_id != START && end_id != END {
			let start_is_small = start.chars().next().unwrap().is_lowercase();

			mapping
				.entry(end_id)
				.or_insert_with(|| Vec::with_capacity(10))
				.push(Cave::from(start_id, start_is_small));
		}

		if end_id != START && start_id != END {
			let end_is_small = end.chars().next().unwrap().is_lowercase();

			mapping
				.entry(start_id)
				.or_insert_with(|| Vec::with_capacity(10))
				.push(Cave::from(end_id, end_is_small));
		}
	});

	mapping
}

struct Cave {
	id: u8,
	is_small: bool,
}

impl Cave {
	fn from(id: u8, is_small: bool) -> Self {
		Cave { id, is_small }
	}
}

advent::problem!(
	r#"
		dc-end
		HN-start
		start-kj
		dc-start
		dc-HN
		LN-dc
		HN-end
		kj-sa
		kj-HN
		kj-dc
	"#,
	19,
	103,
);
