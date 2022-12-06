use itertools::Itertools;
use once_cell::sync::Lazy;
use std::{
    collections::{HashMap, HashSet},
    convert::Infallible,
    str::FromStr,
};

fn main() {
    let rucksacks = std::fs::read_to_string("input/day3.txt").unwrap();
    let priorities: u32 = rucksacks
        .lines()
        .map(|line| Rucksack::from_str(line).unwrap())
        // .map(|rucksack| rucksack.item_in_both_compartments().priority())
        .tuples()
        .map(|(rucksack1, rucksack2, rucksack3)| {
            group_badge(&rucksack1, &rucksack2, &rucksack3).priority()
        })
        .sum();

    println!("Total sum of priorities: {priorities}");
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Item {
    character: char,
}

impl From<char> for Item {
    fn from(c: char) -> Self {
        let character = match c {
            'a'..='z' | 'A'..='Z' => c,
            _ => panic!(""),
        };

        Item { character }
    }
}

impl Item {
    fn priority(&self) -> u32 {
        static ITEM_PRIORITY: Lazy<HashMap<char, u32>> = Lazy::new(|| {
            ('a'..='z')
                .chain('A'..='Z')
                .enumerate()
                .map(|(number, character)| (character, number as u32 + 1))
                .collect()
        });

        *ITEM_PRIORITY.get(&self.character).unwrap()
    }
}

#[derive(Debug)]
struct Rucksack {
    compartment1: HashSet<Item>,
    compartment2: HashSet<Item>,
}

impl FromStr for Rucksack {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        assert!(s.len() % 2 == 0);
        assert!(s.is_ascii()); // make sure the below slicing is sound

        let half = s.len() / 2;
        let compartment1 = s[..half].chars().map(Item::from).collect();
        let compartment2 = s[half..].chars().map(Item::from).collect();

        Ok(Rucksack {
            compartment1,
            compartment2,
        })
    }
}

impl Rucksack {
    #[allow(unused)]
    fn item_in_both_compartments(&self) -> Item {
        let intersection: Vec<_> = self.compartment1.intersection(&self.compartment2).collect();
        assert!(intersection.len() == 1);
        *intersection[0]
    }

    fn all_items(&self) -> HashSet<Item> {
        let mut items = self.compartment1.clone();
        items.extend(&self.compartment2);
        items
    }
}

fn group_badge(rucksack1: &Rucksack, rucksack2: &Rucksack, rucksack3: &Rucksack) -> Item {
    let rucksack1_items = rucksack1.all_items();
    let rucksack2_items = rucksack2.all_items();
    let rucksack3_items = rucksack3.all_items();

    let rucksack1_2_items: HashSet<_> = rucksack1_items
        .intersection(&rucksack2_items)
        .copied()
        .collect();
    let rucksack1_2_3_items: Vec<_> = rucksack1_2_items.intersection(&rucksack3_items).collect();

    assert!(rucksack1_2_3_items.len() == 1);
    *rucksack1_2_3_items[0]
}
