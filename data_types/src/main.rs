fn main() {

    // Scalar Types
    // ...


    // Compound types

    // Tuples:
    // have a fixed length: once declared, they cannot grow or shrink in size.
    let tup: (i32, f32, u8) = (1, 1.0, 3);
    
    println!("tup.0: {}, tup.1: {}, tup.2: {}", tup.0, tup.1, tup.2);

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("x: {}, y: {}, z: {}", x, y, z);

    // Array:
    // 1. every element of an array must have the same type.
    // 2. arrays in Rust have a fixed length

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    println!("months length: {}", months.len());

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("a length: {}", a.len());

    // an array that contains the same value for each element,
    let a = [3; 5];
    println!("a length: {}", a.len());

    //  access elements of an array using indexing
    println!("a[0]: {}, a[1]: {}", a[0], a[1]);

}
