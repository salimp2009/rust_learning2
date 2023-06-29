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
    // v.push(5);
    println!("vec2: {:#?}", third);
}

pub fn vector_iterate() {
    let mut v = vec![1, 2, 3, 4];
    println!("vector_iterate");
    v.iter()
        .map(|elem| elem * 2)
        .for_each(|elem| println!("{}", elem));

    println!("original vector_iterate: {:#?}", v);

    v.iter_mut().for_each(|elem| {
        *elem *= 2;
    });
    println!("modified vector_iterate: {:#?}", v);
}

pub fn flatmap_filter() {
    (0..5)
        .flat_map(|x| x * 100..x * 110)
        .enumerate()
        .filter(|&(i, x)| (i + x) % 3 == 0)
        .for_each(|(i, x)| println!("{i}:{x}"));
}

pub fn vector_with_enum() {
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(20),
        SpreadsheetCell::Text("blue".to_string()),
        SpreadsheetCell::Float(20.55),
    ];
    println!("row: {:#?}", row);
}

fn main() {
    vector_1();
    vector_2();
    vector_iterate();
    flatmap_filter();
    vector_with_enum();
}
