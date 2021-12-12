use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn pt1(input: &str) -> anyhow::Result<usize> {
	fn count_paths(
		starting_point: &Cave,
		mapping: &HashMap<&str, Vec<Cave>>,
		visited: &HashSet<&str>,
	) -> usize {
		if starting_point.s == "end" {
			return 1;
		}

		let is_lowercase = starting_point.lowercase;
		if visited.contains(starting_point.s) && is_lowercase {
			return 0;
		}

		let mut visited = visited.clone();
		if is_lowercase {
			visited.insert(starting_point.s);
		}

		mapping[starting_point.s]
			.iter()
			.map(|next_point| count_paths(next_point, mapping, &visited))
			.sum()
	}

	Ok(count_paths(
		&Cave::from("start"),
		&build_graph(input),
		&HashSet::new(),
	))
}

fn pt2(input: &str) -> anyhow::Result<usize> {
	fn count_paths(
		starting_point: &Cave,
		mapping: &HashMap<&str, Vec<Cave>>,
		visited: &HashSet<&str>,
		mut has_double_visited: bool,
	) -> usize {
		if starting_point.s == "end" {
			return 1;
		}

		let is_lowercase = starting_point.lowercase;
		if is_lowercase && has_double_visited && visited.contains(starting_point.s) {
			return 0;
		}

		let mut visited = visited.clone();
		if is_lowercase && visited.contains(starting_point.s) {
			has_double_visited = true;
		}

		if is_lowercase {
			visited.insert(starting_point.s);
		}

		mapping[starting_point.s]
			.iter()
			.map(|next_point| count_paths(next_point, mapping, &visited, has_double_visited))
			.sum()
	}

	Ok(count_paths(
		&Cave::from("start"),
		&build_graph(input),
		&HashSet::new(),
		false,
	))
}

fn build_graph(input: &str) -> HashMap<&str, Vec<Cave>> {
	let mut mapping = HashMap::new();

	input.lines().for_each(|line| {
		let (start, end) = line.split('-').take(2).collect_tuple().unwrap();

		if start != "start" && end != "end" {
			mapping
				.entry(end)
				.or_insert_with(Vec::new)
				.push(Cave::from(start));
		}

		if end != "start" && start != "end" {
			mapping
				.entry(start)
				.or_insert_with(Vec::new)
				.push(Cave::from(end));
		}
	});

	mapping
}

struct Cave<'a> {
	s: &'a str,
	lowercase: bool,
}

impl<'a> Cave<'a> {
	fn from(s: &'a str) -> Self {
		Cave {
			s,
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
