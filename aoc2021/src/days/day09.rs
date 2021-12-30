use std::collections::HashMap;

use itertools::Itertools;

pub fn pt1(input: &str) -> anyhow::Result<isize> {
	let mut map: HashMap<(isize, isize), u32> = HashMap::new();

	input.lines().enumerate().for_each(|(i, line)| {
		line.chars().enumerate().for_each(|(j, c)| {
			map.insert(
				(i.try_into().unwrap(), j.try_into().unwrap()),
				c.to_digit(10).unwrap(),
			);
		});
	});

	let mut low_points = Vec::new();

	map.iter().for_each(|(&(x, y), v)| {
		if let Some(n) = map.get(&(x - 1, y)) {
			if v >= n {
				return;
			}
		}
		if let Some(n) = map.get(&(x + 1, y)) {
			if v >= n {
				return;
			}
		}
		if let Some(n) = map.get(&(x, y - 1)) {
			if v >= n {
				return;
			}
		}
		if let Some(n) = map.get(&(x, y + 1)) {
			if v >= n {
				return;
			}
		}

		low_points.push(v);
	});

	Ok(low_points
		.iter()
		.map(|&n| n + 1)
		.fold(0, |a, b| a + (b as isize)))
}

pub fn pt2(input: &str) -> anyhow::Result<isize> {
	let mut map: HashMap<(isize, isize), u32> = HashMap::new();

	input.lines().enumerate().for_each(|(i, line)| {
		line.chars().enumerate().for_each(|(j, c)| {
			map.insert(
				(i.try_into().unwrap(), j.try_into().unwrap()),
				c.to_digit(10).unwrap(),
			);
		});
	});

	let mut low_points = Vec::new();

	map.iter().for_each(|(&(x, y), v)| {
		if let Some(n) = map.get(&(x - 1, y)) {
			if v >= n {
				return;
			}
		}
		if let Some(n) = map.get(&(x + 1, y)) {
			if v >= n {
				return;
			}
		}
		if let Some(n) = map.get(&(x, y - 1)) {
			if v >= n {
				return;
			}
		}
		if let Some(n) = map.get(&(x, y + 1)) {
			if v >= n {
				return;
			}
		}

		low_points.push((x, y));
	});

	let (a, b, c) = low_points
		.iter()
		.map(|&(x, y)| {
			let mut count = 1;
			let mut visited = HashMap::new();
			let low_point = map.get(&(x, y)).unwrap();
			let mut to_visit = vec![
				(x - 1, y, low_point),
				(x + 1, y, low_point),
				(x, y - 1, low_point),
				(x, y + 1, low_point),
			];
			visited.insert((x, y), true);

			while !to_visit.is_empty() {
				let (x, y, low_point) = to_visit.pop().unwrap();
				let has_visited = visited.entry((x, y)).or_insert(false);

				if *has_visited {
					continue;
				}

				let n = map.get(&(x, y));
				if matches!(n, None) {
					continue;
				}

				let n = n.unwrap();
				if n > low_point && n < &9 {
					count += 1;

					visited.insert((x, y), true);

					to_visit.push((x - 1, y, n));
					to_visit.push((x + 1, y, n));
					to_visit.push((x, y - 1, n));
					to_visit.push((x, y + 1, n));
				}
			}

			count
		})
		.sorted()
		.rev()
		.take(3)
		.collect_tuple()
		.unwrap();

	Ok(a * b * c)
}

advent::problem!(
	r#"
		2199943210
		3987894921
		9856789892
		8767896789
		9899965678
	"#,
	15,
	1134,
);
