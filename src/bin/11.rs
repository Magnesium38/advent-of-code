use std::collections::HashMap;

use itertools::Itertools;

fn pt1(input: &str) -> anyhow::Result<isize> {
	dbg!(&input);

	let mut map: HashMap<(isize, isize), Option<u32>> = HashMap::new();

	input.lines().enumerate().for_each(|(i, line)| {
		line.chars().enumerate().for_each(|(j, c)| {
			map.insert((i as isize, j as isize), Some(c.to_digit(10).unwrap()));
		});
	});

	World::new(map)
		.take(100)
		.map(|(i, _)| i)
		.last()
		.ok_or(anyhow::anyhow!("no output"))
}

fn pt2(input: &str) -> anyhow::Result<isize> {
	dbg!(&input);

	let mut map: HashMap<(isize, isize), Option<u32>> = HashMap::new();

	input.lines().enumerate().for_each(|(i, line)| {
		line.chars().enumerate().for_each(|(j, c)| {
			map.insert((i as isize, j as isize), Some(c.to_digit(10).unwrap()));
		});
	});

	let r = World::new(map)
		.enumerate()
		.find(|(_, (_, a))| *a == 100);

	Ok((r.unwrap().0 + 1).try_into().unwrap())
}

struct World {
	map: HashMap<(isize, isize), Option<u32>>,
	count: isize,
}

impl World {
	fn new(map: HashMap<(isize, isize), Option<u32>>) -> Self {
		Self { map, count: 0 }
	}
}

impl Iterator for World {
	type Item = (isize, usize);

	fn next(&mut self) -> Option<Self::Item> {
		self.map.iter_mut().for_each(|(_, v)| {
			if let Some(v) = v {
				*v += 1;
			}
		});

		let mut x = 0;

		while self
			.map
			.iter()
			.find(|(_, v)| if let Some(v) = v { *v > 9 } else { false })
			.is_some()
		{
			let mut to_increment = Vec::new();
			let mut to_none = Vec::new();

			self.map.iter().for_each(|(&(x, y), v)| {
				if let Some(v) = v {
					if *v > 9 {
						for i in -1..=1 {
							for j in -1..=1 {
								to_increment.push((x + i, y + j));
							}
						}

						to_none.push((x, y));
					}
				}
			});

			x += to_none.len();
			to_none.iter().for_each(|(x, y)| {
				self.count += 1;
				self.map.insert((*x, *y), None);
			});

			to_increment.iter().for_each(|(x, y)| {
				if let Some(v) = self.map.get_mut(&(*x, *y)) {
					if let Some(v) = v {
						*v += 1;
					}
				}
			});
		}

		self.map.iter_mut().for_each(|(_, v)| {
			if let None = v {
				*v = Some(0);
			}
		});

		Some((self.count, x))
	}
}

advent::problem!(
	r#"
		5483143223
		2745854711
		5264556173
		6141336146
		6357385478
		4167524645
		2176841721
		6882881134
		4846848554
		5283751526
	"#,
	1656,
	195,
);
