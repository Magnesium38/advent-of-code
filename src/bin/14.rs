use hashbrown::HashMap;
use itertools::Itertools;

fn pt1(input: &str) -> anyhow::Result<usize> {
	Solver::new(input)
		.take(10)
		.last()
		.ok_or(anyhow::anyhow!("no solution"))
}

fn pt2(input: &str) -> anyhow::Result<usize> {
	Solver::new(input)
		.take(40)
		.last()
		.ok_or(anyhow::anyhow!("no solution"))
}

struct Solver {
	pairs: HashMap<(char, char), usize>,
	rules: HashMap<(char, char), char>,
	last: char,
}

impl Solver {
	fn new(input: &str) -> Self {
		let (template, rules) = input.split_once("\n\n").unwrap();

		Solver {
			rules: rules
				.lines()
				.map(|line| line.split_once(" -> ").unwrap())
				.map(|(a, b)| {
					(
						a.chars().collect_tuple::<(_, _)>().unwrap(),
						b.chars().nth(0).unwrap(),
					)
				})
				.fold(HashMap::new(), |mut acc, (a, b)| {
					acc.insert(a, b);
					acc
				}),
			pairs: template.chars().tuple_windows().map(|(a, b)| (a, b)).fold(
				HashMap::new(),
				|mut acc, pair| {
					*acc.entry(pair).or_insert(0) += 1;
					acc
				},
			),
			last: template.chars().last().unwrap(),
		}
	}
}

impl Iterator for Solver {
	type Item = usize;

	fn next(&mut self) -> Option<Self::Item> {
		self.pairs = self.pairs.iter().fold(
			HashMap::new(),
			|mut new_pairs, ((first, second), amount)| {
				let third = self.rules.get(&(*first, *second)).unwrap();

				*new_pairs.entry((*first, *third)).or_insert(0) += amount;
				*new_pairs.entry((*third, *second)).or_insert(0) += amount;

				new_pairs
			},
		);

		let counter: HashMap<char, usize> = self.pairs.iter().fold(
			HashMap::from_iter(vec![(self.last, 1)]),
			|mut counter, ((c, _), amount)| {
				*counter.entry(*c).or_insert(0) += amount;
				counter
			},
		);

		let (_, min) = counter.iter().min_by_key(|(_, v)| *v).unwrap();
		let (_, max) = counter.iter().max_by_key(|(_, v)| *v).unwrap();

		Some(max - min)
	}
}

advent::problem!(
	r#"
		NNCB

		CH -> B
		HH -> N
		CB -> H
		NH -> C
		HB -> C
		HC -> B
		HN -> C
		NN -> C
		BH -> H
		NC -> B
		NB -> B
		BN -> B
		BB -> N
		BC -> B
		CC -> N
		CN -> C
	"#,
	1588,
	2188189693529,
);
