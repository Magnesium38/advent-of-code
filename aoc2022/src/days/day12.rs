use advent::Grid;
use anyhow::anyhow;
use hashbrown::HashSet;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Debug)]
enum Node {
	Start(usize),
	End(usize),
	Regular(usize),
}

fn find_lowest_steps(grid: &Grid<Node>, start: (isize, isize)) -> usize {
	let mut priority_queue = BinaryHeap::new();
	let mut visited = HashSet::new();

	priority_queue.push((Reverse(0), 0, start.0, start.1));

	while let Some((Reverse(cost), elevation, x, y)) = priority_queue.pop() {
		for (dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
			let (x, y) = (x + dx, y + dy);

			if let Some(node) = grid.get(x, y) {
				match node {
					Node::End(tile_elevation) => {
						if elevation >= tile_elevation - 1 {
							return cost + 1;
						}
					}
					&Node::Regular(tile_elevation) | &Node::Start(tile_elevation) => {
						if tile_elevation > elevation + 1 {
							continue;
						}

						if !visited.insert((x, y)) {
							continue;
						}

						priority_queue.push((Reverse(cost + 1), tile_elevation, x, y));
					}
				}
			}
		}
	}

	usize::MAX
}

pub fn pt1(input: &str) -> anyhow::Result<usize> {
	let grid = Grid::from(input.lines().map(|line| {
		line.as_bytes().iter().map(|c| match c {
			b'E' => Node::End((b'z' - b'a') as usize),
			b'S' => Node::Start((b'a' - b'a') as usize),
			_ => Node::Regular((c - b'a') as usize),
		})
	}));

	let start = grid
		.iter()
		.find(|(_, node)| matches!(node, Node::Start(_)))
		.unwrap()
		.0;

	Ok(find_lowest_steps(&grid, start))
}

pub fn pt2(input: &str) -> anyhow::Result<usize> {
	let grid = Grid::from(input.lines().map(|line| {
		line.as_bytes().iter().map(|c| match c {
			b'E' => Node::End((b'z' - b'a') as usize),
			b'S' => Node::Start((b'a' - b'a') as usize),
			_ => Node::Regular((c - b'a') as usize),
		})
	}));

	grid.iter()
		.filter(|(_, node)| {
			matches!(node, Node::Start(_))
				|| matches!(node, Node::Regular(elevation) if *elevation == 0)
		})
		.map(|(start, _)| find_lowest_steps(&grid, start))
		.min()
		.ok_or(anyhow!("no elements found"))
}

advent::problem!(
	r#"
		Sabqponm
		abcryxxl
		accszExk
		acctuvwj
		abdefghi
    "#,
	31,
	29,
);
