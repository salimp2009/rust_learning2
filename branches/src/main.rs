pub fn return_frm_loop() {
    let mut counter: usize = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter;
        };
    };
    println!("result: {result}");
}

pub fn labels() {
    let mut count: usize = 0;
    'counting_up: loop {
        println!("count ={count}");
        let mut remaining: usize = 10;

        loop {
            println!("remaining: {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }

    println!("End count = {count}");
}

pub fn while_loops() {
    let mut number = 3;
    while number != 0 {
        println!("{number}");
        number -= 1;
    }

    println!("Lift off! : {number}");
}
pub fn for_loop() {
    for i in (1..4).rev() {
        print!("{i}!");
    }
    println!("2nd LiftOff!");
}

fn main() {
    return_frm_loop();
    labels();
    while_loops();
    for_loop();
}
