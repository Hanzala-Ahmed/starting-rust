// fn main() {
//     let a = 1;
//     let b = 2;
//     println!("Hello, world!, {}, {}", b, is_even(b));
// }

// function to check if a number is even
// pub fn is_even(n: i32) -> bool {
//     n % 2 == 0
// }

// arrays
// fn main() {
//     let arr = [1, 2, 3, 4, 5];
//     let slice = &arr[1 .. 4];
//     borrowing_slices(arr, slice)
// }

// function to borrow slices
// fn borrowing_slices(arr: [u8; 5], slice: &[u8]) {
//     println!("{:?}", arr);
//     println!("{:?}", slice);
//     println!("length of slice: {}", slice.len());
//     println!("first element of slice: {}", slice[0]);
// }

// strings
// fn main() {
//     let str: &str = "Hello, worl";
//     let mut string: String = String::from("Hello, world");

//     string.push_str("!");
//     string.push('1');
//     string = string.replace(" ", "Hi");

//     println!("{}", str);
//     println!("{}", string);
// }

// if statement
// fn main() {
//     let n = 5;

//     if n < 0 {
//         println!("{} is negative", n);
//     } else if n > 0 {
//         println!("{} is positive", n);
//     } else {
//         println!("{} is zero", n);
//     }
// }

// loop
// fn main() {
//     for i in 0..6 {
//         println!("{} is {}", i, if i % 2 == 0 {"even"} else {"odd"});
//     }
// }

// while loop
// fn main() {
//     let mut i = 0;

//     while i < 6 {
//         println!("{} is {}", i, if i % 2 == 0 {"even"} else {"odd"});
//         i += 1;

//         if i == 2 {
//             println!("{} is two", i);
//             continue;
//         }

//         if i == 4 {
//             println!("{} is four", i);
//             break;
//         }
//     }
// }

// match statement
// fn main() {
//     let n = 5;
//     match n {
//         0 => println!("{} is zero", n),
//         1 => println!("{} is one", n),
//         2 => println!("{} is two", n),
//         3..=5 => println!("{} is between 3 and 5", n),
//         _ => println!("{} is not between 0 and 5", n),
//     }
// }

// structs (classes)
// fn main() {
//     // ex 1
//     let name = String::from("Bird");
//     let bird = Bird {name, age: 5};
//     bird.bird_details();

//     // ex 2
//     let name2 = String::from("Bird2");
//     let bird2 = Bird {name: name2, age: 6};
//     bird2.bird_details()
// }

// struct Bird {
//     name: String,
//     age: u8,
// }

// impl Bird {
//     fn bird_details(&self) {
//         println!("name: {}", self.name);
//         println!("age: {}", self.age);
//     }
// }

// traits (interfaces)
// fn main() {
//       let name = String::from("Bird");
//     let bird = Bird {name, age: 5};
//     bird.bird_details();

//     // ex 2
//     let name2 = String::from("Bird2");
//     let bird2 = Bird {name: name2, age: 6};
//     bird2.bird_details();

//     println!("can fly: {}", bird.can_fly());
//     println!("can swim: {}", bird.can_swim());
// }

// struct Bird {
//     name: String,
//     age: u8,
// }

// impl Bird {
//     fn bird_details(&self) {
//         println!("name: {}", self.name);
//         println!("age: {}", self.age);
//     }
// }

// trait Animal  {
//     fn can_fly(&self) -> bool;

//     fn can_swim(&self) -> bool {
//         false
//     }
// }

// impl Animal for Bird {
//     fn can_fly(&self) -> bool {
//         true
//     }
// }

// enums
// fn main() {
//     let up: Direction = Direction::Up;
//     let down: Direction = Direction::Down;
//     let left: Direction = Direction::Left;
//     let right: Direction = Direction::Right;

//     println!("up: {:?}", up);
//     println!("down: {:?}", down);
//     println!("left: {:?}", left);
//     println!("right: {:?}", right);
// }

// #[derive(Debug)]
// enum Direction {
//     Up,
//     Down,
//     Left,
//     Right,
// }

// vector (dynamic array - growable)
// fn main() {
//     let mut vec: Vec<i32> = vec![1, 2, 3, 4, 5];

//     vec.len();
//     vec[0];
//     vec.push(6);
//     vec.pop();

//     println!("{:?}", vec);
// }


// hash map (key value pair)
// use std::collections::HashMap;
// fn main() {
//     let mut map = HashMap::new();

//     map.insert("name", "Bird");
//     map.insert("age", "5");

//     println!("{:?}", map);

//     match map.get(&"name") {
//         Some (value) => println!("value: {}", value),
//         _ => println!("Value not found"),
//     }

//     match map.get(&"can_fly") {
//         Some (value) => println!("value: {}", value),
//         None => println!("Value not found"),
//     }

//     map.remove(&"name");
//     println!("{:?}", map)
// }

// HashMap option 
// None -> to indicate that the value is not present
// Some -> to indicate that the value is present

// fn divide(dividend: i32, divisor: i32) -> Option<i32> {
//     if dividend % divisor != 0 {
//         None
//     }
//     else {
//         Some(dividend / divisor)
//     }
// }
// fn main() {
//     let result1 = divide(10, 2);
//     let result2 = divide(10, 3);

//     // unwrapping a Some variant will return the value inside the Some variant
//     println!("{:?}, value: {}", result1, result1.unwrap());

//     // unwrapping a None variant will panic
//     println!("{:?}, value: {}", result2, result2.unwrap());
// }

fn main() {
    
}
