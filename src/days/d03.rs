// https://adventofcode.com/2022/day/3
use std::collections::HashMap;


#[derive(Debug, Clone)]
struct Item {
    priority: u8,
}

impl Item {
    fn new(priority: u8) -> Self {
        Self { priority }
    }

    fn of(item_char: char) -> Self {
        let priority: u8 = if item_char.is_lowercase() {
            (item_char as u8) - 96
        } else {
            (item_char as u8) - 65 + 27
        };

        Self::new(priority)
    }
}

struct Group {
    group_badge: u32,
}

impl Group {
    fn new(group_badge: u32) -> Self {
        Self { group_badge }
    }

    fn of(content: Vec<&str>) -> Self {
        let group_badge: u32 = Self::find_group_badge(content);

        Self::new(group_badge)
    }

    fn find_group_badge(contents: Vec<&str>) -> u32 {
        let mut common_char: char = ' ';
        let mut dupe_find_helper: HashMap<char, u8> = HashMap::new();

        for item in contents[0].chars() {
            dupe_find_helper.insert(item, 1);
        }

        for item in contents[1].chars() {
            if dupe_find_helper.contains_key(&item) {
                dupe_find_helper.insert(item, 2);
            }
        }

        for item in contents[2].chars() {
            if dupe_find_helper.contains_key(&item) {
                if dupe_find_helper.get(&item) == Some(&2) {
                    common_char = item;
                    break;
                }
            }
        }

        Item::of(common_char).priority as u32
    }
}

pub fn solve(_input: String) -> (String, String) {
    let groups: Vec<Group> = _input
        .split("\r\n")
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|chunk| Group::of(chunk.to_vec()))
        .collect();

    let badge_sum = groups
        .iter()
        .map(|group| group.group_badge)
        .sum::<u32>();

    ("8176".into(), badge_sum.to_string())
}
