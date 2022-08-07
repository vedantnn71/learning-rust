fn main() {
    let int: isize = 284;
    let float: f32 = 4.52;

    let remainder = 88 % 3;

    let boolean: bool = false;

    let tup: (i32, f32) = (500, 7.421);

    let (x, y) = tup;
    
    println!("int: {int}");
    println!("float: {float}");
    println!("reminder: {remainder}");
    println!("boolean: {boolean}");

    println!("y := {y}");
    println!("tup := {:?}", tup);
}
