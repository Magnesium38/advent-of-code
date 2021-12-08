use itertools::Itertools;

fn pt1(input: &str) -> anyhow::Result<isize> {
	let result = input.lines().fold(0, |total, line| {
		let output = line.split(" | ").nth(1).unwrap();
		output.split(' ').fold(total, |total, word| {
			if word.len() == 2 || word.len() == 4 || word.len() == 3 || word.len() == 7 {
				total + 1
			} else {
				total
			}
		})
	});

	Ok(result)
}

fn pt2(input: &str) -> anyhow::Result<isize> {
	Ok(input
		.lines()
		.map(|line| {
			let (uniques, input) = line.split(" | ").collect_tuple().unwrap();

			let one = uniques.split(' ').find(|word| word.len() == 2).unwrap();
			let four = uniques.split(' ').find(|word| word.len() == 4).unwrap();

			input.split(' ').fold(0, |total, word| {
				(total * 10)
					+ match (
						word.len(),
						word.chars().filter(|&c| !one.contains(c)).count(),
						word.chars().filter(|&c| !four.contains(c)).count(),
					) {
						(6, 4, 3) => 0,
						(2, _, _) => 1,
						(5, 4, 3) => 2,
						(5, 3, 2) => 3,
						(4, _, _) => 4,
						(5, 4, 2) => 5,
						(6, 5, 3) => 6,
						(3, _, _) => 7,
						(7, _, _) => 8,
						(6, 4, 2) => 9,
						(_, _, _) => unreachable!(),
					}
			})
		})
		.sum())
}

advent::problem!(
	r#"
		be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
		edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
		fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
		fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
		aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
		fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
		dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
		bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
		egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
		gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
	"#,
	26,
	61229,
);
