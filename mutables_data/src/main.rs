#![allow(dead_code)]

use std::{io, println};

#[derive(Debug)]
enum Coolies {
    Database(String),
    Move { x: i32, y: i32 },
}

pub fn tuple_types() {
    let tup = (500, 64.2, 1, Coolies::Database("hello".to_string()));
    let tup2 = (50, "cool", 23.5, Coolies::Move { x: 5, y: 4 });
    println!("{tup:?}, {tup2:?}",);
}

pub fn tuple_destructure() {
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    let (a, _, _) = tup;
    let my_foo = Foo::Quux as u32;
    println!("discriminant of Foo::Quux: {my_foo}");
    println!("tuple_destructure: {x}, {y}, {z}");
    println!("tuple_destructure2: {a}, {}, {}", tup.1, tup.2);
}
// each member is presented by a discriminant/indices starts with O
// unless specified and increments
enum Foo {
    Bar,       // disc 0
    Baz = 123, // disc 123
    Quux,      // dics 124
}

pub fn array_type() {
    let array = [3; 5];
    println!("array: {array:?}");
    let array = [1, 2, 3, 4, 5];

    let mut index = String::new();

    println!("Please enter an index: ");

    io::stdin().read_line(&mut index).unwrap_or_else(|error| {
        println!("error: {error}");
        error.to_string().len()
    });

    let index = index.trim().parse::<usize>().unwrap_or_else(|error| {
        println!("error: {error}");
        error.to_string().len()
    });

    let element = array[index];
    println!("\nelement: {element}");
}

fn main() {
    let mut x = 42;
    let y = 43;
    let var1 = &x;
    println!("{}", *var1);
    x = 100;
    // this is illegal since var1 is dereference after x changes
    // println!("{}", *var1);
    let mut _var2 = &x;
    x = 200;
    _var2 = &y;
    println!("{}", *_var2);
    println!("{x}");

    // constant's are evaluated at compile time
    // literals, raw byte string literals, number literals, path to functions/constants
    // statics, tuple expressions, array expresssions, structs, block expressions
    // field expression, range expressions, const functions...etc are some compile time thngs.
    // see https://doc.rust-lang.org/stable/reference/const_eval.html
    const _THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    let a = &&&&mut 10;
    println!("{}", ****a);

    let _x2 = 2.0;
    let _x3: f32 = 5.0;
    let heart_eyed_cat = '😻';
    let quotient = 56.7 / 32.2;
    println!("{quotient}, {heart_eyed_cat}");
    tuple_types();
    tuple_destructure();
    array_type();
}
