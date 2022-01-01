use md5::compute;

pub fn pt1(input: &str) -> anyhow::Result<isize> {
	find_prefix(input, "00000")
}

pub fn pt2(input: &str) -> anyhow::Result<isize> {
	find_prefix(input, "000000")
}

fn find_prefix(input: &str, prefix: &str) -> anyhow::Result<isize> {
	let bytes = (0..prefix.len() / 2 * 2)
		.step_by(2)
		.map(|i| u8::from_str_radix(&prefix[i..i + 2], 16))
		.collect::<Result<Vec<_>, _>>()?;

	for n in 0.. {
		let hash = compute(format!("{}{}", input, n));

		if bytes.iter().zip(hash.iter()).all(|(b, h)| b == h)
			&& format!("{:x}", hash).starts_with(prefix)
		{
			return Ok(n);
		}
	}

	unreachable!()
}

advent::expensive_problem! {
	r#"
		abcdef
	"#,
	609043,
	6742839,
}
