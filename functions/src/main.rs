pub fn print_name(name: String) {
    println!("name is : {name}");
}

fn main() {
    // Statement
    let name1 = String::from("yello there");
    print_name(name1);
    let x = {
        let y = 5;
        // expression
        y + 1
    };

    // calling a function is an expression
    println!("x: {x}");
    // illegal use since name1 is moved in the prev function
    // String doesn't have Copy, Clone traits
    // let name2 = name1;
}
