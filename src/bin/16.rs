use itertools::Itertools;
use std::str::Chars;

fn pt1(input: &str) -> anyhow::Result<usize> {
	Ok(parse_packet(&mut input.chars().map(hex_to_binary).join("").chars())?.sum_versions())
}

fn pt2(input: &str) -> anyhow::Result<usize> {
	Ok(parse_packet(&mut input.chars().map(hex_to_binary).join("").chars())?.value())
}

fn hex_to_binary(c: char) -> &'static str {
	match c {
		'0' => "0000",
		'1' => "0001",
		'2' => "0010",
		'3' => "0011",
		'4' => "0100",
		'5' => "0101",
		'6' => "0110",
		'7' => "0111",
		'8' => "1000",
		'9' => "1001",
		'A' => "1010",
		'B' => "1011",
		'C' => "1100",
		'D' => "1101",
		'E' => "1110",
		'F' => "1111",
		_ => panic!("Invalid character"),
	}
}

fn take(input: &mut Chars, n: usize) -> String {
	input.take(n).collect()
}

struct Packet {
	version: u8,
	type_id: u8,
	subpackets: Option<Vec<Packet>>,
	literal: Option<usize>,
}

impl Packet {
	fn sum_versions(&self) -> usize {
		let count = match self.subpackets {
			Some(ref subpackets) => subpackets.iter().map(|p| p.sum_versions()).sum(),
			None => 0,
		};

		count + self.version as usize
	}

	fn value(&self) -> usize {
		match self.type_id {
			4 => self.literal.unwrap(),

			0 => self
				.subpackets
				.as_ref()
				.unwrap()
				.iter()
				.map(|p| p.value())
				.sum(),
			1 => self
				.subpackets
				.as_ref()
				.unwrap()
				.iter()
				.map(|p| p.value())
				.product(),

			2 => self
				.subpackets
				.as_ref()
				.unwrap()
				.iter()
				.map(|p| p.value())
				.min()
				.unwrap(),
			3 => self
				.subpackets
				.as_ref()
				.unwrap()
				.iter()
				.map(|p| p.value())
				.max()
				.unwrap(),

			5 | 6 | 7 => {
				let (a, b) = self
					.subpackets
					.as_ref()
					.unwrap()
					.iter()
					.map(|p| p.value())
					.collect_tuple()
					.unwrap();

				if match self.type_id {
					5 => a > b,
					6 => a < b,
					7 => a == b,
					_ => unreachable!(),
				} {
					1
				} else {
					0
				}
			}

			_ => unreachable!(),
		}
	}
}

fn parse_packet(mut input: &mut Chars) -> anyhow::Result<Packet> {
	let version = u8::from_str_radix(&take(&mut input, 3), 2)?;
	let type_id = u8::from_str_radix(&take(&mut input, 3), 2)?;
	let mut subpackets = None;
	let mut literal = None;

	if type_id == 4 {
		literal = Some(read_literal(&mut input));

		return Ok(Packet {
			version,
			type_id,
			subpackets,
			literal,
		});
	} else {
		let length_type_id = take(&mut input, 1);
		let mut packets = Vec::new();

		if length_type_id == "0" {
			let total_length = usize::from_str_radix(&take(&mut input, 15), 2)?;
			let subpackets: String = input.take(total_length).collect();
			let mut input = subpackets.chars();
			while let Ok(packet) = parse_packet(&mut input) {
				packets.push(packet);
			}
		} else {
			let mut subpacket_count = usize::from_str_radix(&take(&mut input, 11), 2)?;
			while subpacket_count > 0 {
				let subpacket = parse_packet(&mut input)?;
				packets.push(subpacket);
				subpacket_count -= 1;
			}
		}

		subpackets = Some(packets);
	}

	Ok(Packet {
		version,
		type_id,
		subpackets,
		literal,
	})
}

fn read_literal(mut input: &mut Chars) -> usize {
	let mut literal = String::new();

	loop {
		let is_last_group = take(&mut input, 1) == "0";

		literal.push_str(&take(&mut input, 4));

		if is_last_group {
			break;
		}
	}

	usize::from_str_radix(&literal, 2).unwrap()
}

advent::problem!(
	r#"
		A0016C880162017C3686B18A3D4780
	"#,
	31,
	54,
);

#[cfg(test)]
mod addtional_tests {
	use super::*;

	#[test]
	fn part_one() {
		for (input, expected) in vec![
			("D2FE28", 6),
			("38006F45291200", 9),
			("EE00D40C823060", 14),
			("8A004A801A8002F478", 16),
			("620080001611562C8802118E34", 12),
			("C0015000016115A2E0802F182340", 23),
		] {
			assert_eq!(pt1(input).unwrap(), expected);
		}
	}

	#[test]
	fn part_two() {
		for (input, expected) in vec![
			("C200B40A82", 3),
			("04005AC33890", 54),
			("880086C3E88112", 7),
			("CE00C43D881120", 9),
			("D8005AC2A8F0", 1),
			("F600BC2D8F", 0),
			("9C005AC2F8F0", 0),
			("9C0141080250320F1802104A08", 1),
		] {
			assert_eq!(pt2(input).unwrap(), expected);
		}
	}
}
