use std::path::PathBuf;

use hashbrown::HashMap;
use itertools::Itertools;

#[derive(Debug, Clone)]
struct Directory {
	path: PathBuf,
	files: HashMap<PathBuf, usize>,
	directories: Vec<PathBuf>,
}

impl Directory {
	fn new(path: PathBuf) -> Self {
		Directory {
			path,
			files: HashMap::new(),
			directories: Vec::new(),
		}
	}

	fn get_size(&self, fs: &FileSystem) -> usize {
		self.files.iter().map(|(_, size)| size).sum::<usize>()
			+ self
				.directories
				.iter()
				.map(|path| {
					// dbg!(&path);

					fs.directories
						.get(&self.path.with_file_name(path))
						.unwrap()
						.get_size(fs)
				})
				.sum::<usize>()
	}
}

#[derive(Debug, Clone)]
struct FileSystem {
	directories: HashMap<PathBuf, Directory>,
	working_directory: PathBuf,
}

impl FileSystem {
	fn new() -> Self {
		FileSystem {
			directories: HashMap::new(),
			working_directory: PathBuf::new(),
		}
	}
}

pub fn pt1(input: &str) -> anyhow::Result<usize> {
	let fs = input.lines().fold(FileSystem::new(), |mut fs, line| {
		if line.starts_with('$') {
			if line.starts_with("$ ls") {
				// Nothing
			} else if line == "$ cd .." {
				fs.working_directory.pop();
			} else if line == "$ cd /" {
				fs.working_directory = PathBuf::from("/");
			} else {
				fs.working_directory = fs.working_directory.join(&line[5..]);
			}
		} else {
			if line.starts_with("dir") {
				dbg!(&fs.working_directory.clone().join(&line[4..]));
				fs.directories
					.entry(fs.working_directory.clone())
					.or_insert(Directory::new(fs.working_directory.clone()))
					.directories
					.push(fs.working_directory.clone().join(&line[4..]));
			} else {
				let (size, file) = line.split_once(' ').unwrap();

				fs.directories
					.entry(fs.working_directory.clone())
					.or_insert(Directory::new(fs.working_directory.clone()))
					.files
					.insert(
						fs.working_directory.with_file_name(file),
						size.parse().unwrap(),
					);
			}
		}

		fs
	});

	// dbg!(&fs.directories);

	Ok(fs
		.directories
		.iter()
		.map(|(_, directory)| directory.get_size(&fs))
		.filter(|size| *size < 100000)
		.sum())
}

pub fn pt2(input: &str) -> anyhow::Result<usize> {
	let fs = input.lines().fold(FileSystem::new(), |mut fs, line| {
		if line.starts_with('$') {
			if line.starts_with("$ ls") {
				// Nothing
			} else if line == "$ cd .." {
				fs.working_directory.pop();
			} else if line == "$ cd /" {
				fs.working_directory = PathBuf::from("/");
			} else {
				fs.working_directory = fs.working_directory.join(&line[5..]);
			}
		} else {
			if line.starts_with("dir") {
				dbg!(&fs.working_directory.clone().join(&line[4..]));
				fs.directories
					.entry(fs.working_directory.clone())
					.or_insert(Directory::new(fs.working_directory.clone()))
					.directories
					.push(fs.working_directory.clone().join(&line[4..]));
			} else {
				let (size, file) = line.split_once(' ').unwrap();

				fs.directories
					.entry(fs.working_directory.clone())
					.or_insert(Directory::new(fs.working_directory.clone()))
					.files
					.insert(
						fs.working_directory.with_file_name(file),
						size.parse().unwrap(),
					);
			}
		}

		fs
	});

	let used_space = fs
		.directories
		.get(&PathBuf::from("/"))
		.unwrap()
		.get_size(&fs);

	let minimum = 30000000 - (70000000 - used_space);

	Ok(fs
		.directories
		.iter()
		.map(|(_, directory)| directory.get_size(&fs))
		.sorted()
		.filter(|size| *size > minimum)
		.next()
		.unwrap())
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
