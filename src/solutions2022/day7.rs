use crate::files::lines;
use indextree::{Arena, NodeId};
use itertools::Itertools;
use regex::Regex;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

type PuzzleResult = usize;

pub fn day7() -> (PuzzleResult, PuzzleResult) {
    let lines_of_file = lines("inputs/2022/day7.txt");
    inner(lines_of_file)
}

#[derive(Debug, PartialEq)]
enum LsOutput {
    File(String, usize),
    Directory(String),
}

#[derive(Debug, PartialEq)]
enum Commands {
    Ls(Vec<LsOutput>),
    Cd(String),
}

#[derive(Debug)]
enum IntermediateResult {
    Command(String, Option<String>),
    OutputLine(String),
}

fn parse_intermediate_results(lines: Vec<String>) -> Result<Vec<IntermediateResult>, String> {
    let re = Regex::new(r"^\$ (?<command>\S+)\s?(?<args>.*)$").unwrap();

    let commands_and_output = lines
        .iter()
        .map(|line| match re.captures(line) {
            Some(caps) => {
                let command_string = caps["command"].to_string();
                let args = caps.name("args").map(|m| m.as_str().to_string());
                IntermediateResult::Command(command_string, args)
            }
            None => IntermediateResult::OutputLine(line.to_string()),
        })
        .collect_vec();
    let first_command = commands_and_output.first().unwrap();
    if let IntermediateResult::OutputLine(_) = first_command {
        return Err("Wrong input. The first line is not a command".to_string());
    }
    return Ok(commands_and_output);
}

fn parse_commands(lines: Vec<String>) -> Result<Vec<Commands>, String> {
    if lines.is_empty() {
        return Ok(Vec::new());
    }

    let commands_and_output = parse_intermediate_results(lines)?;

    let mut outputs = Vec::new();
    let mut ret = Vec::new();
    for res in commands_and_output.iter().rev() {
        match res {
            IntermediateResult::Command(name, args) => {
                let command = match name.as_str() {
                    "ls" => {
                        outputs.reverse();
                        let ls_outputs = parse_ls_outputs(&outputs);
                        outputs.truncate(0);
                        Commands::Ls(ls_outputs)
                    }
                    "cd" => {
                        let option = args.as_ref();
                        Commands::Cd(
                            option
                                .expect("The cd command must always have arguments")
                                .clone(),
                        )
                    }
                    _ => {
                        let error_message = format!("Unknown command: {name}");
                        return Err(error_message);
                    }
                };
                ret.push(command);
            }
            IntermediateResult::OutputLine(line) => {
                outputs.push(line.clone());
            }
        }
    }
    ret.reverse();
    Ok(ret)
}

fn parse_ls_outputs(lines: &Vec<String>) -> Vec<LsOutput> {
    lines
        .iter()
        .map(|line| {
            let error_message = format!("Invalid output line for 'ls' command: {line}");
            let (a, b) = line.split_once(" ").expect(error_message.as_str());
            return if a == "dir" {
                LsOutput::Directory(b.to_string())
            } else {
                let error_message = format!("Invalid file size for file {b}: {a}");
                let file_size = a.parse::<usize>().expect(error_message.as_str());
                LsOutput::File(b.to_string(), file_size)
            };
        })
        .collect_vec()
}

#[derive(Debug, PartialEq)]
struct Data {
    name: String,
    files: HashMap<String, usize>,
}

struct Filesystem {
    arena: Rc<RefCell<Arena<Data>>>,
    node: NodeId,
}

impl Filesystem {
    pub fn empty() -> Filesystem {
        Filesystem::new(HashMap::new())
    }

    /// A convenience function
    fn convert_files(files: HashMap<&str, i32>) -> HashMap<String, usize> {
        files
            .iter()
            .map(|(filename, size)| (filename.to_string(), (*size as usize)))
            .collect()
    }

    pub fn new(files: HashMap<&str, i32>) -> Filesystem {
        let files = Filesystem::convert_files(files);
        let mut arena = Arena::new();
        let root = arena.new_node(Data {
            files,
            name: "/".to_string(),
        });
        Filesystem {
            arena: Rc::new(RefCell::new(arena)),
            node: root,
        }
    }

    pub fn name(&self) -> String {
        let arena = self.arena.borrow();
        arena.get(self.node).unwrap().get().name.clone()
    }

