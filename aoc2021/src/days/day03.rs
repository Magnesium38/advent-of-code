use std::collections::HashMap;

pub fn pt1(input: &str) -> anyhow::Result<isize> {
	let (gamma, epsilon) = input
		.lines()
		.fold(vec![0; 16], |mut bit_count, binary| {
			for (i, bit) in binary.chars().rev().enumerate() {
				bit_count[i] += match bit {
					'0' => -1,
					'1' => 1,
					_ => 0,
				}
			}

			bit_count
		})
		.iter()
		.rev()
		.fold((0, 0), |(gamma, epsilon), count: &i32| {
			if count.is_positive() {
				((gamma << 1) + 1, epsilon << 1)
			} else if count.is_negative() {
				(gamma << 1, (epsilon << 1) + 1)
			} else {
				(gamma, epsilon)
			}
		});

	Ok(gamma * epsilon)
}

pub fn pt2(input: &str) -> anyhow::Result<isize> {
	let input = input.lines().collect::<Vec<_>>();

	let oxygen = {
		let mut input = input.clone();
		let mut bit_index = 0;
		while input.len() != 1 {
			let counted_bits = input
				.iter()
				.map(|binary| binary.chars().nth(bit_index).unwrap())
				.fold(HashMap::new(), |mut counted_bits, bit| {
					*counted_bits.entry(bit).or_insert(0) += 1;
					counted_bits
				});

			let (one_count, zero_count) = (
				counted_bits.get(&'1').copied().unwrap_or(0),
				counted_bits.get(&'0').copied().unwrap_or(0),
			);

			let most_frequent_bit = if one_count >= zero_count { '1' } else { '0' };

			input.retain(|binary| binary.chars().nth(bit_index).unwrap() == most_frequent_bit);
			bit_index += 1;
		}

		u32::from_str_radix(input.first().unwrap(), 2)?
	};

	let co2 = {
		let mut input = input;
		let mut bit_index = 0;
		while input.len() != 1 {
			let counted_bits = input
				.iter()
				.map(|binary| binary.chars().nth(bit_index).unwrap())
				.fold(HashMap::new(), |mut counted_bits, bit| {
					*counted_bits.entry(bit).or_insert(0) += 1;
					counted_bits
				});

			let (one_count, zero_count) = (
				counted_bits.get(&'1').copied().unwrap_or(0),
				counted_bits.get(&'0').copied().unwrap_or(0),
			);

			let least_frequent_bit = if one_count >= zero_count { '0' } else { '1' };

			input.retain(|binary| binary.chars().nth(bit_index).unwrap() == least_frequent_bit);
			bit_index += 1;
		}

		u32::from_str_radix(input.first().unwrap(), 2)?
	};

	Ok((oxygen * co2).try_into()?)
}

advent::problem!(
	r#"
        00100
        11110
        10110
        10111
        10101
        01111
        00111
        11100
        10000
        11001
        00010
        01010
    "#,
	198,
	230,
);
