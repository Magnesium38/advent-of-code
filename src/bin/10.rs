use itertools::Itertools;

fn pt1(input: &str) -> anyhow::Result<isize> {
	Ok(input
		.lines()
		.map(|line| {
			let mut stack = Vec::new();

			line.chars()
				.find_map(|c| match c {
					'(' | '[' | '{' | '<' => {
						stack.push(c);
						None
					}

					_ => match (stack.pop(), c) {
						(Some('('), ')')
						| (Some('['), ']')
						| (Some('{'), '}')
						| (Some('<'), '>') => None,

						(Some(_), _) => match c {
							')' => return Some(3),
							']' => return Some(57),
							'}' => return Some(1197),
							'>' => return Some(25137),
							_ => unreachable!(),
						},

						(None, _) => unreachable!(),
					},
				})
				.unwrap_or(0)
		})
		.sum())
}

fn pt2(input: &str) -> anyhow::Result<isize> {
	let scores = input
		.lines()
		.map(|line| {
			let mut stack = Vec::new();

			let is_corrupted = line.chars().find_map(|c| match c {
				'(' | '[' | '{' | '<' => {
					stack.push(c);
					None
				}

				_ => match (stack.pop(), c) {
					(Some('('), ')') | (Some('['), ']') | (Some('{'), '}') | (Some('<'), '>') => {
						None
					}

					(Some(_), _) => match c {
						')' | ']' | '}' | '>' => return Some(true),
						_ => unreachable!(),
					},

					(None, _) => unreachable!(),
				},
			});

			if let Some(true) = is_corrupted {
				return 0;
			}

			stack.reverse();
			stack.iter().fold(0, |acc, c| {
				(acc * 5)
					+ match c {
						'(' => 1,
						'[' => 2,
						'{' => 3,
						'<' => 4,
						_ => 0,
					}
			})
		})
		.filter(|&score| score > 0)
		.sorted()
		.collect::<Vec<_>>();

	Ok(scores[scores.len() / 2])
}

advent::problem!(
	r#"
		[({(<(())[]>[[{[]{<()<>>
		[(()[<>])]({[<{<<[]>>(
		{([(<{}[<>[]}>{[]{[(<()>
		(((({<>}<{<{<>}{[]{[]{}
		[[<[([]))<([[{}[[()]]]
		[{[{({}]{}}([{[{{{}}([]
		{<[[]]>}<{[{[{[]{()[[[]
		[<(<(<(<{}))><([]([]()
		<{([([[(<>()){}]>(<<{{
		<{([{{}}[<[[[<>{}]]]>[]]
	"#,
	26397,
	288957,
);