    pub fn ls(&self) -> Vec<LsOutput> {
        let arena = self.arena.borrow();
        let directories = self
            .node
            .children(&arena)
            .map(|node| arena.get(node).unwrap().get())
            .map(|data| data.name.clone())
            .sorted()
            .map(|subdir| LsOutput::Directory(subdir.clone()));
        let files = arena
            .get(self.node)
            .unwrap()
            .get()
            .files
            .iter()
            .sorted_by_key(|(filename, _size)| *filename)
            .map(|(filename, size)| LsOutput::File(filename.clone(), *size));

        return directories.chain(files).collect_vec();
    }
    pub fn cd(&self, directory: &str) -> Option<Filesystem> {
        let arena = self.arena.borrow();
        match directory {
            ".." => {
                let parent = self.node.ancestors(&arena).skip(1).next();
                parent.map(|node| {
                    let ret = Filesystem {
                        arena: self.arena.clone(),
                        node,
                    };
                    ret
                })
            }
            "/" => {
                let root = self.node.ancestors(&arena).last();
                root.map(|node| Filesystem {
                    arena: self.arena.clone(),
                    node,
                })
            }
            _ => {
                self.node
                    .children(&arena)
                    .find(|node| {
                        let data = arena.get(*node).unwrap().get();
                        data.name == directory
                    })
                    .map(|node| Filesystem {
                        arena: self.arena.clone(),
                        node,
                    })
            }
        }
    }

    pub fn mkdir(&mut self, directory: &str, files: HashMap<&str, i32>) -> Filesystem {
        let files = Filesystem::convert_files(files);
        let mut arena = self.arena.borrow_mut();
        let node = arena.new_node(Data {
            name: directory.to_string(),
            files,
        });
        self.node.append(node, arena.deref_mut());
        Filesystem {
            arena: self.arena.clone(),
            node,
        }
    }

    pub fn touch_file(&mut self, filename: &str, filesize: usize) {
        let mut arena = self.arena.borrow_mut();
        let mut data = arena.get_mut(self.node).unwrap().get_mut();
        data.files.insert(filename.to_string(), filesize);
    }

    fn foo(parent: &Filesystem, ret: &mut Vec<Filesystem>) {
        for ls in parent.ls() {
            if let LsOutput::Directory(dir) = ls {
                let filesystem = parent.cd(dir.as_str()).unwrap();
                let subdir = filesystem;
                Filesystem::foo(&subdir, ret);
                ret.push(subdir);
            }
        }
    }

    pub fn walk(&self) -> Vec<Filesystem> {
        let arena = self.arena.borrow();
        self.node
            .descendants(&arena)
            .map(|node| Filesystem {
                arena: self.arena.clone(),
                node,
            })
            .collect_vec()
    }
}

impl Debug for Filesystem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let arena = self.arena.borrow();
        let s = format!("{:?}", self.node.debug_pretty_print(&arena));
        f.write_str(s.as_str())
    }
}

impl PartialEq for Filesystem {
    fn eq(&self, other: &Self) -> bool {
        let lhs = &*self.arena.borrow();
        let rhs = &*other.arena.borrow();
        lhs == rhs && self.node == other.node
    }
}

fn build_filesystem_from_commands(commands: Vec<Commands>) -> Filesystem {
    let mut current = Filesystem::empty();
    for command in commands.iter() {
        match command {
            Commands::Ls(outputs) => {
                for entry in outputs.iter() {
                    match entry {
                        LsOutput::File(filename, size) => {
                            current.touch_file(filename, *size);
                        }
                        LsOutput::Directory(dirname) => {
                            current.mkdir(dirname, HashMap::new());
                        }
                    }
                }
            }
            Commands::Cd(directory) => {
                current = current
                    .cd(directory)
                    .unwrap_or_else(|| current.mkdir(directory, HashMap::new()))
            }
        }
    }
    current.cd("/").unwrap()
}

fn calculate_recursive_size(filesystem: &Filesystem) -> usize {
    filesystem
        .ls()
        .iter()
        .map(|ls_output| match ls_output {
            LsOutput::File(_, filesize) => *filesize,
            LsOutput::Directory(dirname) => {
                calculate_recursive_size(&filesystem.cd(dirname).unwrap())
            }
        })
        .sum()
}

