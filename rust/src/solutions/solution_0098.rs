use std::{
    cmp::max,
    collections::{BTreeMap, HashMap},
    fs,
};

use itertools::Itertools;

fn sort_digits(mut number: i64) -> String {
    let mut digits: Vec<String> = Vec::new();
    while number > 0 {
        digits.push((number % 10).to_string());
        number /= 10;
    }
    digits.sort();
    digits.join("")
}

fn split_on_change(s: &str) -> Vec<String> {
    s.chars().fold(Vec::new(), |mut acc, c| {
        match acc.last_mut() {
            Some(last) if last.starts_with(c) => last.push(c),
            _ => acc.push(String::from(c)),
        }
        acc
    })
}

fn normalize_mask(class: &str) -> String {
    split_on_change(class)
        .iter()
        .map(|x| x.len())
        .sorted()
        .map(|l| l.to_string())
        .join("-")
}

fn make_mapping(word: &str, number: &str) -> Option<BTreeMap<char, char>> {
    let mapping: BTreeMap<char, char> = word.chars().zip(number.to_string().chars()).collect();
    if mapping.len() == word.len() {
        Some(mapping)
    } else {
        None
    }
}

fn solution() -> i64 {
    let content = fs::read_to_string("../data/0098_words.txt").expect("File couldn't be found!");
    let row = content.trim();
    let json_string = format!("[{}]", row);
    let vec: Vec<String> = serde_json::from_str(&json_string).expect("Invalid JSON!");

    let mut anagram_classes: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for s in vec {
        let class: String = s.chars().sorted().collect();
        anagram_classes.entry(class).or_default().push(s);
    }
    anagram_classes.retain(|_class, strings| strings.len() > 1);
    let max_length = anagram_classes
        .keys()
        .map(|class| class.len())
        .max()
        .unwrap();

    let squares: Vec<i64> = (1_i64..)
        .map(|x| x.pow(2))
        .take_while(|&sq| sq < 10_i32.pow(max_length as u32).into())
        .collect();

    let mut squares_classes: HashMap<String, Vec<i64>> = HashMap::new();
    for square in squares {
        let sorted_digits = sort_digits(square);
        if sorted_digits[0..1] == *"0" {
            continue;
        }
        squares_classes
            .entry(sorted_digits)
            .or_default()
            .push(square);
    }
    squares_classes.retain(|_class, squares| squares.len() > 1);

    let mut squares_classes_2: HashMap<String, Vec<String>> = HashMap::new();
    for class in squares_classes.keys() {
        squares_classes_2
            .entry(normalize_mask(class))
            .or_default()
            .push(class.clone());
    }

    let mut biggest_prime: i64 = 0;
    for (word_class, words) in anagram_classes.iter() {
        for square_class in squares_classes_2
            .get(&normalize_mask(word_class))
            .into_iter()
            .flatten()
        {
            for (word_1, word_2) in words.iter().zip(words).filter(|(w1, w2)| w1 < w2) {
                for square_1 in squares_classes.get(square_class).unwrap().iter() {
                    let mapping = match make_mapping(word_1, &square_1.to_string()) {
                        Some(x) => x,
                        None => continue,
                    };

                    for square_2 in squares_classes
                        .get(square_class)
                        .unwrap()
                        .iter()
                        .filter(|s2| square_1 < s2)
                    {
                        if mapping == make_mapping(word_2, &square_2.to_string()).unwrap() {
                            // println!(
                            //     "| {} - {} | {} - {} | `{:?}` |",
                            //     word_1, word_2, square_1, square_2, mapping
                            // );
                            biggest_prime = max(biggest_prime, *square_2);
                        }
                    }
                }
            }
        }
    }
    biggest_prime
}

inventory::submit! {
    crate::registry::SolutionEntry {
        id: 98,
        implementations: &[("", solution)],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_mask() {
        assert_eq!(normalize_mask("AADDFFY"), "aabbccd");
    }

    #[test]
    fn test_solution() {
        assert_eq!(solution(), 17_689);
    }
}
