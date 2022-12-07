use std::{
    collections::HashMap,
    path::{Path, PathBuf},
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
    todo!()
}

fn task2(input: &str) -> Result<usize> {
    todo!()
}

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

    fn register_paths(&mut self, directory: PathBuf, entries: Vec<Entry>) -> Result<()> {
        let dir_as_str = directory.display();

        {
            // Need a scoped blocked, otherwise would borrow `self.fs` mutable twice
            // about which the borrow checker is _not happy_.
            let d = self
                .fs
                .get_mut(&directory)
                .ok_or_else(|| anyhow!("Unknown directory: {}", dir_as_str))?;

            let d = match d {
                Entry::File(_) => bail!("files can't have children: {}", dir_as_str),
                Entry::Dir(d) => d,
            };

            d.children.extend(entries.iter().map(|e| e.name()));
        }

        for entry in entries {
            let path: PathBuf = format!("{dir_as_str}/{}", entry.name()).into();

            self.fs.insert(path, entry);
        }

        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Command {
    Ls(Vec<String>),
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
            todo!()
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
        let res = match s {
            ".." => Self::Up,
            "/" => Self::Root,
            dir => Self::Dir(dir.to_string()),
        };

        Ok(res)
    }
}

enum Entry {
    File(File),
    Dir(Directory),
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

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
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
}
