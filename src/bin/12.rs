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
		path: String,
		visited: HashSet<&str>,
		has_double_visited: bool,
	) -> Vec<String> {
		let mut paths: Vec<String> = Vec::new();

		mapping[starting_point].iter().for_each(|end| {
			if end == &"start" {
				return;
			}

			if end == &"end" {
				let mut dest = path.clone();
				dest.push_str(format!(",{}", end).as_str());
				paths.push(dest);
				return;
			}

			let mut new_path = path.clone();
			new_path.push_str(format!(",{}", end).as_str());

			let mut visited = visited.clone();
			if end.chars().all(|c| c.is_ascii_lowercase()) {
				if visited.contains(end) {
					if has_double_visited {
						return;
					}

					let mut visited = visited.clone();
					visited.insert(end);
					paths.extend(find_paths(
						end,
						mapping.clone(),
						new_path.clone(),
						visited,
						true,
					));
					return;
				}

				visited.insert(end);
			}

			paths.extend(find_paths(
				end,
				mapping.clone(),
				new_path,
				visited,
				has_double_visited,
			));
		});

		paths
	}

	let paths = find_paths(
		"start",
		build_graph(input),
		String::from("start"),
		HashSet::new(),
		false,
	);

	Ok(paths.len())
}

fn build_graph(input: &str) -> HashMap<&str, Vec<&str>> {
	let mut mapping = HashMap::new();

	input.lines().for_each(|line| {
		let (start, end) = line.split('-').take(2).collect_tuple().unwrap();

		mapping.entry(start).or_insert_with(Vec::new).push(end);
		mapping.entry(end).or_insert_with(Vec::new).push(start);
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
