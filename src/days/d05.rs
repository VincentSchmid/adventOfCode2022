// https://adventofcode.com/2022/day/5
use std::ops::Range;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
struct Stack {
    content: VecDeque<char>,
}

impl Stack {
    fn new() -> Self {
        Self { content: VecDeque::new() }
    }

    fn push(&mut self, item: char) {
        self.content.push_front(item);
    }

    fn push_front_multiple(&mut self, items: Vec<char>) {
        items
            .iter()
            .rev()
            .for_each(|item| self.push(*item));
    }

    fn pop(&mut self) -> Option<char> {
        self.content.pop_front()
    }

    fn pop_multiple(&mut self, count: u8) -> Vec<char> {
        (Range { start: 0, end: count }).map(|_| self.pop().unwrap()).collect::<Vec<char>>()
    }

    fn peek(&self) -> Option<&char> {
        self.content.front()
    }
}

#[derive(Debug)]
struct Instruction {
    count: u8,
    from: usize,
    to: usize,
}

impl Instruction {
    fn new(count: u8, from: usize, to: usize) -> Self {
        Self { count, from, to }
    }

    fn of(input: Vec<u8>) -> Self {
        Self::new(input[0], input[1].into(), input[2].into())
    }

    fn execute(&self, stackys: &Vec<Stack>) -> Vec<Stack> {
        let mut stacks = stackys.clone();
        let mut from = stacks[self.from - 1].clone();
        let mut to = stacks[self.to - 1].clone();

        (Range { start: 0, end: self.count }).for_each(|_| {
            let item = from.pop().unwrap();
            to.push(item);
        });

        stacks[self.from - 1] = from;
        stacks[self.to - 1] = to;
        stacks
    }

    fn execute_multiple(&self, stackys: &Vec<Stack>) -> Vec<Stack> {
        let mut stacks = stackys.clone();
        let mut from = stacks[self.from - 1].clone();
        let mut to = stacks[self.to - 1].clone();

        let items = from.pop_multiple(self.count);
        to.push_front_multiple(items);

        stacks[self.from - 1] = from;
        stacks[self.to - 1] = to;
        stacks
    }
}

pub fn solve(_input: String) -> (String, String) {
    let graphic = split_graphic(&_input).split("\r\n").collect::<Vec<_>>();
    let stacks = build_stacks(graphic);
    let instructions = get_instructions(&_input);
    let result1: String = instructions
        .iter()
        .fold(stacks, |stacks, instruction| instruction.execute(&stacks))
        .iter()
        .map(|stack| stack.peek())
        .filter(|c| c.is_some())
        .map(|c| c.unwrap())
        .collect::<String>();

    let graphic = split_graphic(&_input).split("\r\n").collect::<Vec<_>>();
    let stacks = build_stacks(graphic);
    let result2: String = instructions
        .iter()
        .fold(stacks, |stacks, instruction| instruction.execute_multiple(&stacks))
        .iter()
        .map(|stack| stack.peek())
        .filter(|c| c.is_some())
        .map(|c| c.unwrap())
        .collect::<String>();

    (result1, result2)
}

fn get_instructions(input: &str) -> Vec<Instruction> {
    input
        .split("\r\n\r\n")
        .collect::<Vec<_>>()[1]
        .split("\r\n")
        .map(|s|
            s
                .split(" ")
                .filter(|s| s.chars().nth(0).unwrap().is_numeric())
                .map(|s| s.parse::<u8>().unwrap())
                .collect::<Vec<_>>()
        )
        .map(|v| Instruction::of(v))
        .collect::<Vec<Instruction>>()
}

fn split_graphic(input: &str) -> &str {
    input.split("\r\n\r\n").collect::<Vec<_>>()[0]
}

fn build_stacks(graphic: Vec<&str>) -> Vec<Stack> {
    let r: Range<u8> = Range { start: 1, end: graphic[0].len() as u8 };
    r.filter(|i|
        graphic
            .last()
            .unwrap()
            .chars()
            .nth(*i as usize)
            .unwrap()
            .is_alphanumeric()
    )
        .map(|i| {
            let mut stack = Stack::new();
            graphic
                .iter()
                .rev()
                .for_each(|line| {
                    let c = line
                        .chars()
                        .nth(i as usize)
                        .unwrap();
                    if c.is_alphabetic() {
                        stack.push(c);
                    }
                });
            stack
        })
        .collect::<Vec<Stack>>()
}
