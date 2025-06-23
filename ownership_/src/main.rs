use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
#[derive(Debug)]
enum FileSystemNode {
    File {
        name: String,
        size: u64,
    },
    Directory {
        name: String,
        children: HashMap<String, Rc<RefCell<FileSystemNode>>>,
    },
}
impl FileSystemNode {
    fn new_file(name: String, size: u64) -> Self {
        FileSystemNode::File { name, size }
    }
    fn new_directory(name: String) -> Self {
        FileSystemNode::Directory {
            name,
            children: HashMap::new(),
        }
    }
    fn add_child(
        &mut self,
        name: String,
        child: Rc<RefCell<FileSystemNode>>,
    ) -> Result<(), &'static str> {
        match self {
            FileSystemNode::Directory { children, .. } => {
                children.insert(name, child);
                Ok(())
            }
            FileSystemNode::File { .. } => Err("Cannot add child to a file"),
        }
    }
}
fn main() {
    let test_file = Rc::new(RefCell::new(FileSystemNode::new_file(
        "test.txt".to_string(),
        1024,
    )));
    println!("Test file is: {:?}", test_file);
    let root = Rc::new(RefCell::new(FileSystemNode::new_directory(
        "root".to_string(),
    )));
    println!("The current directory is: {:?}", root);
    let dir = root
        .borrow_mut()
        .add_child("home".to_string(), test_file.clone());
    match dir {
        Ok(_) => println!("Added file to directory successfully."),
        Err(e) => println!("Error adding file to directory: {}", e),
    }
    println!("Final structure: {:#?}", root);
    if let FileSystemNode::File { name, size } = &*test_file.borrow() {
        println!("The name of the file is :{name} and its size is: {size} bytes");
    }
}
