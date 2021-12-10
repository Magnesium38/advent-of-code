fn pt1(input: &str) -> anyhow::Result<isize> {
	let result = input
		.lines()
		.map(|line| {
			let mut last_length = 0;
			let mut line = line.to_string();

			while line.len() != last_length {
				last_length = line.len();
				line = line
					.replace("()", "")
					.replace("[]", "")
					.replace("{}", "")
					.replace("<>", "");
			}

			for c in line.chars() {
				match c {
					')' => return 3,
					']' => return 57,
					'}' => return 1197,
					'>' => return 25137,
					_ => continue,
				}
			}

			0
		})
		.collect::<Vec<_>>();

	Ok(result.iter().sum::<isize>())
}

fn pt2(input: &str) -> anyhow::Result<isize> {
	let mut scores = input
		.lines()
		.map(|line| {
			let mut last_length = 0;
			let mut line = line.to_string();

			while line.len() != last_length {
				last_length = line.len();
				line = line
					.replace("()", "")
					.replace("[]", "")
					.replace("{}", "")
					.replace("<>", "");
			}

			for c in line.chars() {
				match c {
					')' | ']' | '}' | '>' => return 0,
					_ => continue,
				}
			}

			line = line
				.chars()
				.rev()
				.collect::<String>()
				.replace('(', ")")
				.replace('[', "]")
				.replace('{', "}")
				.replace('<', ">");

			line.chars().fold(0, |acc, c| {
				(acc * 5)
					+ match c {
						')' => 1,
						']' => 2,
						'}' => 3,
						'>' => 4,
						_ => 0,
					}
			})
		})
		.filter(|&score| score > 0)
		.collect::<Vec<_>>();

	scores.sort_unstable();
	let len = scores.len();

	Ok(scores[len / 2])
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
