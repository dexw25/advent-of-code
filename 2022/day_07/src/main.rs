#![allow(clippy::expect_used)]
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};
enum FsItem {
    VirtualFile {
        size: usize,
        parent: Option<Weak<RefCell<FsItem>>>,
    },
    VirtualDir {
        // The overhead of the hashmap isn't quite necessary but maybe speeding up the directory access methods will negate the dynamic dispatch cost
        // the problem statement doesnt need the names in part 1 really, but it's easier to just keep them
        items: HashMap<String, Rc<RefCell<FsItem>>>,
        parent: Option<Weak<RefCell<FsItem>>>,
    },
}

impl FsItem {
    fn size(&self) -> usize {
        match self {
            Self::VirtualDir { items, .. } => items.values().map(|i| i.borrow().size()).sum(),
            Self::VirtualFile { size, .. } => *size,
        }
    }
    fn link_parent(&mut self, new_parent: Weak<RefCell<Self>>) {
        match self {
            Self::VirtualFile { parent, .. } | Self::VirtualDir { parent, .. } => {
                *parent = Some(new_parent);
            }
        }
    }
    fn get_parent(&self) -> Option<Weak<RefCell<Self>>> {
        match self {
            Self::VirtualDir { parent, .. } | Self::VirtualFile { parent, .. } => parent.clone(),
        }
    }

    fn link_child(&mut self, name: &str, new_child: Self) {
        match self {
            Self::VirtualDir { items, .. } => {
                items.insert(name.to_string(), Rc::new(RefCell::new(new_child)));
            }
            Self::VirtualFile { .. } => panic!("Illegal to link to not dir"),
        }
    }
    fn get_child(&self, name: &str) -> Option<Rc<RefCell<Self>>> {
        match self {
            Self::VirtualFile { .. } => panic!("Cannot get child of file"),
            Self::VirtualDir { items, .. } => items.get(name).cloned(),
        }
    }
}

fn main() {
    let input = include_str!("input.txt").lines();

    let root_node = Rc::new(RefCell::new(FsItem::VirtualDir {
        items: HashMap::new(),
        parent: None,
    }));

    let mut cwd = root_node.clone();

    for line in input {
        if line == "$ ls" {
            // I don't think I have to do anything except just let this part pass
        } else if let Some(arg) = line.strip_prefix("$ cd ") {
            match arg {
                ".." => {
                    let parent = cwd
                        .borrow()
                        .get_parent()
                        .expect("cd up from root not allowed")
                        .upgrade()
                        .expect("BAD");
                    cwd = parent;
                }
                "/" => {
                    // Go up to root
                    cwd = root_node.clone();
                }
                dir => {
                    let folder = cwd.borrow().get_child(dir).expect("folder does not exist!");
                    cwd = folder;
                }
            }
        } else if let Some(dir) = line.strip_prefix("dir ") {
            let mut new_dir = FsItem::VirtualDir {
                items: HashMap::new(),
                parent: Some(Rc::downgrade(&cwd)),
            };
            new_dir.link_parent(Rc::downgrade(&cwd));
            // Assume no path will be listed more than once and make a new folder on the tree
            cwd.borrow_mut().link_child(dir, new_dir);
        } else {
            todo!("file addition");
        }
    }

    println!("Hello, world!");
}
