use std::ops::{Add, Sub};

use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

pub fn pt1(input: &str) -> anyhow::Result<usize> {
	solve(input).map(|(count, _)| count)
}

pub fn pt2(input: &str) -> anyhow::Result<isize> {
	solve(input).map(|(_, max)| max)
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Vector3<T> {
	x: T,
	y: T,
	z: T,
}

impl<T: std::fmt::Debug> std::fmt::Debug for Vector3<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "({:?}, {:?}, {:?})", self.x, self.y, self.z)
	}
}

impl<T> Vector3<T> {
	fn new(x: T, y: T, z: T) -> Self {
		Self { x, y, z }
	}
}

impl<T: Add<Output = T>> Add for Vector3<T> {
	type Output = Self;

	fn add(self, other: Self) -> Self::Output {
		Self {
			x: self.x + other.x,
			y: self.y + other.y,
			z: self.z + other.z,
		}
	}
}

impl<T: Sub<Output = T>> Sub for Vector3<T> {
	type Output = Self;

	fn sub(self, other: Self) -> Self::Output {
		Self {
			x: self.x - other.x,
			y: self.y - other.y,
			z: self.z - other.z,
		}
	}
}

impl<'a, T: Add<Output = T> + Copy> Add for &'a Vector3<T> {
	type Output = Vector3<T>;

	fn add(self, other: Self) -> Self::Output {
		Vector3::new(self.x + other.x, self.y + other.y, self.z + other.z)
	}
}

impl<'a, T: Sub<Output = T> + Copy> Sub for &'a Vector3<T> {
	type Output = Vector3<T>;

	fn sub(self, other: Self) -> Self::Output {
		Vector3::new(self.x - other.x, self.y - other.y, self.z - other.z)
	}
}

type Point = Vector3<isize>;
type Fingerprint = (isize, isize);

fn solve(input: &str) -> anyhow::Result<(usize, isize)> {
	let mut map = HashSet::new();

	let mut scanners = input
		.split("\n\n")
		.map(|scanner| {
			scanner
				.lines()
				.skip(1)
				.map(|line| {
					let (x, y, z) = line.split(',').collect_tuple::<(_, _, _)>().unwrap();

					Vector3::new(
						x.trim().parse::<isize>().unwrap(),
						y.trim().parse::<isize>().unwrap(),
						z.trim().parse::<isize>().unwrap(),
					)
				})
				.collect_vec()
		})
		.map(|scanner| {
			let fingerprints = scanner
				.iter()
				.tuple_combinations::<(_, _)>()
				.map(|(p1, p2)| (fingerprint(p1, p2), (*p1, *p2)))
				.collect_vec();

			(scanner, fingerprints)
		})
		.collect_vec();

	let mut fingerprints: HashMap<Fingerprint, Vec<(Point, Point)>> = HashMap::new();

	let (scanner, scanner_fingerprints) = scanners.remove(0);
	scanner.iter().for_each(|&point| {
		map.insert(point);
	});

	scanner_fingerprints
		.iter()
		.for_each(|&(fingerprint, (p1, p2))| {
			fingerprints.entry(fingerprint).or_default().push((p1, p2));
		});

	let mut scanner_positions = Vec::with_capacity(scanners.len());
	scanner_positions.push(Vector3::new(0, 0, 0));

	while !scanners.is_empty() {
		let (scanner, scanner_fingerprints) = scanners.remove(0);

		if let Some((delta, reoriented_beacons)) =
			merge_scanner(&mut map, &fingerprints, &scanner, &scanner_fingerprints)
		{
			scanner_positions.push(delta);

			for (fingerprint, (p1, p2)) in reoriented_beacons
				.iter()
				.tuple_combinations::<(_, _)>()
				.map(|(p1, p2)| (fingerprint(&p1, &p2), (p1, p2)))
			{
				fingerprints
					.entry(fingerprint)
					.or_default()
					.push((*p1, *p2));
			}

			map.extend(reoriented_beacons);
		} else {
			scanners.push((scanner, scanner_fingerprints));
		}
	}

	scanner_positions
		.iter()
		.tuple_combinations()
		.map(
			|(
				Vector3 {
					x: x1,
					y: y1,
					z: z1,
				},
				Vector3 {
					x: x2,
					y: y2,
					z: z2,
				},
			)| (x1 - x2).abs() + (y1 - y2).abs() + (z1 - z2).abs(),
		)
		.max()
		.map(|max| (map.len(), max))
		.ok_or(anyhow::anyhow!("no solution found"))
}

fn fingerprint(p1: &Point, p2: &Point) -> Fingerprint {
	let delta = p2 - p1;

	(
		delta.x.pow(2) + delta.y.pow(2) + delta.z.pow(2),
		delta.x.abs().max(delta.y.abs().max(delta.z.abs())),
	)
}

