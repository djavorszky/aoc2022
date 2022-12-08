use std::{
    collections::HashMap,
    path::{PathBuf},
    str::FromStr,
};

use crate::prelude::*;

pub fn run() -> Result<()> {
    let input = include_str!("input/day7.txt");

    println!("{}", task1(input)?);

    println!("{}", task2(input)?);

    Ok(())
}

fn task1(input: &str) -> Result<usize> {
    let mut fs = FileSystem::new();

    for command in parse_commands(input)? {
        fs.apply(command)?;
    }

    Ok(fs
        .get_directory_sizes()
        .into_iter()
        .filter(|n| *n <= 100_000)
        .sum())
}

fn task2(input: &str) -> Result<usize> {
    let mut fs = FileSystem::new();

    for command in parse_commands(input)? {
        fs.apply(command)?;
    }

    let directory_sizes = fs.get_directory_sizes();

    let total_size: usize = *directory_sizes
        .iter()
        .max()
        .ok_or_else(|| anyhow!("numbers should have maxmum values"))?;
    let free_space = 70000000 - total_size;

    let mut smallest_big_folder_sizes: Vec<usize> = directory_sizes
        .into_iter()
        .filter(|n| *n + free_space > 30_000_000)
        .collect();

    smallest_big_folder_sizes.sort();

    smallest_big_folder_sizes
        .into_iter()
        .min()
        .ok_or_else(|| anyhow!("numbers should have minimum values"))
}

fn parse_commands(input: &str) -> Result<Vec<Command>> {
    input
        .split("$ ")
        .filter(|part| !part.is_empty())
        .map(|part| part.parse::<Command>())
        .collect()
}

#[derive(Debug)]
struct FileSystem {
    cwd: PathBuf,
    fs: HashMap<PathBuf, Entry>,
}

impl FileSystem {
    fn new() -> Self {
        let root_path: PathBuf = "/".into();
        let mut fs = HashMap::new();
        fs.insert(root_path.clone(), Entry::dir("/"));

        Self { cwd: root_path, fs }
    }

    fn cwd(&self) -> Result<Directory> {
        match self
            .fs
            .get(&self.cwd)
            .ok_or_else(|| anyhow!("cwd not in file system: {}", self.cwd.display()))?
        {
            Entry::File(_) => bail!("cwd is somehow a file: {}", self.cwd.display()),
            Entry::Dir(dir) => Ok(dir.clone()),
        }
    }

    fn register_paths(&mut self, entries: Vec<Entry>) -> Result<()> {
        let dir_as_str = self.cwd.display();

        {
            // Need a scoped blocked, otherwise would borrow `self.fs` mutable twice
            // about which the borrow checker is _not happy_.
            let d = self
                .fs
                .get_mut(&self.cwd)
                .ok_or_else(|| anyhow!("Unknown directory: {}", dir_as_str))?;

            let d = match d {
                Entry::File(_) => bail!("files can't have children: {}", dir_as_str),
                Entry::Dir(d) => d,
            };

            d.children.extend(entries.iter().map(|e| e.name()));
        }

        for entry in entries {
            let path: PathBuf = self.cwd.join(entry.name());

            self.fs.insert(path, entry);
        }

        Ok(())
    }

    fn apply(&mut self, command: Command) -> Result<()> {
        match command {
            Command::Ls(entries) => {
                self.register_paths(entries);

                Ok(())
            }
            Command::ChDir(target) => {
                self.cwd = match target {
                    Target::Root => "/".into(),
                    Target::Up => self
                        .cwd
                        .parent()
                        .map(|parent| parent.to_path_buf())
                        .ok_or_else(|| anyhow!("Can't go up from path: {}", self.cwd.display()))?,
                    Target::Dir(dir) => self.cwd.join(dir),
                };

                Ok(())
            }
        }
    }

