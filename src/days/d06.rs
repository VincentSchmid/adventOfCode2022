// https://adventofcode.com/2022/day/6
use std::collections::VecDeque;
use std::collections::HashSet;


pub fn solve(_input: String) -> (String, String) {
    let result1 = _input
        .split("\r\n")
        .map(|line| get_packet_marker(line))
        .sum::<usize>()
        .to_string();

    let result2 = _input
        .split("\r\n")
        .map(|line| get_start_message_marker(&line))
        .sum::<usize>()
        .to_string();

    (result1, result2)
}

struct StringBuffer<const N: usize> {
    content: VecDeque<char>,
}

impl<const N: usize> StringBuffer<N> {
    fn new() -> Self {
        Self { content: VecDeque::with_capacity(N) }
    }

    fn push(&mut self, item: char) {
        if self.content.len() == N {
            self.content.pop_back();
        }
        self.content.push_front(item);
    }

    fn filled_and_unique(&self) -> bool {
        let mut dupe_finder: HashSet<char> = HashSet::new();
        for c in self.content.iter() {
            dupe_finder.insert(*c);
        }

        dupe_finder.len() == N
    }
}

fn get_packet_marker(buffer: &str) -> usize {
    let index = get_first_index_after_unique_string::<4>(buffer);
    println!("{}: first marker after character {}", buffer, index);
    index
}

fn get_start_message_marker(buffer: &str) -> usize {
    let index = get_first_index_after_unique_string::<14>(buffer);
    println!("first marker after character {}", index);
    index
}

fn get_first_index_after_unique_string<const N: usize>(string_to_check: &str) -> usize {
    let mut index = 0;
    let mut packet_marker = StringBuffer::<N>::new();

    for (i, c) in string_to_check.chars().enumerate() {
        packet_marker.push(c);

        if packet_marker.filled_and_unique() {
            index = i + 1;
            break;
        }
    }
    index
}
