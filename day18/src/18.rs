use itertools::Itertools;
use std::ops::Add;

pub fn pt1(input: &str) -> anyhow::Result<usize> {
	input
		.lines()
		.map(|line| {
			Snailfish::new(line).map(|mut snailfish| {
				snailfish.reduce();

				snailfish
			})
		})
		.collect::<Result<Vec<_>, _>>()?
		.into_iter()
		.reduce(|acc, snailfish| &acc + &snailfish)
		.map(|snailfish| snailfish.magnitude())
		.ok_or(anyhow::anyhow!("No magnitude found"))
}

pub fn pt2(input: &str) -> anyhow::Result<usize> {
	input
		.lines()
		.map(|line| {
			Snailfish::new(line).map(|mut snailfish| {
				snailfish.reduce();

				snailfish
			})
		})
		.collect::<Result<Vec<_>, _>>()?
		.into_iter()
		.tuple_combinations::<(_, _)>()
		.map(|(a, b)| (&a + &b).magnitude().max((&b + &a).magnitude()))
		.max()
		.ok_or(anyhow::anyhow!("No maximum found"))
}

#[derive(Clone, Debug, PartialEq)]
struct Snailfish {
	numbers: Vec<(usize, usize)>,
}

impl Snailfish {
	fn new(input: &str) -> anyhow::Result<Snailfish> {
		let mut depth = 0;
		let mut numbers = Vec::new();

		input.chars().for_each(|c| match c {
			'[' => depth += 1,
			']' => depth -= 1,
			',' => {}
			'0'..='9' => {
				numbers.push((c.to_digit(10).unwrap() as usize, depth));
			}
			_ => panic!("unexpected character: {}", c),
		});

		Ok(Snailfish { numbers })
	}

	fn reduce(&mut self) {
		loop {
			if !self.explode() {
				break;
			}
		}

		if self.split() {
			self.reduce();
		}
	}

	fn explode(&mut self) -> bool {
		if let Some(i) = self.numbers.iter().position(|&(_, depth)| depth > 4) {
			let (lhs, rhs) = (self.numbers[i], self.numbers[i + 1]);

			if i > 0 {
				self.numbers[i - 1].0 += lhs.0;
			}

			if i < self.numbers.len() - 2 {
				self.numbers[i + 2].0 += rhs.0;
			}

			self.numbers.splice(i..i + 2, vec![(0, lhs.1 - 1)]);

			true
		} else {
			false
		}
	}

	fn split(&mut self) -> bool {
		if let Some(i) = self.numbers.iter().position(|&(value, _)| value > 9) {
			let (value, depth) = self.numbers[i];

			let value = (value as f64) / 2.0;

			self.numbers.splice(
				i..=i,
				vec![
					(value.floor() as usize, depth + 1),
					(value.ceil() as usize, depth + 1),
				],
			);

			true
		} else {
			false
		}
	}

	fn magnitude(mut self) -> usize {
		while let Some(index) = self
			.numbers
			.iter()
			.tuple_windows::<(_, _)>()
			.position(|(lhs, rhs)| lhs.1 == rhs.1)
		{
			let (lhs, depth) = self.numbers[index];
			let (rhs, _) = self.numbers[index + 1];

			self.numbers
				.splice(index..index + 2, vec![(lhs * 3 + rhs * 2, depth - 1)]);
		}

		self.numbers.first().map(|&(value, _)| value).unwrap()
	}
}

impl<'a, 'b> Add<&'b Snailfish> for &'a Snailfish {
	type Output = Snailfish;

	fn add(self, other: &'b Snailfish) -> Snailfish {
		let mut snailfish = Snailfish {
			numbers: self
				.numbers
				.iter()
				.map(|(value, depth)| (*value, depth + 1))
				.chain(
					other
						.numbers
						.iter()
						.map(|(value, depth)| (*value, depth + 1)),
				)
				.collect(),
		};

		snailfish.reduce();

		snailfish
	}
}

advent::problem!(
	r#"
		[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
		[[[5,[2,8]],4],[5,[[9,9],0]]]
		[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
		[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
		[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
		[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
		[[[[5,4],[7,7]],8],[[8,3],8]]
		[[9,3],[[9,9],[6,[4,9]]]]
		[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
		[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
	"#,
	4140,
	3993,
);

#[cfg(test)]
mod addtional_tests {
	use super::*;

	#[test]
	fn snailfish_reducing() -> anyhow::Result<()> {
		for (input, expected) in vec![
			("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]"),
			(
				"[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]",
				"[[[[0,7],4],[[7,8],[6,0]]],[8,1]]",
			),
		] {
			let mut snailfish = Snailfish::new(input)?;

			snailfish.reduce();

			assert_eq!(snailfish, Snailfish::new(expected)?);
		}

		Ok(())
	}

	#[test]
	fn snailfish_magnitude() -> anyhow::Result<()> {
		for (input, expected) in vec![
			("[[1,2],[[3,4],5]]", 143),
			("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", 1384),
			("[[[[1,1],[2,2]],[3,3]],[4,4]]", 445),
			("[[[[3,0],[5,3]],[4,4]],[5,5]]", 791),
			("[[[[5,0],[7,4]],[5,5]],[6,6]]", 1137),
			(
				"[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
				3488,
			),
		] {
			assert_eq!(Snailfish::new(input)?.magnitude(), expected);
		}

		Ok(())
	}
}
