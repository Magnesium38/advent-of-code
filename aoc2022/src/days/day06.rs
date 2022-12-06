use std::collections::HashSet;

use itertools::Itertools;

fn find_marker(input: &str, window_size: usize) -> usize {
	input
		.as_bytes()
		.windows(window_size)
		.map(|window| -> HashSet<_> { HashSet::from_iter(window) })
		.find_position(|set| set.len() == window_size)
		.map(|(position, _)| position + window_size)
		.unwrap()
}

pub fn pt1(input: &str) -> anyhow::Result<usize> {
	Ok(find_marker(input, 4))
}

pub fn pt2(input: &str) -> anyhow::Result<usize> {
	Ok(find_marker(input, 14))
}

advent::problem!(
	r#"
		mjqjpqmgbljsphdztnvjfqwrcgsmlb
    "#,
	7,
	19,
);

#[cfg(test)]
mod additional {
	use super::*;

	#[test]
	fn cases() {
		[
			("bvwbjplbgvbhsrlpgdmjqwftvncz", 5, 23),
			("nppdvjthqldpwncqszvftbrmjlhg", 6, 23),
			("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10, 29),
			("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11, 26),
		]
		.iter()
		.for_each(|(input, pt1_result, pt2_result)| {
			assert_eq!(&pt1(input).unwrap(), pt1_result);
			assert_eq!(&pt2(input).unwrap(), pt2_result);
		})
	}
}
