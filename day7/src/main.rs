use std::{
    env,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

const FILE_SYSTEM: usize = 70_000_000;
const UPDATE_SIZE: usize = 30_000_000;

fn get_input_lines<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path> + Copy,
{
    let mut file = env::current_dir().unwrap();
    file.push(filename);
    let file = File::open(filename).unwrap();
    io::BufReader::new(file)
        .lines()
        .map(|x| x.unwrap())
        .collect()
}

fn main() {
    let lines = get_input_lines("src/input.txt");
    // Stack of directory sizes
    let mut directory_stack: Vec<(String, usize)> = Vec::new();
    // List of indexed directories and their sizes
    let mut indexed_directories: Vec<(String, usize)> = Vec::new();
    for cmd in lines {
        let mut cmd = cmd.split_whitespace();
        // Match the three patterns we care about
        match (cmd.next(), cmd.next(), cmd.next()) {
            // Done searching directory
            (Some("$"), Some("cd"), Some("..")) => {
                let current_dir = directory_stack.pop().unwrap();
                let parent_dir = directory_stack.last_mut().unwrap();
                parent_dir.1 += current_dir.1;
                indexed_directories.push(current_dir);
            }
            // New directory to explore
            (Some("$"), Some("cd"), Some(child)) => directory_stack.push((child.to_string(), 0)),
            // Noise
            (Some("$"), _, _) => continue,
            (Some("dir"), _, _) => continue,
            // Sized file
            (Some(size), Some(_), None) => {
                let current_dir = directory_stack.last_mut().unwrap();
                let file_size = size.parse::<usize>().unwrap();
                current_dir.1 += file_size;
            }
            _ => continue,
        }
    }

    // Move over directories which were never popped
    let mut sum = 0;
    while let Some(mut dir) = directory_stack.pop() {
        sum += dir.1;
        dir.1 = sum;
        indexed_directories.push(dir);
    }
    // Sum all directories with a size of 100_000 or less
    let answer_1 = indexed_directories
        .iter()
        .filter(|dir| dir.1 <= 100_000)
        .fold(0, |acc, dir| acc + dir.1);

    // Last dir is '/', since it is the directory tree in reverse
    let used_size = indexed_directories.last().unwrap().1;
    let size_required_for_update = UPDATE_SIZE - (FILE_SYSTEM - used_size);
    // Filter out all directories which are too small, then take the smallest that is left
    let answer_2 = indexed_directories
        .iter()
        .filter(|dir| dir.1 >= size_required_for_update)
        .min_by(|x, y| x.1.cmp(&y.1))
        .unwrap()
        .1;

    println!("Answer 1: {answer_1}");
    println!("Asnwer 2: {answer_2}");
}
