pub fn vector_1() {
    let v = vec![1, 2, 3, 4];
    let third_element = &v[2];
    let third_element_alt = &v[2];
    assert_eq!(third_element, third_element_alt);
    println!("vector1: {}", third_element);
    println!(
        "3rd elem == 3r elem_alt: {}",
        third_element == third_element_alt
    );
    println!("100th elem of vector1: {:#?}", v.get(100).take());
}

pub fn vector_2() {
    let mut v: Vec<_> = vec![1, 2, 3, 4];
    assert_eq!(v, &[1, 2, 3, 4]);
    let third = v.get(2);
    // illegal mut borrow since it is borrowed as ref on line below
    // v.push(5);
    println!("vec2: {:#?}", third);
}
fn main() {
    vector_1();
    vector_2();
}
