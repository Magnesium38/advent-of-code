pub fn pt1(input: &str) -> anyhow::Result<isize> {
	World::new(input.split(',').map(|s| s.parse().unwrap()))
		.take(80)
		.last()
		.ok_or(anyhow::anyhow!("no output"))
}

pub fn pt2(input: &str) -> anyhow::Result<isize> {
	World::new(input.split(',').map(|s| s.parse().unwrap()))
		.take(256)
		.last()
		.ok_or(anyhow::anyhow!("no output"))
}

struct World([isize; 9]);

impl World {
	fn new<I: Iterator<Item = usize>>(input: I) -> Self {
		let mut world = [0; 9];

		for i in input {
			world[i] += 1;
		}

		Self(world)
	}
}

impl Iterator for World {
	type Item = isize;

	fn next(&mut self) -> Option<Self::Item> {
		let zeroes = self.0[0];
		let mut sum = zeroes + zeroes;

		for i in 1..=8 {
			let x = self.0[i];
			self.0[i - 1] = x;
			sum += x;
		}

		self.0[6] += zeroes;
		self.0[8] = zeroes;

		Some(sum)
	}
}

advent::problem!(
	r#"
		3,4,3,1,2
    "#,
	5934,
	26984457539,
);
