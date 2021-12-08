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

			let one = uniques.split(" ").find(|word| word.len() == 2).unwrap();
			let seven = uniques.split(" ").find(|word| word.len() == 3).unwrap();
			let four = uniques.split(" ").find(|word| word.len() == 4).unwrap();
			let eight = uniques.split(" ").find(|word| word.len() == 7).unwrap();
			let six = uniques
				.split(" ")
				.filter(|word| word.len() == 6)
				.find(|word| word.chars().filter(|&c| !seven.contains(c)).count() == 4)
				.unwrap();
			let three = uniques
				.split(" ")
				.filter(|word| word.len() == 5)
				.find(|word| word.chars().filter(|&c| !seven.contains(c)).count() == 2)
				.unwrap();

			let top = seven.chars().filter(|&c| !one.contains(c)).next().unwrap();
			let top_left = four.chars().filter(|&c| !three.contains(c)).next().unwrap();
			let top_right = eight.chars().filter(|&c| !six.contains(c)).next().unwrap();
			let bottom_right = one.chars().filter(|&c| c != top_right).next().unwrap();
			let bottom = three
				.chars()
				.filter(|&c| !four.contains(c) && c != top)
				.next()
				.unwrap();
			let middle = four
				.chars()
				.filter(|&c| c != top_left && c != top_right && c != bottom_right)
				.next()
				.unwrap();
			let bottom_left = ('a'..='g')
				.filter(|&c| {
					c != top
						&& c != top_left && c != top_right
						&& c != middle && c != bottom_right
						&& c != bottom
				})
				.next()
				.unwrap();

			let key = format!(
				"{}{}{}{}{}{}{}",
				top, top_left, top_right, middle, bottom_left, bottom_right, bottom
			);

			input
				.split(' ')
				.map(|digit| get_digit(digit, &key))
				.fold_options(0, |total, digit| (total * 10) + digit)
				.unwrap()
		})
		.sum())
}

fn get_digit(digit: &str, key: &str) -> Option<isize> {
	let (top, top_left, top_right, middle, bottom_left, bottom_right, bottom) =
		key.chars().collect_tuple().unwrap();

	match digit.len() {
		2 => {
			if digit.contains(top_right) && digit.contains(bottom_right) {
				Some(1)
			} else {
				None
			}
		}
		3 => {
			if digit.contains(top) && digit.contains(top_right) && digit.contains(bottom_right) {
				Some(7)
			} else {
				None
			}
		}
		4 => {
			if digit.contains(top_left)
				&& digit.contains(top_right)
				&& digit.contains(middle)
				&& digit.contains(bottom_right)
			{
				Some(4)
			} else {
				None
			}
		}
		7 => Some(8),
		5 => {
			if digit.contains(top)
				&& digit.contains(top_right)
				&& digit.contains(middle)
				&& digit.contains(bottom_left)
				&& digit.contains(bottom)
			{
				Some(2)
			} else if digit.contains(top)
				&& digit.contains(top_right)
				&& digit.contains(middle)
				&& digit.contains(bottom_right)
				&& digit.contains(bottom)
			{
				Some(3)
			} else if digit.contains(top)
				&& digit.contains(top_left)
				&& digit.contains(middle)
				&& digit.contains(bottom_right)
				&& digit.contains(bottom)
			{
				Some(5)
			} else {
				None
			}
		}
		6 => {
			if digit.contains(top)
				&& digit.contains(top_left)
				&& digit.contains(top_right)
				&& digit.contains(bottom_left)
				&& digit.contains(bottom_right)
				&& digit.contains(bottom)
			{
				Some(0)
			} else if digit.contains(top)
				&& digit.contains(top_left)
				&& digit.contains(middle)
				&& digit.contains(bottom_left)
				&& digit.contains(bottom_right)
				&& digit.contains(bottom)
			{
				Some(6)
			} else if digit.contains(top)
				&& digit.contains(top_left)
				&& digit.contains(top_right)
				&& digit.contains(middle)
				&& digit.contains(bottom_right)
				&& digit.contains(bottom)
			{
				Some(9)
			} else {
				None
			}
		}
		_ => None,
	}
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
