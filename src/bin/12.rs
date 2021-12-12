use std::collections::{HashMap, HashSet};
use itertools::Itertools;

fn pt1(input: &str) -> anyhow::Result<usize> {
	Ok(count_paths(
		&Cave::from("start", 0),
		&build_graph(input),
		&HashSet::new(),
		true,
	))
}

fn pt2(input: &str) -> anyhow::Result<usize> {
	Ok(count_paths(
		&Cave::from("start", 0),
		&build_graph(input),
		&HashSet::new(),
		false,
	))
}

const START: u8 = 0;
const END: u8 = 1;

fn count_paths(
	starting_point: &Cave,
	mapping: &HashMap<u8, Vec<Cave>>,
	visited: &HashSet<u8>,
	mut has_double_visited: bool,
) -> usize {
	if starting_point.id == END {
		return 1;
	}

	let is_lowercase = starting_point.lowercase;
	if is_lowercase && has_double_visited && visited.contains(&starting_point.id) {
		return 0;
	}

	let mut visited = visited.clone();
	if is_lowercase && visited.contains(&starting_point.id) {
		has_double_visited = true;
	}

	if is_lowercase {
		visited.insert(starting_point.id);
	}

	mapping[&starting_point.id]
		.iter()
		.map(|next_point| count_paths(next_point, mapping, &visited, has_double_visited))
		.sum()
}

fn build_graph(input: &str) -> HashMap<u8, Vec<Cave>> {
	let mut mapping = HashMap::new();
	let mut ids = HashMap::new();
	ids.insert("start", START);
	ids.insert("end", END);
	let mut id: u8 = 1;

	input.lines().for_each(|line| {
		let (start, end) = line.split('-').take(2).collect_tuple().unwrap();

		let start_id = *ids.entry(start).or_insert_with(|| {
			id += 1;
			id
		});
		let end_id = *ids.entry(end).or_insert_with(|| {
			id += 1;
			id
		});

		if start != "start" && end != "end" {
			mapping
				.entry(end_id)
				.or_insert_with(Vec::new)
				.push(Cave::from(start, start_id));
		}

		if end != "start" && start != "end" {
			mapping
				.entry(start_id)
				.or_insert_with(Vec::new)
				.push(Cave::from(end, end_id));
		}
	});

	mapping
}

struct Cave {
	id: u8,
	lowercase: bool,
}

impl Cave {
	fn from(s: &str, id: u8) -> Self {
		Cave {
			id,
			lowercase: s.chars().all(|c| c.is_ascii_lowercase()),
		}
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
