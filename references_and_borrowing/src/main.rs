// fn main() {
//     let s1 = String::from("hello");
//
//     let len = calculate_length(&s1);
//
//     println!("{}", s1);
//
//     // let reference = &s1;
//     // let dereference = &*reference;
//
//     // println!("reference: {reference}");
//     // println!("dereference: {:?}", dereference);
//
//     println!("The length of '{}' is {}.", s1, len);
// }
//
// fn calculate_length(s: &String) -> usize {
//     return s.len();
// }

fn main() {
    let mut s = String::from("hello");

    change(&mut s);

    println!("hey {}", &mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}