fn merge_scanner(
	map: &mut HashSet<Point>,
	known_fingerprints: &HashMap<Fingerprint, Vec<(Point, Point)>>,
	scanner: &[Point],
	scanner_fingerprints: &[(Fingerprint, (Point, Point))],
) -> Option<(Vector3<isize>, Vec<Vector3<isize>>)> {
	let fns = [
		|Vector3 { x, y, z }: Point| Vector3::new(x, y, z),
		|Vector3 { x, y, z }: Point| Vector3::new(y, z, x),
		|Vector3 { x, y, z }: Point| Vector3::new(z, x, y),
		|Vector3 { x, y, z }: Point| Vector3::new(z, y, -x),
		|Vector3 { x, y, z }: Point| Vector3::new(y, x, -z),
		|Vector3 { x, y, z }: Point| Vector3::new(x, z, -y),
		|Vector3 { x, y, z }: Point| Vector3::new(x, -y, -z),
		|Vector3 { x, y, z }: Point| Vector3::new(y, -z, -x),
		|Vector3 { x, y, z }: Point| Vector3::new(z, -x, -y),
		|Vector3 { x, y, z }: Point| Vector3::new(z, -y, x),
		|Vector3 { x, y, z }: Point| Vector3::new(y, -x, z),
		|Vector3 { x, y, z }: Point| Vector3::new(x, -z, y),
		|Vector3 { x, y, z }: Point| Vector3::new(-x, y, -z),
		|Vector3 { x, y, z }: Point| Vector3::new(-y, z, -x),
		|Vector3 { x, y, z }: Point| Vector3::new(-z, x, -y),
		|Vector3 { x, y, z }: Point| Vector3::new(-z, y, x),
		|Vector3 { x, y, z }: Point| Vector3::new(-y, x, z),
		|Vector3 { x, y, z }: Point| Vector3::new(-x, z, y),
		|Vector3 { x, y, z }: Point| Vector3::new(-x, -y, z),
		|Vector3 { x, y, z }: Point| Vector3::new(-y, -z, x),
		|Vector3 { x, y, z }: Point| Vector3::new(-z, -x, y),
		|Vector3 { x, y, z }: Point| Vector3::new(-z, -y, -x),
		|Vector3 { x, y, z }: Point| Vector3::new(-y, -x, -z),
		|Vector3 { x, y, z }: Point| Vector3::new(-x, -z, -y),
	];

	let matching_fingerprints = scanner_fingerprints
		.iter()
		.filter(|(fingerprint, _)| known_fingerprints.contains_key(fingerprint))
		.collect_vec();

	if matching_fingerprints.len() < 66 {
		return None;
	}

	for (fingerprint, (bp1, bp2)) in matching_fingerprints {
		for (kp1, kp2) in known_fingerprints.get(fingerprint).unwrap() {
			for f in fns.iter().filter(|f| {
				let (tp1, tp2) = (f(*bp1), f(*bp2));

				kp1 - &tp1 == kp2 - &tp2
			}) {
				let delta = kp1 - &f(*bp1);

				let reoriented_scanner_points =
					scanner.iter().map(|&point| f(point) + delta).collect_vec();

				if reoriented_scanner_points
					.iter()
					.filter(|v| map.contains(v))
					.count() >= 12
				{
					return Some((delta, reoriented_scanner_points));
				}
			}
		}
	}

	None
}

advent::problem!(
	r#"
		--- scanner 0 ---
		404,-588,-901
		528,-643,409
		-838,591,734
		390,-675,-793
		-537,-823,-458
		-485,-357,347
		-345,-311,381
		-661,-816,-575
		-876,649,763
		-618,-824,-621
		553,345,-567
		474,580,667
		-447,-329,318
		-584,868,-557
		544,-627,-890
		564,392,-477
		455,729,728
		-892,524,684
		-689,845,-530
		423,-701,434
		7,-33,-71
		630,319,-379
		443,580,662
		-789,900,-551
		459,-707,401

		--- scanner 1 ---
		686,422,578
		605,423,415
		515,917,-361
		-336,658,858
		95,138,22
		-476,619,847
		-340,-569,-846
		567,-361,727
		-460,603,-452
		669,-402,600
		729,430,532
		-500,-761,534
		-322,571,750
		-466,-666,-811
		-429,-592,574
		-355,545,-477
		703,-491,-529
		-328,-685,520
		413,935,-424
		-391,539,-444
		586,-435,557
		-364,-763,-893
		807,-499,-711
		755,-354,-619
		553,889,-390

		--- scanner 2 ---
		649,640,665
		682,-795,504
		-784,533,-524
		-644,584,-595
		-588,-843,648
		-30,6,44
		-674,560,763
		500,723,-460
		609,671,-379
		-555,-800,653
		-675,-892,-343
		697,-426,-610
		578,704,681
		493,664,-388
		-671,-858,530
		-667,343,800
		571,-461,-707
		-138,-166,112
		-889,563,-600
		646,-828,498
		640,759,510
		-630,509,768
		-681,-892,-333
		673,-379,-804
		-742,-814,-386
		577,-820,562

		--- scanner 3 ---
		-589,542,597
		605,-692,669
		-500,565,-823
		-660,373,557
		-458,-679,-417
		-488,449,543
		-626,468,-788
		338,-750,-386
		528,-832,-391
		562,-778,733
		-938,-730,414
		543,643,-506
		-524,371,-870
		407,773,750
		-104,29,83
		378,-903,-323
		-778,-728,485
		426,699,580
		-438,-605,-362
		-469,-447,-387
		509,732,623
		647,635,-688
		-868,-804,481
		614,-800,639
		595,780,-596

		--- scanner 4 ---
		727,592,562
		-293,-554,779
		441,611,-461
		-714,465,-776
		-743,427,-804
		-660,-479,-426
		832,-632,460
		927,-485,-438
		408,393,-506
		466,436,-512
		110,16,151
		-258,-428,682
		-393,719,612
		-211,-452,876
		808,-476,-593
		-575,615,604
		-485,667,467
		-680,325,-822
		-627,-443,-432
		872,-547,-609
		833,512,582
		807,604,487
		839,-516,451
		891,-625,532
		-652,-548,-490
		30,-46,-14
	"#,
	79,
	3621,
);