    fn get_directory_sizes(&self) -> Vec<usize> {
        fn walk_folders(fs: &FileSystem, pb: &PathBuf, res: &mut Vec<usize>) -> Result<usize> {
            let entry = fs
                .fs
                .get(pb)
                .ok_or_else(|| anyhow!("can't walk into {}", pb.display()))?;

            if let Entry::Dir(dir) = entry {
                // I'm lazy to do error checking here....

                let mut size = 0;
                for child_name in dir.children.clone() {
                    let child_path = pb.join(child_name);
                    let child = fs
                        .fs
                        .get(&child_path)
                        .ok_or_else(|| anyhow!("can't find child",))?;

                    match child {
                        Entry::File(f) => size += f.size,
                        Entry::Dir(_d) => size += walk_folders(fs, &child_path, res)?,
                    }
                }

                res.push(size);

                return Ok(size);
            }
            bail!("not a directory: {}", entry.name())
        }

        let start: PathBuf = "/".into();
        let mut res = Vec::new();

        walk_folders(self, &start, &mut res);

        res
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Command {
    Ls(Vec<Entry>),
    ChDir(Target),
}

impl FromStr for Command {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if s.starts_with("cd") {
            let (_, s) = s
                .split_once(' ')
                .ok_or_else(|| anyhow!("Invalid chdir pattern: {s}"))?;

            let target: Target = s.parse()?;

            Ok(Self::ChDir(target))
        } else {
            let entries = s
                .lines()
                .skip(1)
                .map(|line| line.parse::<Entry>())
                .collect::<Result<Vec<Entry>>>()?;

            Ok(Self::Ls(entries))
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Target {
    Root,
    Up,
    Dir(String),
}

impl FromStr for Target {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let res = match s.trim() {
            ".." => Self::Up,
            "/" => Self::Root,
            dir => Self::Dir(dir.to_string()),
        };

        Ok(res)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Entry {
    File(File),
    Dir(Directory),
}

impl FromStr for Entry {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (first, second) = s
            .split_once(' ')
            .ok_or_else(|| anyhow!("Invalid entry pattern: {s}"))?;

        if first == "dir" {
            return Ok(Self::dir(second));
        }

        let size = first.parse::<usize>()?;
        Ok(Self::file(second, size))
    }
}

impl Entry {
    fn file(name: &str, size: usize) -> Self {
        Self::File(File::new(name, size))
    }

    fn dir(name: &str) -> Self {
        Self::Dir(Directory::new(name))
    }

    fn name(&self) -> String {
        match self {
            Entry::File(f) => f.name.clone(),
            Entry::Dir(d) => d.name.clone(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Directory {
    name: String,
    children: Vec<String>,
}

impl Directory {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            children: vec![],
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct File {
    name: String,
    size: usize,
}

impl File {
    fn new(name: &str, size: usize) -> Self {
        Self {
            name: name.to_string(),
            size,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use test_case::test_case;

    #[test_case("cd ..", Command::ChDir(Target::Up) ; "cd_up")]
    #[test_case("cd /", Command::ChDir(Target::Root) ; "cd_root")]
    #[test_case("cd hehe", Command::ChDir(Target::Dir("hehe".to_string())) ; "cd_hehe")]
    fn test_command_parse_chdir(input: &str, expected: Command) {
        assert_eq!(input.parse::<Command>().unwrap(), expected);
    }

    #[test]
    fn test_command_parse_ls() {
        let input = "ls\ndir ohno\n12001 f.txt";

        let expected = Command::Ls(vec![Entry::dir("ohno"), Entry::file("f.txt", 12001)]);

        assert_eq!(input.parse::<Command>().unwrap(), expected);
    }

    #[test]
    fn test_task_1() {
        let input = include_str!("input/day7_example.txt");

        assert_eq!(task1(input).unwrap(), 95437);
    }

    #[test]
    fn test_task_2() {
        let input = include_str!("input/day7_example.txt");

        assert_eq!(task2(input).unwrap(), 24933642);
    }
}
