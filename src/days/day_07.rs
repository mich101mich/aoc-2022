use crate::utils::*;

#[derive(Debug, Default)]
struct Dir {
    dirs: HashMap<&'static str, Dir>,
    files: HashMap<&'static str, usize>,
    size: usize,
}

impl Dir {
    fn update(&mut self) {
        self.size = self.files.values().sum();
        for dir in self.dirs.values_mut() {
            dir.update();
            self.size += dir.size;
        }
    }
}

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/07.txt");

    let mut files = Dir::default();
    let mut parents: Vec<*mut Dir> = vec![];
    let mut current = &mut files;

    for l in input.lines() {
        if let Some(dir) = l.strip_prefix("$ cd ") {
            match dir {
                "/" => {
                    parents.clear();
                    current = &mut files;
                }
                ".." => {
                    current = unsafe { &mut *parents.pop().unwrap() };
                }
                s => {
                    parents.push(current as *mut Dir);
                    current = current.dirs.get_mut(s).unwrap();
                }
            }
        } else if let Some(name) = l.strip_prefix("dir ") {
            current.dirs.insert(name, Dir::default());
        } else if let Ok((size, name)) = sscanf!(l, "{usize} {str}") {
            current.files.insert(name, size);
            current.size += size;
        }
    }

    files.update();

    let free = 70000000 - files.size;
    let required = 30000000 - free;
    let mut best_fit = files.size;

    let mut next = vec![&files];
    while let Some(dir) = next.pop() {
        for (_, dir) in &dir.dirs {
            next.push(dir);
        }
        if dir.size >= required && dir.size < best_fit {
            best_fit = dir.size;
        }
    }

    pv!(best_fit);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/07.txt");

    let mut files = Dir::default();
    let mut parents: Vec<*mut Dir> = vec![];
    let mut current = &mut files;

    for l in input.lines() {
        if let Some(dir) = l.strip_prefix("$ cd ") {
            match dir {
                "/" => {
                    parents.clear();
                    current = &mut files;
                }
                ".." => {
                    current = unsafe { &mut *parents.pop().unwrap() };
                }
                s => {
                    parents.push(current as *mut Dir);
                    current = current.dirs.get_mut(s).unwrap();
                }
            }
        } else if let Some(name) = l.strip_prefix("dir ") {
            current.dirs.insert(name, Dir::default());
        } else if let Ok((size, name)) = sscanf!(l, "{usize} {str}") {
            current.files.insert(name, size);
            current.size += size;
        }
    }

    files.update();

    let mut next = vec![&files];
    let mut sum = 0;
    while let Some(dir) = next.pop() {
        for (_, dir) in &dir.dirs {
            next.push(dir);
        }
        if dir.size <= 100000 {
            sum += dir.size;
        }
    }

    pv!(sum);
}
