use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn pt1(input: &str) -> anyhow::Result<usize> {
	fn find_paths(
		starting_point: &str,
		mapping: HashMap<&str, Vec<&str>>,
		path: String,
		visited: HashSet<&str>,
	) -> Vec<String> {
		let mut paths: Vec<String> = Vec::new();

		mapping
			.clone()
			.get(starting_point)
			.unwrap()
			.iter()
			.for_each(|end| {
				if end == &"end" {
					let mut dest = path.clone();
					dest.push_str(format!(",{}", end).as_str());
					paths.push(dest);
					return;
				}

				if visited.contains(end) {
					return;
				}

				let mut new_path = path.clone();
				new_path.push_str(format!(",{}", end).as_str());

				let mut visited = visited.clone();
				if end.chars().next().unwrap().is_ascii_lowercase() {
					visited.insert(end);
				}

				paths.extend(find_paths(end, mapping.clone(), new_path, visited));
			});

		paths
	}

	let mut mapping = HashMap::new();

	input.lines().for_each(|line| {
		let (start, end) = line.split('-').take(2).collect_tuple().unwrap();

		{
			mapping.entry(start).or_insert_with(Vec::new).push(end);
		}

		{
			mapping.entry(end).or_insert_with(Vec::new).push(start);
		}
	});

	let paths = find_paths(
		"start",
		mapping.clone(),
		String::from("start"),
		HashSet::from_iter(vec!["start"]),
	);

	Ok(paths.len())
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

		mapping
			.clone()
			.get(starting_point)
			.unwrap()
			.iter()
			.for_each(|end| {
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
				if end.chars().next().unwrap().is_ascii_lowercase() {
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

	let mut mapping = HashMap::new();

	input.lines().for_each(|line| {
		let (start, end) = line.split('-').take(2).collect_tuple().unwrap();

		{
			mapping.entry(start).or_insert_with(Vec::new).push(end);
		}

		{
			mapping.entry(end).or_insert_with(Vec::new).push(start);
		}
	});

	let paths = find_paths(
		"start",
		mapping.clone(),
		String::from("start"),
		HashSet::from_iter(vec!["start"]),
		false,
	);

	Ok(paths.len())
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
