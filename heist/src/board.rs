use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::str::FromStr;
use std::sync::LazyLock;

static DICTIONARY: LazyLock<HashSet<String>> = LazyLock::new(|| {
    include_str!("./rsc/slowa.txt")
        .lines()
        .map(str::to_string)
        .collect::<HashSet<String>>()
});
const NEIGHBOURS: [(usize, usize); 8] = [
    (usize::MAX, usize::MAX),
    (usize::MAX, 0),
    (usize::MAX, 1),
    (0, usize::MAX),
    (0, 1),
    (1, usize::MAX),
    (1, 0),
    (1, 1),
];
const VISITED: char = '$';

#[derive(Debug, Serialize, Deserialize)]
pub struct Board {
    board: [[char; 4]; 4],
}

impl FromStr for Board {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut board = s.chars();

        let mut ans = [['0'; 4]; 4];
        for x in 0..4 {
            for y in 0..4 {
                ans[x][y] = board
                    .next()
                    .ok_or_else(|| "Not enough letters in the boggle")?;
            }
        }
        Ok(Board { board: ans })
    }
}

impl Board {
    pub fn good_words(&self) -> Vec<String> {
        let mut answer = vec![];
        fn rec(
            current: &mut Vec<char>,
            x: usize,
            y: usize,
            words: &mut Vec<String>,
            board: &mut [[char; 4]; 4],
        ) {
            current.push(board[x][y]);
            let curr_word = current.iter().collect::<String>();
            if DICTIONARY.contains(&curr_word) {
                words.push(curr_word);
            }
            let previous = board[x][y];
            board[x][y] = VISITED;

            NEIGHBOURS.into_iter().for_each(|(dx, dy)| {
                let new_x = x.wrapping_add(dx);
                let new_y = y.wrapping_add(dy);
                if new_x < 4 && new_y < 4 && board[new_x][new_y] != VISITED {
                    rec(current, new_x, new_y, words, board);
                }
            });
            board[x][y] = previous;
            current.pop();
        }
        for x in 0..4 {
            for y in 0..4 {
                let mut board = self.board;
                rec(&mut vec![], x, y, &mut answer, &mut board);
            }
        }
        answer.sort_unstable();
        answer.dedup();

        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_from_json() {
        let board = r#"{"board":[["a","b","c","d"],["a","b","c","d"],["a","b","c","d"],["a","b","c","d"]]}"#;
        let deserialized = serde_json::from_str::<Board>(board).unwrap();

        assert_eq!(deserialized.board[0][0], 'a');
    }

    #[test]
    fn good_words() {
        let board = r#"{"board":[["a","b","c","d"],["a","b","c","d"],["a","b","c","d"],["a","b","c","d"]]}"#;
        let deserialized = serde_json::from_str::<Board>(board).unwrap();

        let ans = deserialized.good_words();
        assert_eq!(ans, vec!["aa", "aaa", "abba", "ba", "bab", "baba"])
    }
}
