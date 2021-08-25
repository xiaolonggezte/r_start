fn main() {
    // mutable
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // constants
    const MAX_POINTS: u32 = 1000_000;
    println!("The value of constant: {}", MAX_POINTS);

    // shadow
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    // shadow: change the type of variable
    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces: {}", spaces);

    
}
