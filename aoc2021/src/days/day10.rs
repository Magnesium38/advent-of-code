use itertools::Itertools;

pub fn pt1(input: &str) -> anyhow::Result<isize> {
	Ok(input
		.lines()
		.map(|line| {
			let mut stack = Vec::new();

			line.chars()
				.find_map(|c| handle_character(&mut stack, c))
				.unwrap_or(0)
		})
		.sum())
}

pub fn pt2(input: &str) -> anyhow::Result<isize> {
	let scores = input
		.lines()
		.map(|line| {
			let mut stack = Vec::new();

			match line.chars().find_map(|c| handle_character(&mut stack, c)) {
				Some(_) => 0,
				None => {
					stack.reverse();
					stack.iter().fold(0, |acc, c| {
						(acc * 5)
							+ match c {
								'(' => 1,
								'[' => 2,
								'{' => 3,
								'<' => 4,
								_ => unreachable!(),
							}
					})
				}
			}
		})
		.filter(|&score| score > 0)
		.sorted()
		.collect_vec();

	Ok(scores[scores.len() / 2])
}

fn handle_character(stack: &mut Vec<char>, c: char) -> Option<isize> {
	match c {
		'(' | '[' | '{' | '<' => {
			stack.push(c);
			None
		}

		_ => match (stack.pop(), c) {
			(Some('('), ')') | (Some('['), ']') | (Some('{'), '}') | (Some('<'), '>') => None,

			(Some(_), _) => match c {
				')' => Some(3),
				']' => Some(57),
				'}' => Some(1197),
				'>' => Some(25137),
				_ => unreachable!(),
			},

			(None, _) => unreachable!(),
		},
	}
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
