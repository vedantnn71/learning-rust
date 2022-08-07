fn main() {
    let number = 312427;
    const COND: bool = true;

    let my_cond_var = if COND { "hehe" } else { "haha" };

    println!("{my_cond_var}");

    // if number < 5 {
    //     println!("condition is true, man!");
    //     // println!("I'm exiting!"); return;
    // } else {
    //     println!("condition is false, man!");
    // }

    if number % 4 == 0 {
        println!("divisible by 5");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("divi by 2");
    } else {
        println!("not divisible >4!");
    }
}
