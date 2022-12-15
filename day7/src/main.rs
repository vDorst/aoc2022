use std::io::Read;

const DISK_SPACE: u32 = 70_000_000;
const DISK_SPACE_NEEDED: u32 = 30_000_000;

fn part1(input: &str) -> u32 {
    let tree = build_tree(input);

    let sum: u32 = tree
        .iter()
        .filter(|e| e.item == Item::Dir && e.size <= 100_000)
        .map(|e| e.size)
        .sum();
    sum
}

fn part2(input: &str) -> u32 {
    let tree = build_tree(input);

    let disk_usage = tree[0].size;
  
    let current_free = DISK_SPACE - disk_usage;

    let size_to_delete = DISK_SPACE_NEEDED - current_free;

    let dir_size: u32 = tree
        .iter()
        .filter(|e| e.item == Item::Dir && e.size >= size_to_delete)
        .map(|e| e.size)
        .min().unwrap();

    dir_size
}

fn main() {
    let mut f = std::fs::File::open("input/input.txt").unwrap();
    let mut input = String::with_capacity(1_000_000);
    f.read_to_string(&mut input).unwrap();

    println!("Part1: sum: {}", part1(&input));
    println!("Part2: sum: {}", part2(&input));
}

#[derive(Debug, PartialEq, Eq)]
enum Item {
    Dir,
    File,
}

type ParentID = u16;

#[derive(Debug, PartialEq, Eq)]
struct Entry<'a> {
    name: &'a str,
    parent: Option<ParentID>,
    item: Item,
    size: u32,
    id: ParentID,
}

fn build_tree(input: &str) -> Vec<Entry> {
    let mut entries = Vec::<Entry>::with_capacity(1000);

    entries.push(Entry {
        name: "/",
        parent: None,
        item: Item::Dir,
        size: 0,
        id: 0,
    });

    let mut path = Vec::<ParentID>::with_capacity(10);
    path.push(0);

    for line in input.split_terminator('\n') {
        let mut b = line.split_terminator(' ');

        let start = b.next().unwrap();

        match start {
            "$" => match b.next().unwrap() {
                "ls" => (),
                "cd" => match b.next().unwrap() {
                    "/" => {
                        path.clear();
                        path.push(0);
                    }
                    ".." => {
                        let current_id = path.pop().unwrap() as usize;
                        let parent_id = path.last().unwrap().to_owned() as usize;
                        let size = entries[current_id].size;

                        entries[parent_id].size += size;
                    }
                    name => {
                        let parent_id = path.last().cloned();
                        let parent = entries
                            .iter()
                            .find(|e| e.parent == parent_id && e.name == name)
                            .map(|x| x.id)
                            .unwrap();
                        path.push(parent);
                    }
                },
                _ => (),
            },
            "dir" => {
                let parent = path.last().cloned();
                entries.push(Entry {
                    name: b.next().unwrap(),
                    parent,
                    item: Item::Dir,
                    size: 0,
                    id: entries.len() as ParentID,
                })
            }
            size => {
                let parent = path.last().cloned();
                let size = size.parse().unwrap();

                entries[parent.unwrap() as usize].size += size;
                entries.push(Entry {
                    name: b.next().unwrap(),
                    parent,
                    item: Item::File,
                    size,
                    id: entries.len() as ParentID,
                })
            }
        }
    }

    loop {
        if path.len() < 2 {
            break;
        }
        let current_id = path.pop().unwrap() as usize;
        let parent_id = path.last().unwrap().to_owned() as usize;
        let size = entries[current_id].size;

        entries[parent_id].size += size;
    }
    entries
}

#[cfg(test)]
mod tests {
    use super::{build_tree, Item, DISK_SPACE, DISK_SPACE_NEEDED};

    const INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn test_example() {
        let tree = build_tree(INPUT);

        assert_eq!(tree.len(), 14);
        assert_eq!(tree[0].size, 48381165);

        let sum: u32 = tree
            .iter()
            .filter(|e| e.item == Item::Dir && e.size <= 100_000)
            .map(|e| e.size)
            .sum();
        assert_eq!(sum, 95437);
    }


    #[test]
    fn test_example_part2() {
        let tree = build_tree(INPUT);

        assert_eq!(tree.len(), 14);

        let disk_usage = tree[0].size;
        assert_eq!(disk_usage, 48_381_165);

        let current_free = DISK_SPACE - disk_usage;

        assert_eq!(current_free, 21_618_835);

        let size_to_delete = DISK_SPACE_NEEDED - current_free;

        assert_eq!(size_to_delete, 8_381_165);

        let dir_size: u32 = tree
            .iter()
            .filter(|e| e.item == Item::Dir && e.size >= size_to_delete)
            .map(|e| e.size)
            .min().unwrap();

        assert_eq!(dir_size, 24_933_642);

    }
}
