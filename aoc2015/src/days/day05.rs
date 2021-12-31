use fancy_regex::Regex;

pub fn pt1(input: &str) -> anyhow::Result<usize> {
	let has_three_vowels = Regex::new(r"[aeiou].*[aeiou].*[aeiou]")?;
	let has_double_letter = Regex::new(r"(\w)\1")?;
	let has_bad_strings = Regex::new(r"(ab|cd|pq|xy)")?;

	Ok(input
		.lines()
		.filter(|line| {
			has_three_vowels.is_match(line).unwrap_or(false)
				&& has_double_letter.is_match(line).unwrap_or(false)
				&& !has_bad_strings.is_match(line).unwrap_or(true)
		})
		.count())
}

pub fn pt2(input: &str) -> anyhow::Result<usize> {
	let has_pair_without_overlap = Regex::new(r"(..).*\1")?;
	let has_pair_with_middle = Regex::new(r"(.).\1")?;

	Ok(input
		.lines()
		.filter(|line| {
			has_pair_without_overlap.is_match(line).unwrap_or(false)
				&& has_pair_with_middle.is_match(line).unwrap_or(false)
		})
		.count())
}

advent::problem!(
	r#"
		ugknbfddgicrmopn
		aaa
		jchzalrnumimnmhp
		haegwjzuvuyypxyu
		dvszwmarrgswjxmb
		qjhvhtzxzqqjkmpb
		xxyxx
		uurcxstgmygtbstg
		ieodomkazucvgmuy
    "#,
	2,
	2,
);
