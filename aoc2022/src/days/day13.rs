use itertools::Itertools;
use nom::{
	branch::alt, bytes::complete::tag, character::complete::u8, combinator::map,
	multi::separated_list0, sequence::delimited, IResult,
};
use std::cmp::Ordering;
use std::fmt::{Display, Error, Formatter};

#[derive(Debug, PartialEq, Eq, Clone)]
enum Packet {
	List(Vec<Packet>),
	Integer(u8),
}

fn parse_packet(input: &str) -> IResult<&str, Packet> {
	alt((
		map(u8, Packet::Integer),
		map(
			delimited(tag("["), separated_list0(tag(","), parse_packet), tag("]")),
			Packet::List,
		),
	))(input)
}

impl PartialOrd<Packet> for Packet {
	fn partial_cmp(&self, other: &Packet) -> Option<Ordering> {
		match (self, other) {
			(Packet::List(lhs), Packet::List(rhs)) => lhs.partial_cmp(rhs),
			(Packet::Integer(lhs), Packet::Integer(rhs)) => lhs.partial_cmp(rhs),
			(Packet::List(_), Packet::Integer(rhs)) => {
				self.partial_cmp(&Packet::List(vec![Packet::Integer(*rhs)]))
			}
			(Packet::Integer(lhs), Packet::List(_)) => {
				Packet::List(vec![Packet::Integer(*lhs)]).partial_cmp(other)
			}
		}
	}
}

impl Ord for Packet {
	fn cmp(&self, other: &Self) -> Ordering {
		self.partial_cmp(other).unwrap()
	}
}

impl Display for Packet {
	fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
		match self {
			Self::Integer(n) => write!(f, "{n}"),
			Self::List(list) => {
				write!(f, "[")?;

				for (i, p) in list.iter().enumerate() {
					write!(f, "{p}")?;

					if i != list.len() - 1 {
						write!(f, ",")?;
					}
				}

				write!(f, "]")
			}
		}
	}
}

pub fn pt1(input: &str) -> anyhow::Result<usize> {
	Ok(input
		.split("\n\n")
		.map(|pair| pair.split_once('\n').unwrap())
		.map(|(first, second)| parse_packet(first).unwrap() <= parse_packet(second).unwrap())
		.enumerate()
		.filter(|(_, result)| *result)
		.map(|(i, _)| i + 1)
		.sum())
}

pub fn pt2(input: &str) -> anyhow::Result<usize> {
	let divider_packets = [
		parse_packet("[[2]]").unwrap().1,
		parse_packet("[[6]]").unwrap().1,
	];

	Ok(input
		.split("\n\n")
		.map(|pair| pair.split_once('\n').unwrap())
		.flat_map(|(first, second)| [parse_packet(first).unwrap(), parse_packet(second).unwrap()])
		.map(|(_, packet)| packet)
		.interleave(divider_packets.clone())
		.sorted()
		.enumerate()
		.filter(|(_, packet)| divider_packets.contains(packet))
		.map(|(i, _)| i + 1)
		.product())
}

advent::problem!(
	r#"
		[1,1,3,1,1]
		[1,1,5,1,1]

		[[1],[2,3,4]]
		[[1],4]

		[9]
		[[8,7,6]]

		[[4,4],4,4]
		[[4,4],4,4,4]

		[7,7,7,7]
		[7,7,7]

		[]
		[3]

		[[[]]]
		[[]]

		[1,[2,[3,[4,[5,6,7]]]],8,9]
		[1,[2,[3,[4,[5,6,0]]]],8,9]
	"#,
	13,
	140,
);
