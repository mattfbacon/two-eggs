#![deny(
	absolute_paths_not_starting_with_crate,
	keyword_idents,
	macro_use_extern_crate,
	meta_variable_misuse,
	missing_abi,
	missing_copy_implementations,
	non_ascii_idents,
	nonstandard_style,
	noop_method_call,
	pointer_structural_match,
	private_in_public,
	rust_2018_idioms,
	unused_qualifications
)]
#![warn(clippy::pedantic)]
#![forbid(unsafe_code)]

use std::ops::Range;

const FLOORS: Range<u32> = 0..100;

struct Tester {
	first_unsafe_floor: u32,
	num_tries: u32,
	eggs_left: u32,
}

impl Tester {
	fn new(first_unsafe_floor: u32) -> Self {
		Self {
			first_unsafe_floor,
			num_tries: 0,
			eggs_left: 2,
		}
	}

	/// Returns whether the egg broke.
	fn breaks_from(&mut self, floor: u32) -> bool {
		assert!(self.eggs_left > 0, "no eggs left");

		self.num_tries += 1;

		let breaks = floor >= self.first_unsafe_floor;
		if breaks {
			self.eggs_left -= 1;
		}
		breaks
	}
}

trait Strategy: std::fmt::Debug {
	/// Returns `tester.n` as determined by dropping eggs.
	fn solve(&self, tester: &mut Tester) -> u32;
}

#[derive(Debug)]
struct Linear;

impl Strategy for Linear {
	fn solve(&self, tester: &mut Tester) -> u32 {
		FLOORS
			.clone()
			.find(|&floor| tester.breaks_from(floor))
			.unwrap_or(FLOORS.end)
	}
}

#[derive(Debug)]
struct Chunks {
	size: u32,
}

fn two_stage(mut first_stage: impl Iterator<Item = u32>, tester: &mut Tester) -> u32 {
	let mut last = None;
	let broke_at = first_stage.find(|&floor| {
		let breaks = tester.breaks_from(floor);
		if !breaks {
			last = Some(floor);
		}
		breaks
	});

	let min = last.map_or(0, |last| last + 1);
	let max = broke_at.unwrap_or(FLOORS.end);

	(min..max)
		.find(|&floor| tester.breaks_from(floor))
		.unwrap_or(max)
}

impl Strategy for Chunks {
	fn solve(&self, tester: &mut Tester) -> u32 {
		two_stage(
			FLOORS
				.clone()
				.step_by(self.size as usize)
				.map(|floor| floor + (self.size - 1)),
			tester,
		)
	}
}

#[derive(Debug)]
struct ShrinkingChunks {
	first_size: u32,
	shrink_by: u32,
}

impl Strategy for ShrinkingChunks {
	fn solve(&self, tester: &mut Tester) -> u32 {
		let first_stage = std::iter::from_fn({
			let mut floor = 0;
			let mut step = self.first_size;
			move || {
				if step == 0 {
					return None;
				}
				floor += step;
				step -= self.shrink_by;
				Some(floor)
			}
		})
		.take_while(|floor| FLOORS.contains(floor));
		two_stage(first_stage, tester)
	}
}

#[derive(Debug, PartialEq, Eq)]
struct Case {
	floor: u32,
	num_tries: u32,
}

fn find_worst_case(strategy: &dyn Strategy) -> Case {
	// Include the situation where the egg is safe at all floors.
	let range = FLOORS.start..=FLOORS.end;
	range
		.clone()
		.map(|floor| {
			let mut tester = Tester::new(floor);
			let guessed = strategy.solve(&mut tester);
			assert_eq!(
				guessed, floor,
				"strategy {strategy:?}'s guess was not correct"
			);
			Case {
				floor,
				num_tries: tester.num_tries,
			}
		})
		.max_by_key(|case| case.num_tries)
		.unwrap()
}

#[test]
fn maintain_same() {
	assert_eq!(
		find_worst_case(&Linear),
		Case {
			floor: 100,
			num_tries: 100,
		},
	);
	assert_eq!(
		find_worst_case(&Chunks { size: 10 }),
		Case {
			floor: 99,
			num_tries: 19,
		},
	);
	assert_eq!(
		find_worst_case(&ShrinkingChunks {
			first_size: 14,
			shrink_by: 1
		}),
		Case {
			floor: 14,
			num_tries: 15,
		},
	);
}

fn main() {
	dbg!(find_worst_case(&Linear));
	dbg!(find_worst_case(&Chunks { size: 10 }));
	dbg!(find_worst_case(&ShrinkingChunks {
		first_size: 14,
		shrink_by: 1,
	}));
}
