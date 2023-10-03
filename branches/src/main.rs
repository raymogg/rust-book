fn main() {
    let number = 8;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let conditional_number = if number == 3 {3} else {5};
    println!("Conditional number is {conditional_number}");

    let mut loop_count = 0;
    loop {
        println!("Help, im stuck!");
        loop_count += 1;
        if loop_count > 3 {
            break;
        }
    }

    let mut counter_2 = 0;
    let loopy_result = loop {
        counter_2 += 1;
        if counter_2 == 3 {
            break counter_2 + 5;
        }
    };
    print!("Feeling loopy? The number is {loopy_result}");

    // The inception loop

    let mut dreams = 0;
    'level1: loop {
        println!("You are on dream level {dreams}");
        
        let mut inner_dreams = 0;
        loop {
            println!("Are you awake? {inner_dreams}");
            inner_dreams += 1;
            if dreams == 3 {
                println!("Time to wake up! Dreamer {dreams} {inner_dreams}");
                break 'level1;
            }

            if inner_dreams == 2 {
                break;
            }
        }

        dreams += 1;
    }

    // counting sheep
    let sheeps: [u32; 6] = [1,2,3,5,8,13];
    for sheep in sheeps {
        println!("{sheep} sheep");
    }

    for num in (1..5).rev() {
        println!("Tick...");
        println!("{num}")
    }
    println!("KABOOOM")
}