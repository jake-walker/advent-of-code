use std::fs;
use counter::Counter;

#[derive(Debug, PartialEq, Eq)]
struct File {
    full_path: String,
    size: i32
}

// parse the input log as a flat list of files and their sizes (e.g. [('/a/b/c', 1234), ('/a/x', 42)])
fn parse_tree(input: &str) -> Vec<File> {
    let mut items: Vec<File> = Vec::new();
    // this stores the current folder that we are in
    let mut prefix = "/".to_string();

    // iterate through each line of the log, skipping the first line (assuming it is "$ cd /")
    for line in input.lines().skip(1) {
        // if this is a cd command
        if line.starts_with("$ cd") {
            // get the folder path by getting the last item when splitting at spaces
            let dir = line.split_whitespace().last().unwrap();

            // if going up a folder
            if dir == ".." {
                // remove the final character (leading slash)
                prefix.pop();
                // set the prefix - remove everything including and after the last slash
                // split once from the right giving rest of path and last item
                prefix = prefix.rsplit_once('/').unwrap().0.to_string() + "/";
            } else {
                prefix.push_str(&(dir.to_string() + "/"));
            }
        // we don't care about other commands - assuming all other output is ls
        } else if !line.starts_with("$ ") {
            let parts = line.split_whitespace().collect::<Vec<&str>>();
            // the first part is either "dir" or the file size
            // dirs can be ignored because the prefix tells where the files are
            if parts.get(0).unwrap() != &"dir" {
                // add a new item with the prefix + the file name and file size
                items.push(File { full_path: prefix.clone() + parts.get(1).unwrap(), size: parts.get(0).unwrap().parse().unwrap() })
            }
        }
    }

    items
}

// count the sizes of each directory (e.g. [('/a/b', 1234), ('/a', 1276)])
fn count_dirs(tree: &Vec<File>) -> Counter<String> {
    // python-style counter - there's maybe something that doesn't involve libraries but this is quick and easy
    let mut counter: Counter<String> = Counter::new();

    for item in tree {
        // split by slash to get directory for each file
        let parts = item.full_path.split('/').collect::<Vec<&str>>();
        // step through the most path components to the least
        // e.g. /a/b/c -> /a/b/c, /a/b, /a
        // then add the current file size to each of those partial paths
        for i in (2..parts.len()).rev() {
            counter[&("/".to_string() + &parts[1..i].join("/"))] += item.size as usize;
        }
        // at the end each directory has the size of all the files inside (including nested folders)
    }

    counter
}

// find the smallest directories underneath a given threshold
fn smallest_dirs(tree: &Vec<File>, threshold: i32) -> i32 {
    let counter = count_dirs(tree);
    counter.most_common().iter().filter(|x| (x.1 as i32) <= threshold).map(|x| x.1 as i32).sum::<i32>()
}

// calculate the amount of space that needs to be freed given the total space and space needed
fn space_to_free(tree: &Vec<File>, available_space: i32, required_space: i32) -> i32 {
    let used_space: i32 = tree.iter().map(|x| x.size).sum();
    let unused_space = available_space - used_space;
    required_space - unused_space
}

// for part 2, find the best directory to delete - closest to the amount that needs to be freed
fn dir_to_delete(tree: &Vec<File>, to_free: i32) -> i32 {
    let counter = count_dirs(tree);
    // filter out directories under the amount to be freed, then take the smallest of what is left
    counter.iter().map(|x| *x.1 as i32).filter(|x| *x > to_free).min().unwrap()
}

fn main() {
    let input = fs::read_to_string("inputs/input.txt").expect("should be able to read input");
    let tree = parse_tree(&input);

    println!("Part 1: {}", smallest_dirs(&tree, 100000));

    let to_free = space_to_free(&tree, 70000000, 30000000);
    println!("Part 2: {}", dir_to_delete(&tree, to_free))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use assert_unordered::assert_eq_unordered;

    #[test]
    fn parse_tree_example() {
        let input = fs::read_to_string("inputs/example.txt").expect("should be able to read input");
        assert_eq_unordered!(parse_tree(&input), vec![
            File { full_path: "/a/e/i".to_string(), size: 584 },
            File { full_path: "/a/f".to_string(), size: 29116 },
            File { full_path: "/a/g".to_string(), size: 2557 },
            File { full_path: "/a/h.lst".to_string(), size: 62596 },
            File { full_path: "/b.txt".to_string(), size: 14848514 },
            File { full_path: "/c.dat".to_string(), size: 8504156 },
            File { full_path: "/d/j".to_string(), size: 4060174 },
            File { full_path: "/d/d.log".to_string(), size: 8033020 },
            File { full_path: "/d/d.ext".to_string(), size: 5626152 },
            File { full_path: "/d/k".to_string(), size: 7214296 }
        ]);
    }

    #[test]
    fn test_smallest_dirs() {
        assert_eq!(smallest_dirs(&vec![
            File { full_path: "/a/e/i".to_string(), size: 584 },
            File { full_path: "/a/f".to_string(), size: 29116 },
            File { full_path: "/a/g".to_string(), size: 2557 },
            File { full_path: "/a/h.lst".to_string(), size: 62596 },
            File { full_path: "/b.txt".to_string(), size: 14848514 },
            File { full_path: "/c.dat".to_string(), size: 8504156 },
            File { full_path: "/d/j".to_string(), size: 4060174 },
            File { full_path: "/d/d.log".to_string(), size: 8033020 },
            File { full_path: "/d/d.ext".to_string(), size: 5626152 },
            File { full_path: "/d/k".to_string(), size: 7214296 }
        ], 100000), 95437);
    }

    #[test]
    fn test_space_to_free() {
        assert_eq!(space_to_free(&vec![
            File { full_path: "/a/e/i".to_string(), size: 584 },
            File { full_path: "/a/f".to_string(), size: 29116 },
            File { full_path: "/a/g".to_string(), size: 2557 },
            File { full_path: "/a/h.lst".to_string(), size: 62596 },
            File { full_path: "/b.txt".to_string(), size: 14848514 },
            File { full_path: "/c.dat".to_string(), size: 8504156 },
            File { full_path: "/d/j".to_string(), size: 4060174 },
            File { full_path: "/d/d.log".to_string(), size: 8033020 },
            File { full_path: "/d/d.ext".to_string(), size: 5626152 },
            File { full_path: "/d/k".to_string(), size: 7214296 }
        ], 70000000, 30000000), 8381165);
    }

    #[test]
    fn test_dir_to_delete() {
        assert_eq!(dir_to_delete(&vec![
            File { full_path: "/a/e/i".to_string(), size: 584 },
            File { full_path: "/a/f".to_string(), size: 29116 },
            File { full_path: "/a/g".to_string(), size: 2557 },
            File { full_path: "/a/h.lst".to_string(), size: 62596 },
            File { full_path: "/b.txt".to_string(), size: 14848514 },
            File { full_path: "/c.dat".to_string(), size: 8504156 },
            File { full_path: "/d/j".to_string(), size: 4060174 },
            File { full_path: "/d/d.log".to_string(), size: 8033020 },
            File { full_path: "/d/d.ext".to_string(), size: 5626152 },
            File { full_path: "/d/k".to_string(), size: 7214296 }
        ], 8381165), 24933642);
    }
}