fn inner(lines: Vec<String>) -> (PuzzleResult, PuzzleResult) {
    let commands = parse_commands(lines).unwrap();
    let root = build_filesystem_from_commands(commands);
    let part1 = root
        .walk()
        .iter()
        .map(|filesystem| calculate_recursive_size(&filesystem))
        .filter(|size| *size <= 100_000)
        .sum::<usize>();

    let total_space = 70_000_000;
    let used_space = calculate_recursive_size(&root);
    let free_space = total_space - used_space;
    let required_space = (30_000_000 - free_space);
    let part2_dir = root
        .walk()
        .iter()
        .map(|filesystem| (filesystem.name(), calculate_recursive_size(&filesystem)))
        .filter(|(name, size)| *size >= required_space)
        .min_by_key(|(name, size)| *size)
        .unwrap();
    let part2 = part2_dir.1;
    (part1, part2)
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use map_macro::hash_map;
    use textwrap::dedent;

    use super::*;
    use crate::strings::SkipEmptyLines;
    use pretty_assertions::assert_eq;

    fn to_vec(f: Vec<&str>) -> Vec<String> {
        f.into_iter().map(|s| String::from(s)).collect_vec()
    }

    #[test]
    fn test_part1_inner() -> Result<(), String> {
        let commands_string = dedent(
            "
            $ cd /
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
            7214296 k",
        )
        .skip_empty_start_lines();
        let result = inner(to_vec(commands_string.lines().collect_vec())).0;

        assert_eq!(result, 95437);

        Ok(())
    }

    #[test]
    fn test_part2_inner() -> Result<(), String> {
        let commands_string = dedent(
            "
            $ cd /
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
            7214296 k",
        )
        .skip_empty_start_lines();
        let result = inner(to_vec(commands_string.lines().collect_vec())).1;
        assert_eq!(result, 24933642);

        Ok(())
    }

    #[test]
    fn test_walk() -> Result<(), String> {
        let filesystem = filesystem();
        let vec = filesystem.walk();
        // assert_eq!(vec);
        Ok(())
    }

    #[test]
    fn test_parse_commands() -> Result<(), String> {
        let commands_string = dedent(
            "
            $ cd /
            $ ls
            dir a
            14848514 b.txt
            8504156 c.dat
            dir d
        ",
        )
        .skip_empty_start_lines();
        let commands = parse_commands(to_vec(commands_string.lines().collect_vec()))?;
        assert_eq!(commands.len(), 2);

        assert_eq!(commands[0], Commands::Cd("/".to_string()));
        let ls_output = vec![
            LsOutput::Directory("a".to_string()),
            LsOutput::File("b.txt".to_string(), 14848514),
            LsOutput::File("c.dat".to_string(), 8504156),
            LsOutput::Directory("d".to_string()),
        ];
        assert_eq!(commands[1], Commands::Ls(ls_output));
        Ok(())
    }

    #[test]
    fn test_build_filesystem_representation() -> Result<(), String> {
        let filesystem = filesystem();
        let commands_string = dedent(
            "
            $ cd /
            $ ls
            dir a
            10 b.txt
            20 c.dat
            dir d
            $ cd d
            $ ls
            99 d.txt
            $ cd e
        ",
        )
        .skip_empty_start_lines();
        let commands = parse_commands(to_vec(commands_string.lines().collect_vec()))?;
        let root = build_filesystem_from_commands(commands);
        assert_eq!(root, filesystem);
        Ok(())
    }

    #[test]
    fn test_ls() {
        let root = filesystem();
        assert_eq!(
            root.ls(),
            vec![
                LsOutput::Directory("a".to_string()),
                LsOutput::Directory("d".to_string()),
                LsOutput::File("b.txt".to_string(), 10),
                LsOutput::File("c.dat".to_string(), 20),
            ]
        );

        assert_eq!(root.cd("a").unwrap().ls(), vec![]);
        let d_directory = root.cd("d").unwrap();
        assert_eq!(d_directory.name(), "d");
        let d_contents = d_directory.ls();
        assert_eq!(
            d_contents,
            vec![
                LsOutput::Directory("e".to_string()),
                LsOutput::File("d.txt".to_string(), 99),
            ]
        );
    }

    #[test]
    fn test_navigate_one_level_up() {
        let filesystem = filesystem();
        let dir_a = filesystem.cd("a").unwrap();
        assert_eq!(dir_a.name(), "a");

        assert_ne!(dir_a, filesystem);
        let back_to_root = dir_a.cd("..");
        assert!(back_to_root.is_some());
        let back_to_root = back_to_root.unwrap();
        assert_eq!(back_to_root.name(), "/");
        assert_eq!(back_to_root, filesystem);

        let actual = filesystem.cd("d").unwrap().cd("e").unwrap().cd("..");
        assert!(actual.is_some());
        assert_eq!(actual, filesystem.cd("d"));
    }

    #[test]
    fn test_navigate_up_at_root_returns_none() {
        let filesystem = filesystem();
        let actual = filesystem.cd("..");
        assert_eq!(actual, None);
    }

    #[test]
    fn test_navigate_to_root() {
        let root = filesystem();
        assert_eq!(root.cd("/").as_ref(), Some(&root));

        let subdir = &root.cd("a").unwrap();
        assert_eq!(subdir.cd("/").as_ref(), Some(&root));
    }

    #[test]
    fn test_calculate_dir_size() {
        let filesystem = filesystem();
        assert_eq!(calculate_recursive_size(&Filesystem::empty()), 0);

        assert_eq!(calculate_recursive_size(&filesystem), 10 + 20 + 99);
        assert_eq!(calculate_recursive_size(&filesystem.cd("a").unwrap()), 0);
        assert_eq!(calculate_recursive_size(&filesystem.cd("d").unwrap()), 99);
    }

    fn filesystem() -> Filesystem {
        let mut ret = Filesystem::new(hash_map![
            "b.txt" => 10,
            "c.dat" => 20,
        ]);
        ret.mkdir("a", hash_map![]);
        let mut d = ret.mkdir(
            "d",
            hash_map![
                "d.txt" => 99,
            ],
        );
        d.mkdir("e", hash_map![]);
        return ret;
    }
}
