extern crate regex;

use std::collections::hash_set::HashSet;
use std::io;
use std::io::prelude::*;
use std::time::SystemTime;

use regex::Regex;


static WORDS_FILE: &'static str = include_str!("words.txt");


struct Board {
    letters: String,
    size: usize,
}


#[derive(PartialEq, Eq, Hash, Clone)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Clone)]
struct Path {
    points: Vec<Point>,
    string: String,
}


// Returns a HashSet containing words that (a) only contain the letters
//  in `board` [not count-sensitive] and (b) have lengths of between 4 and the total
//  number of letters in `board`

fn get_word_candidates(board: &Board) -> HashSet<String> {
    let mut words = HashSet::new();
    
    let valid_chars: HashSet<char> = board.letters.chars().collect();
    let mut letters = String::new();
    for c in valid_chars {
        letters.push(c);
    }

    let letter_validator = Regex::new(&format!("^[{}]{{4,{}}}$", letters, board.size.pow(2))).unwrap();

    for word in WORDS_FILE.to_lowercase().replace("qu", "_").split(",") {
        if letter_validator.is_match(word) {
            words.insert(word.to_string());
        }
    }
    words
}


// Returns a HashSet of all possible prefixes (ex: the prefixes of 'apple'
//  are 'a', 'ap', 'app', 'appl', and 'apple'). Allows paths to be abandoned
//  if they cannot possibly contain any words

fn get_prefixes(words: &HashSet<String>) -> HashSet<String> {
    let mut prefixes = HashSet::new();
    for word in words {
        for i in 0..word.len() {
            prefixes.insert(word[..i + 1].to_string());
        }
    }
    prefixes
}


// Prompts the user for input via `stdin` and returns the response

fn prompt(msg: String) -> String {
    print!("{}", msg);
    io::stdout().flush().expect("Error flushing stdout");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error reading line");
    input.trim().to_string()
}


// Returns a HashSet of all points adjacent to `point` (including diagonally)
//  that lie on the board

fn get_adjacent_points(point: &Point, size: &usize) -> HashSet<Point> {
    let mut points = HashSet::new();
    let ix = point.x as i8;
    let iy = point.y as i8;
    for x in ix - 1..ix + 2 {
        for y in iy - 1..iy + 2 {
            if x >= 0 && x < *size as i8 && y >= 0 && y < *size as i8 {
                points.insert(Point { x: x as usize, y: y as usize });
            }
        }
    }
    points.remove(point);
    points
}


// Returns the letter at `point` in `board`

fn get_letter(point: &Point, board: &Board) -> String {
    board.letters.chars().nth(point.y * board.size + point.x).unwrap().to_string()
}


// Takes `path` and continues the search by extending it along all unused
//  points adjacent to the last point in `path`

fn continue_path(path: Path, board: &Board, words: &HashSet<String>, prefixes: &HashSet<String>, results: &mut HashSet<String>) {
    for point in get_adjacent_points(path.points.last().unwrap(), &board.size) {
        if !path.clone().points.into_iter().any(|p| point == p) {
            let mut new_path = path.clone();
            new_path.string += &get_letter(&point, board);
            if prefixes.contains(&new_path.string) {
                new_path.points.push(point);
                if words.contains(&new_path.string) {
                    results.insert(new_path.string.clone());
                }
                continue_path(new_path, board, words, prefixes, results);
            }
        }
    }
}


fn main() {
    println!("Enter the letters of the board, row by row, without spaces (for \"Qu\" enter just \"Q\":");
    let mut board = Board { letters: String::new(), size: 0 };

    // Get first row, which determines size
    board.letters = prompt("R1: ".to_string()).to_lowercase().replace("q", "_");
    board.size = board.letters.len();
    if board.size < 2 {
        println!("Error: board must be at least 2x2");
        return;
    }

    // Get the remaining rows
    for r in 1..board.size {
        let row = prompt(format!("R{}: ", r + 1)).to_lowercase().replace("q", "_");
        if row.len() == board.size {
            board.letters += &row;
        } else {
            println!("Error: incorrect size for R{}", r + 1);
            return;
        }
    }


    // Get the current SystemTime
    let time = SystemTime::now();


    let words = get_word_candidates(&board);
    let prefixes = get_prefixes(&words);

    
    // Start the search
    let mut results: HashSet<String> = HashSet::new();
    for x in 0..board.size {
        for y in 0..board.size {
            let point = Point { x: x, y: y };
            let path = Path { points: vec![point.clone()], string: get_letter(&point, &board) };
            continue_path(path, &board, &words, &prefixes, &mut results);
        }
    }
    

    // Sort the words by size, then alphabetically
    let mut sorted_results = Vec::new();
    sorted_results.extend(results.into_iter());
    sorted_results.sort();
    sorted_results.sort_by_key(|w| -(w.len() as i8));


    // Display the results
    let elapsed = time.elapsed().unwrap();
    println!("{} word(s) found in {} seconds:", sorted_results.len(), (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000000000f64));
    for word in sorted_results {
        println!("{}", word.replace("_", "qu"));
    }
}