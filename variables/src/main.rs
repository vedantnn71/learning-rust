fn main() {
    // let mut x = 5;
    // println!("val of x: {x}");
    //
    // x = 6;
    // println!("val of x: {x}");

    let x = 5;
    
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope:  {x}");
    }

    println!("val of x: {x}");

    // const SEVEN_HOURS_IN_SECNODS: u32 = 60 * 60 * 7;
}
