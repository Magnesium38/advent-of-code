use itertools::Itertools;

fn parse<'a>(input: &mut impl Iterator<Item = &'a str>) -> Vec<usize> {
	let mut sizes = vec![];
	let mut total = 0;

	loop {
		match input
			.next()
			.map(|line| line.split(' ').collect_vec())
			.as_deref()
		{
			Some(["$", "ls"]) | Some(["dir", _]) => (),
			Some(["$", "cd", ".."]) | None => break,
			Some(["$", "cd", _]) => {
				sizes.extend(parse(input));
				total += sizes
					.last()
					.expect("at least one element should have been added");
			}
			Some([size, _]) => total += size.parse::<usize>().expect("size is numeric"),
			_ => unreachable!(),
		}
	}

	sizes.push(total);
	sizes
}

pub fn pt1(input: &str) -> anyhow::Result<usize> {
	Ok(parse(&mut input.lines())
		.into_iter()
		.filter(|size| *size < 100000)
		.sum())
}

pub fn pt2(input: &str) -> anyhow::Result<usize> {
	let sizes = parse(&mut input.lines());
	let minimum = sizes
		.iter()
		.max()
		.expect("at least one element should exist")
		+ 30000000
		- 70000000;

	Ok(sizes
		.into_iter()
		.sorted()
		.find(|size| *size > minimum)
		.expect("at least one element should have been found"))
}

advent::problem!(
	r#"
		$ cd /
		$ ls
		dir a
		14848514 b.txt
		8504156 c.dat
		dir d
		$ cd a
		$ ls
		dir e
		29116 f
		2557 g
		62596 h.lst
		$ cd e
		$ ls
		584 i
		$ cd ..
		$ cd ..
		$ cd d
		$ ls
		4060174 j
		8033020 d.log
		5626152 d.ext
		7214296 k
    "#,
	95437,
	24933642,
);
