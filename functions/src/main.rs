fn main() {
    println!("Hello, world!");

    another_function(3, 'm');

    let a_num = five();
    println!("A num is {a_num}");
}

fn another_function(x: i32, label: char) {
    println!("The value of x is {x}{label}");
}

fn five() -> u32 {
    // hey im walking here
    5
}
