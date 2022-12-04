// https://adventofcode.com/2022/day/3
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Clone)]
struct Item {
    name: char,
    priority: u8,
}

impl Item {
    fn new(name: char, priority: u8) -> Self {
        Self { name, priority }
    }

    fn of(item_char: char) -> Self {
        let priority: u8 = if item_char.is_lowercase() {
            (item_char as u8) - 96
        } else {
            (item_char as u8) - 65 + 27
        };

        Self::new(item_char, priority)
    }
}

#[derive(Debug, Clone)]
struct Compartment {
    content: Vec<Item>,
}

impl Compartment {
    fn new(content: Vec<Item>) -> Self {
        Self { content: content }
    }

    fn of(items: &str) -> Self {
        let mut content: Vec<Item> = Vec::with_capacity(items.len());
        for item_char in items.chars() {
            content.push(Item::of(item_char));
        }

        Self::new(content)
    }
}

#[derive(Debug, Clone)]
struct Rucksack {
    compartments: (Compartment, Compartment),
}

impl Rucksack {
    fn new(compartments: (Compartment, Compartment)) -> Self {
        Self { compartments }
    }

    fn of(content: &str) -> Self {
        let (content1, content2) = Self::split_string_in_half(content);
        Self::new((Compartment::of(content1), Compartment::of(content2)))
    }

    fn split_string_in_half(input: &str) -> (&str, &str) {
        let mid = input.len() / 2;
        let (left, right) = input.split_at(mid);
        (left, right)
    }

    fn items_in_both_compartments(&self) -> Vec<u32> {
        let mut duplicate_priorities: HashSet<u32> = HashSet::new();
        let mut dupe_find_helper: HashMap<u8, u8> = HashMap::new();

        for item in &self.compartments.0.content {
            dupe_find_helper.insert(item.priority, 1);
        }

        for item in &self.compartments.1.content {
            if dupe_find_helper.contains_key(&item.priority) {
                duplicate_priorities.insert(item.priority as u32);
            }
        }

        duplicate_priorities.into_iter().collect()
    }
}

struct Group {
    rucksacks: (Rucksack, Rucksack, Rucksack),
    group_badge: u32,
}

impl Group {
    fn new(rucksacks: (Rucksack, Rucksack, Rucksack), group_badge: u32) -> Self {
        Self { rucksacks, group_badge }
    }

    fn of(content: Vec<&str>) -> Self {
        let mut rucksacks: Vec<Rucksack> = Vec::with_capacity(3);
        let content2 = content.clone();
        let group_badge: u32 = Self::find_group_badge(content);

        for rucksack_content in content2 {
            rucksacks.push(Rucksack::of(rucksack_content));
        }

        Self::new(
            (rucksacks[0].clone(), rucksacks[1].clone(), rucksacks[2].clone()),
            group_badge,
        )
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