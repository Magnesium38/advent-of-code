use itertools::Itertools;
use nom::{
	branch::alt, bytes::complete::tag, character::complete::u8, combinator::map,
	multi::separated_list0, sequence::delimited, IResult,
};
use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
enum Packet {
	List(Vec<Packet>),
	Integer(u8),
}

impl Packet {
	fn new(input: &str) -> Self {
		parse_packet(input).expect("failed to parse packet").1
	}
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
		Some(self.cmp(other))
	}
}

impl From<u8> for Packet {
	fn from(n: u8) -> Self {
		Packet::List(vec![Packet::Integer(n)])
	}
}

impl Ord for Packet {
	fn cmp(&self, other: &Self) -> Ordering {
		match (self, other) {
			(Packet::List(lhs), Packet::List(rhs)) => lhs.cmp(rhs),
			(Packet::Integer(lhs), Packet::Integer(rhs)) => lhs.cmp(rhs),
			(Packet::List(_), Packet::Integer(rhs)) => self.cmp(&Packet::from(*rhs)),
			(Packet::Integer(lhs), Packet::List(_)) => Packet::from(*lhs).cmp(other),
		}
	}
}

pub fn pt1(input: &str) -> anyhow::Result<usize> {
	Ok(input
		.split("\n\n")
		.map(|pair| pair.split_once('\n').expect("expected two lines"))
		.map(|(first, second)| Packet::new(first) <= Packet::new(second))
		.enumerate()
		.filter(|(_, result)| *result)
		.map(|(i, _)| i + 1)
		.sum())
}

pub fn pt2(input: &str) -> anyhow::Result<usize> {
	Ok(input
		.split("\n\n")
		.map(|pair| pair.split_once('\n').expect("expected two lines"))
		.flat_map(|(first, second)| [Packet::new(first), Packet::new(second)])
		.interleave([Packet::Integer(2), Packet::Integer(6)])
		.sorted()
		.enumerate()
		.filter(|(_, packet)| [Packet::Integer(2), Packet::Integer(6)].contains(packet))
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
