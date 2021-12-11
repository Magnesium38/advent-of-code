fn pt1(input: &str) -> anyhow::Result<isize> {
	World::new(input)
		.take(100)
		.map(|(i, _)| i)
		.last()
		.ok_or(anyhow::anyhow!("no output"))
}

fn pt2(input: &str) -> anyhow::Result<isize> {
	let r = World::new(input).enumerate().find(|(_, (_, a))| *a == 100);

	Ok((r.unwrap().0 + 1).try_into().unwrap())
}

struct World {
	grid: advent::Grid<Option<u32>>,
	count: isize,
}

impl World {
	fn new(input: &str) -> Self {
		Self {
			grid: advent::Grid::new(input).map(|c| Some(c)),
			count: 0,
		}
	}
}

impl Iterator for World {
	type Item = (isize, usize);

	fn next(&mut self) -> Option<Self::Item> {
		self.grid.iter_mut().for_each(|(_, v)| {
			if let Some(v) = v {
				*v += 1;
			}
		});

		let mut x = 0;

		while self
			.grid
			.iter()
			.any(|(_, v)| if let Some(v) = v { *v > 9 } else { false })
		{
			let mut to_increment = Vec::new();
			let mut to_none = Vec::new();

			self.grid.iter().for_each(|((x, y), v)| {
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
			to_none.iter().for_each(|&(x, y)| {
				self.count += 1;
				if let Some(v) = self.grid.get_mut(x, y) {
					*v = None;
				}
			});

			to_increment.iter().for_each(|&(x, y)| {
				if let Some(Some(v)) = self.grid.get_mut(x, y) {
					*v += 1;
				}
			});
		}

		self.grid.iter_mut().for_each(|(_, v)| {
			if v.is_none() {
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
