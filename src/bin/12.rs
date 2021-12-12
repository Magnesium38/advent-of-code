use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn pt1(input: &str) -> anyhow::Result<usize> {
	fn count_paths(
		starting_point: &str,
		mapping: &HashMap<&str, Vec<&str>>,
		visited: &HashSet<&str>,
	) -> usize {
		if starting_point == "end" {
			return 1;
		}

		let is_lowercase = starting_point.chars().all(|c| c.is_ascii_lowercase());
		if visited.contains(starting_point) && is_lowercase {
			return 0;
		}

		let mut visited = visited.clone();
		if is_lowercase {
			visited.insert(starting_point);
		}

		mapping[starting_point]
			.iter()
			.map(|&next_point| count_paths(next_point, mapping, &visited))
			.sum()
	}

	Ok(count_paths("start", &build_graph(input), &HashSet::new()))
}

fn pt2(input: &str) -> anyhow::Result<usize> {
	fn find_paths(
		starting_point: &str,
		mapping: HashMap<&str, Vec<&str>>,
		visited: HashSet<&str>,
		has_double_visited: bool,
	) -> usize {
		let mut count = 0;

		mapping[starting_point].iter().for_each(|end| {
			if end == &"end" {
				count += 1;
				return;
			}

			let mut visited = visited.clone();
			if end.chars().all(|c| c.is_ascii_lowercase()) {
				if visited.contains(end) {
					if has_double_visited {
						return;
					}

					let mut visited = visited.clone();
					visited.insert(end);
					count += find_paths(end, mapping.clone(), visited, true);
					return;
				}

				visited.insert(end);
			}

			count += find_paths(end, mapping.clone(), visited, has_double_visited);
		});

		count
	}

	Ok(find_paths(
		"start",
		build_graph(input),
		HashSet::new(),
		false,
	))
}

fn build_graph(input: &str) -> HashMap<&str, Vec<&str>> {
	let mut mapping = HashMap::new();

	input.lines().for_each(|line| {
		let (start, end) = line.split('-').take(2).collect_tuple().unwrap();

		if start != "start" && end != "end" {
			mapping.entry(end).or_insert_with(Vec::new).push(start);
		}

		mapping.entry(start).or_insert_with(Vec::new).push(end);
	});

	mapping
}

advent::problem!(
	r#"
		start-A
		start-b
		A-c
		A-b
		b-d
		A-end
		b-end
	"#,
	10,
	36,
);
