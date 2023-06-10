use std::cell::RefCell;
use std::fs;
use std::collections::HashMap;
use std::rc::Rc;

trait FileSystem {
    fn get_size(&self) -> u64;
    fn get_name(&self) -> &String;
    fn get_dir(&mut self) -> Option<Rc<RefCell<Dir>>>;
    fn get_parent(&self) -> Option<Rc<RefCell<Dir>>>;
}

struct File {
    size: u64,
    name: String,
}

impl FileSystem for File {
    fn get_size(&self) -> u64 {
        self.size
    }

    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_dir(&mut self) -> Option<Rc<RefCell<Dir>>> {
        None
    }

    fn get_parent(&self) -> Option<Rc<RefCell<Dir>>> {
        None
    }
}

struct Dir {
    this: Option<Rc<RefCell<Dir>>>,
    parent: Option<Rc<RefCell<Dir>>>,
    size: u64,
    name: String,
    has: HashMap<String, Rc<RefCell<dyn FileSystem>>>,
}

impl FileSystem for Dir {
    fn get_size(&self) -> u64 {
        self.size
    }

    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_dir(&mut self) -> Option<Rc<RefCell<Dir>>> {
        Some(self.this.as_ref().unwrap().clone())
    }

    fn get_parent(&self) -> Option<Rc<RefCell<Dir>>> {
        Some(self.parent.as_ref().unwrap().clone())
    }
}

impl Dir {
    fn insert(&mut self, to_insert: Rc<RefCell<dyn FileSystem>>) {
        self.add_to_size(to_insert.borrow().get_size());
        self.has.insert(to_insert.borrow().get_name().to_owned(), to_insert.clone());
    }

    fn get_child(&self, child: &str) -> Rc<RefCell<dyn FileSystem>> {
        if child == ".." {
            self.parent.as_ref().expect("Should be legal name").clone()
        } else if child == "/" {
            if self.parent.is_some() {
                self.parent.as_ref().unwrap().clone()
            } else {
                self.this.as_ref().unwrap().clone()
            }
        } else {
            self.has.get(&child.to_string()).expect("Should be legal name").clone()
        }
    }

    fn get_total_size(&self) -> u64 {
        let mut total = 0;
        if self.size <= 100000 {
            total += self.size;
        }
        for child in self.has.values() {
            let dir_option = child.borrow_mut().get_dir();
            total += match dir_option {
                Some(child_as_dir) => child_as_dir.borrow().get_total_size(),
                None => 0,
            };
        }
        total
    }

    fn add_to_size(&mut self, size: u64) {
        self.size += size;
        if let Some(parent) = self.parent.as_mut() {
            parent.borrow_mut().add_to_size(size);
        };
    }

    fn get_min_size_can_remove(&self, cur_min_size: u64, min_size: u64) -> u64 {
        let mut res: u64 = cur_min_size;
        if self.size >= min_size {
            if res > self.size {
                res = self.size;

            }
            for child in self.has.values() {
                let dir_option = child.borrow_mut().get_dir();
                if let Some(child_as_dir) = dir_option {
                    let min_child = child_as_dir.borrow().get_min_size_can_remove(res, min_size);
                    if res > min_child && min_child >= min_size {
                        res = min_child;
                    }
                };
            }
        }
        res
    }
}

pub fn day_7a() -> u64 {
    let contents = fs::read_to_string("src/data/day7.txt").expect("Should be able to open file").trim().to_string();
    let mut dir = Rc::new(
        RefCell::new(Dir { this: None, parent: None, size: 0, name: "\\".to_string(), has: HashMap::new() })
    );
    dir.borrow_mut().this = Some(dir.clone());
    let mut cur_cmd = None;
    for line in contents.split('\n') {
        let cmd: Vec<_> = line.split(' ').collect();
        if cmd[0] == "$" {
            match cmd[1] {
                "cd" => {
                    let child = dir.borrow().get_child(cmd[2]);
                    match child.borrow_mut().get_dir() {
                        Some(new_dir) => {
                            dir = new_dir.clone();
                        }
                        None => panic!("cd should not be used on files"),
                    };
                }
                "ls" => {
                    cur_cmd = Some("ls");
                }
                _ => panic!("Illegal Command"),
            }
        } else {
            match cur_cmd {
                Some(cmd) => {
                    match cmd {
                        "ls" => {
                            let file_data: Vec<_> = line.split(" ").collect();
                            let (size, name) = (file_data[0], file_data[1]);
                            dir.borrow_mut().insert({
                                if size == "dir" {
                                    let res = Rc::new(
                                        RefCell::new(Dir {
                                            this: None,
                                            parent: Some(dir.clone()),
                                            size: 0,
                                            name: name.to_string(),
                                            has: HashMap::new(),
                                        })
                                    );
                                    res.borrow_mut().this = Some(res.clone());
                                    res
                                } else {
                                    Rc::new(
                                        RefCell::new(File {
                                            size: size.parse::<u64>().unwrap(),
                                            name: name.to_string(),
                                        })
                                    )
                                }
                            });
                        }
                        _ => panic!("Illegal command"),
                    }
                }
                None => panic!("Cannot receive output without issuing a command"),
            }
        }
    }
    let res = dir.borrow().get_child("/").borrow_mut().get_dir().clone();
    let r = res.unwrap().borrow().get_total_size();
    r
}

pub fn day_7b() -> u64 {
    let contents = fs::read_to_string("src/data/day7.txt").expect("Should be able to open file").trim().to_string();
    let mut dir = Rc::new(
        RefCell::new(Dir { this: None, parent: None, size: 0, name: "\\".to_string(), has: HashMap::new() })
    );
    dir.borrow_mut().this = Some(dir.clone());
    let mut cur_cmd = None;
    for line in contents.split('\n') {
        let cmd: Vec<_> = line.split(' ').collect();
        if cmd[0] == "$" {
            match cmd[1] {
                "cd" => {
                    let child = dir.borrow().get_child(cmd[2]);
                    match child.borrow_mut().get_dir() {
                        Some(new_dir) => {
                            dir = new_dir.clone();
                        }
                        None => panic!("cd should not be used on files"),
                    };
                }
                "ls" => {
                    cur_cmd = Some("ls");
                }
                _ => panic!("Illegal Command"),
            }
        } else {
            match cur_cmd {
                Some(cmd) => {
                    match cmd {
                        "ls" => {
                            let file_data: Vec<_> = line.split(" ").collect();
                            let (size, name) = (file_data[0], file_data[1]);
                            dir.borrow_mut().insert({
                                if size == "dir" {
                                    let res = Rc::new(
                                        RefCell::new(Dir {
                                            this: None,
                                            parent: Some(dir.clone()),
                                            size: 0,
                                            name: name.to_string(),
                                            has: HashMap::new(),
                                        })
                                    );
                                    res.borrow_mut().this = Some(res.clone());
                                    res
                                } else {
                                    Rc::new(
                                        RefCell::new(File {
                                            size: size.parse::<u64>().unwrap(),
                                            name: name.to_string(),
                                        })
                                    )
                                }
                            });
                        }
                        _ => panic!("Illegal command"),
                    }
                }
                None => panic!("Cannot receive output without issuing a command"),
            }
        }
    }
    let res = dir.borrow().get_child("/").borrow_mut().get_dir().clone().unwrap();
    println!("{}", res.borrow().get_size() - 40000000);
    println!("{}", res.borrow().get_size());
    let r = res.borrow().get_min_size_can_remove(u64::MAX, res.borrow().get_size() - 40000000);
    r

}