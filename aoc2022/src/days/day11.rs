use std::cmp::Reverse;

use itertools::Itertools;

struct Monkey {
	items: Vec<usize>,
	operation: Box<dyn Fn(usize) -> usize>,
	destinations: (usize, usize),
	inspect_count: usize,
	test_divisor: usize,
}

impl Monkey {
	fn new(input: &str) -> Self {
		let mut lines = input.lines();

		let starting_items = lines
			.nth(1)
			.unwrap()
			.split(':')
			.nth(1)
			.unwrap()
			.split(',')
			.map(|s| s.trim().parse().unwrap())
			.collect_vec();

		let operation = lines.next().unwrap();
		let rhs: Option<usize> = if operation.ends_with('d') {
			None
		} else {
			Some(
				operation
					.split_whitespace()
					.last()
					.unwrap()
					.parse()
					.unwrap(),
			)
		};
		let is_addition = operation.contains('+');
		let operator = move |x| match (is_addition, rhs) {
			(true, Some(y)) => x + y,
			(true, None) => x + x,
			(false, Some(y)) => x * y,
			(false, None) => x * x,
		};

		let test: usize = lines
			.next()
			.unwrap()
			.split_whitespace()
			.last()
			.unwrap()
			.parse()
			.unwrap();

		let if_true = lines
			.next()
			.unwrap()
			.split_whitespace()
			.last()
			.unwrap()
			.parse()
			.unwrap();
		let if_false = lines
			.next()
			.unwrap()
			.split_whitespace()
			.last()
			.unwrap()
			.parse()
			.unwrap();

		Monkey {
			items: starting_items,
			operation: Box::new(operator),
			destinations: (if_true, if_false),
			inspect_count: 0,
			test_divisor: test,
		}
	}
}

pub fn pt1(input: &str) -> anyhow::Result<usize> {
	let mut monkeys = input.split("\n\n").map(Monkey::new).collect_vec();

	for _ in 0..20 {
		for i in 0..monkeys.len() {
			let len = monkeys[i].items.len();
			let items = monkeys[i].items.drain(0..len).collect_vec();

			monkeys[i].inspect_count += items.len();

			for item in items {
				let worry = (monkeys[i].operation)(item) / 3;
				let destinations = monkeys[i].destinations;

				if worry % monkeys[i].test_divisor == 0 {
					monkeys[destinations.0].items.push(worry);
				} else {
					monkeys[destinations.1].items.push(worry);
				}
			}
		}
	}

	Ok(monkeys
		.into_iter()
		.map(|monkey| Reverse(monkey.inspect_count))
		.sorted()
		.take(2)
		.map(|count| count.0)
		.product())
}

pub fn pt2(input: &str) -> anyhow::Result<usize> {
	let mut monkeys = input.split("\n\n").map(Monkey::new).collect_vec();
	let common_divisor: usize = monkeys.iter().map(|monkey| monkey.test_divisor).product();

	for _ in 0..10000 {
		for i in 0..monkeys.len() {
			let len = monkeys[i].items.len();
			let items = monkeys[i].items.drain(0..len).collect_vec();

			monkeys[i].inspect_count += items.len();

			for item in items {
				let worry = (monkeys[i].operation)(item);
				let destinations = monkeys[i].destinations;

				if worry % monkeys[i].test_divisor == 0 {
					monkeys[destinations.0].items.push(worry % common_divisor);
				} else {
					monkeys[destinations.1].items.push(worry % common_divisor);
				}
			}
		}
	}

	Ok(monkeys
		.into_iter()
		.map(|monkey| Reverse(monkey.inspect_count))
		.sorted()
		.take(2)
		.map(|count| count.0)
		.product())
}

advent::problem!(
	r#"
		Monkey 0:
		  Starting items: 79, 98
		  Operation: new = old * 19
		  Test: divisible by 23
			If true: throw to monkey 2
			If false: throw to monkey 3

		Monkey 1:
		  Starting items: 54, 65, 75, 74
		  Operation: new = old + 6
		  Test: divisible by 19
			If true: throw to monkey 2
			If false: throw to monkey 0

		Monkey 2:
		  Starting items: 79, 60, 97
		  Operation: new = old * old
		  Test: divisible by 13
			If true: throw to monkey 1
			If false: throw to monkey 3

		Monkey 3:
		  Starting items: 74
		  Operation: new = old + 3
		  Test: divisible by 17
			If true: throw to monkey 0
			If false: throw to monkey 1
    "#,
	10605,
	2713310158,
);
