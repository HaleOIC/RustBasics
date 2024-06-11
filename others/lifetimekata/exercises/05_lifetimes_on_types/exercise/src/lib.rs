use std::collections::HashSet;

#[derive(Debug, Default)]
pub struct Difference<'first, 'second> {
    first_only: Vec<&'first str>,
    second_only: Vec<&'second str>,
}

pub fn find_difference<'s1, 's2>(sentence1: &'s1 str, sentence2: &'s2 str) -> Difference<'s1, 's2> {
    let sentence_1_words: HashSet<&str> = sentence1.split(" ").collect();
    let sentence_2_words: HashSet<&str> = sentence2.split(" ").collect();

    let mut diff = Difference::default();

    for word in &sentence_1_words {
        if !sentence_2_words.contains(word) {
            diff.first_only.push(word)
        }
    }

    for word in &sentence_2_words {
        if !sentence_1_words.contains(word) {
            diff.second_only.push(word)
        }
    }

    diff.first_only.sort();
    diff.second_only.sort();

    diff
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main() {
        let first_sentence = String::from("I hate the surf and the sand.");
        let second_sentence = String::from("I love the surf and the sand.");

        let first_only = {
            let third_sentence = String::from("I love the snow and the sand.");
            let diff = find_difference(&first_sentence, &third_sentence);
            diff.first_only
        };

        assert_eq!(first_only, vec!["hate", "surf"]);

        let second_only = {
            let third_sentence = String::from("I love the snow and the sand.");
            let diff = find_difference(&third_sentence, &second_sentence);
            diff.second_only
        };

        assert_eq!(second_only, vec!["surf"]);
    }
}
