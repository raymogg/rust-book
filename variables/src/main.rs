fn main() {
    let mut x = 5;
    println!("the value of x is {x}");
    x = 6;
    println!("The value of x is: {x}");

    let sum = 5 + 10;

    let tup: (i32, u64, f64) = (0, 1, 0.5);

    let (x, y, z) = tup;

    let y_val = tup.1;

    println!("The value of x is {x}");
    println!("The value of y is {y_val}");

    let sample_array: [u32; 3] = [1, 2, 3];
}
