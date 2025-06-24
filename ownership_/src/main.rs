// use std::cell::RefCell;
// use std::collections::HashMap;
// use std::rc::Rc;
// #[derive(Debug)]
// enum FileSystemNode {
//     File {
//         name: String,
//         size: u64,
//     },
//     Directory {
//         name: String,
//         children: HashMap<String, Rc<RefCell<FileSystemNode>>>,
//     },
// }
// impl FileSystemNode {
//     fn new_file(name: String, size: u64) -> Self {
//         FileSystemNode::File { name, size }
//     }
//     fn new_directory(name: String) -> Self {
//         FileSystemNode::Directory {
//             name,
//             children: HashMap::new(),
//         }
//     }
//     fn add_child(
//         &mut self,
//         name: String,
//         child: Rc<RefCell<FileSystemNode>>,
//     ) -> Result<(), &'static str> {
//         match self {
//             FileSystemNode::Directory { children, .. } => {
//                 children.insert(name, child);
//                 Ok(())
//             }
//             FileSystemNode::File { .. } => Err("Cannot add child to a file"),
//         }
//     }
// }
// fn main() {
//     let test_file = Rc::new(RefCell::new(FileSystemNode::new_file(
//         "test.txt".to_string(),
//         1024,
//     )));
//     println!("Test file is: {:?}", test_file);
//     let root = Rc::new(RefCell::new(FileSystemNode::new_directory(
//         "root".to_string(),
//     )));
//     println!("The current directory is: {:?}", root);
//     let dir = root
//         .borrow_mut()
//         .add_child("home".to_string(), test_file.clone());
//     match dir {
//         Ok(_) => println!("Added file to directory successfully."),
//         Err(e) => println!("Error adding file to directory: {}", e),
//     }
//     println!("Final structure: {:#?}", root);
//     if let FileSystemNode::File { name, size } = &*test_file.borrow() {
//         println!("The name of the file is :{name} and its size is: {size} bytes");
//     }
// }
trait Device {
    fn power_on(&mut self);
    fn power_off(&mut self);
    fn get_status(&self) -> String;
    fn reset(&mut self) {
        println!("Resetting device...");
        self.power_off();
        self.power_on();
    }
}
fn show_status<T: Device>(device: &T) -> String {
    device.get_status()
}
struct SmartLight {
    is_on: bool,
    brightness: u8,
}
struct Thermostat {
    is_on: bool,
    temperature: f32,
}
struct Speaker {
    is_on: bool,
    volume: u8,
    track: String,
}
impl Device for SmartLight {
    fn power_on(&mut self) {
        self.is_on = true;
        self.brightness = 100;
        println!(
            "SmartLight powered on with brightness set to {}",
            self.brightness
        );
    }
    fn power_off(&mut self) {
        self.is_on = false;
        self.brightness = 0;
        println!("SmartLight powered off");
    }
    fn get_status(&self) -> String {
        match self.is_on {
            true => format!("SmartLight is ON with brightness {}", self.brightness),
            false => "SmartLight is OFF".to_string(),
        }
    }
}
fn main() {
    // let mut light = SmartLight {
    //     is_on: false,
    //     brightness: 0,
    // };
    // light.power_on();
    // println!("Status1:{}", light.get_status());
    // light.reset();
    // println!("Status2:{}", light.get_status());
    // light.power_off();
    // println!("Status3:  {}", light.get_status());
    // let current_light_status = show_status(&light);
    // println!("Current light status: {}", current_light_status);
    let str1 = String::from("abcd");
    let str2 = "xyz";
    let result = longest_str(str1.as_str(), str2);
    println!("The longest string is: {}", result);
}
fn longest_str(str1: &str, str2: &str) -> &str {
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}
