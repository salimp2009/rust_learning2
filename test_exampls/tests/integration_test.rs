use test_exampls;

mod common;

#[test]
fn it_adds() {
    let value = test_exampls::add(3, 2);
    assert_eq!(value, 5);
}
