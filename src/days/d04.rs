// https://adventofcode.com/2022/day/4
use std::ops::Range;

struct Assignment {
    range1: Range<u32>,
    range2: Range<u32>,
}

impl Assignment {
    fn new(range1: Range<u32>, range2: Range<u32>) -> Self {
        Self { range1, range2 }
    }

    fn of(assignment: &str) -> Self {
        let split_vec = assignment.split(",").collect::<Vec<_>>();
        let range1 = Self::parse_range(split_vec[0]);
        let range2 = Self::parse_range(split_vec[1]);
        Self::new(range1, range2)
    }

    fn parse_range(range: &str) -> Range<u32> {
        let a: Vec<u32> = range
            .split("-")
            .map(|s| s.parse().unwrap())
            .collect();
        Range { start: a[0], end: a[1] }
    }

    fn are_ranges_fully_overlapping(&self) -> bool {
        (self.range1.start <= self.range2.start && self.range1.end >= self.range2.end) ||
            (self.range2.start <= self.range1.start && self.range2.end >= self.range1.end)
    }

    fn are_ranges_partially_overlapping(&self) -> bool {
        (self.range1.start <= self.range2.start && self.range1.end >= self.range2.start) ||
            (self.range2.start <= self.range1.start && self.range2.end >= self.range1.start)
    }
}

pub fn solve(_input: String) -> (String, String) {
    let result1: u32 = _input
        .split("\r\n")
        .map(|line| Assignment::of(line))
        .filter(|assignment| assignment.are_ranges_fully_overlapping())
        .collect::<Vec<_>>()
        .iter()
        .count() as u32;

    let result2: u32 = _input
        .split("\r\n")
        .map(|line| Assignment::of(line))
        .filter(|assignment| assignment.are_ranges_partially_overlapping())
        .collect::<Vec<_>>()
        .iter()
        .count() as u32;

    (result1.to_string(), result2.to_string())
}