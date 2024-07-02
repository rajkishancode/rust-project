// fn main() {
//     println!("Hello, world!");
//     let x: i32 = 1000; // signed integer   - numbers that can be positive or negative
//     let y: u32 = 100; // unsigned integer - only postive number
//     let z: f32 = 100.001; // float numbers

//     print!(" x:{} ", x);
//     print!(" y:{} ", y);
//     print!(" z:{} ", z);
// }

// =========Booleans=and=Conditionals==============
// fn main() {
//     let is_male: bool = true;
//     let is_above_18: bool = true;
//     if is_male {
//         print!("You are male.")
//     } else {
//         print!("you are not male")
//     }
//     if is_male && is_above_18 {
//         print!("You are legal male")
//     }
// }
// =========Strings================
// fn main() {
//     let greeting: String = String::from("Hello world");
//     println!("{}", greeting);
// }

// =========loops================
// fn main() {
//     for i in 0..10 {
//         print!("{} ", i);
//     }
// }

// =========mutable string================

fn main() {
    let mut str: String = String::from("Hello");
    update_str(&mut str);
    print!("{}", str)
}

fn update_str(str: &mut String) {
    str.push_str("world");
}
