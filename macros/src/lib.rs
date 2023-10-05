#[macro_export]
macro_rules! vecb {
    ($($x:expr), *) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

// #[macro_export]
// macro_rules! vecn {
//     () => {
//         std::vec::Vec::new()
//     };
//     ($elem:expr; $n:expr) => {
//         std::vec::from_elem($elem, $n)
//     };
//     ($($x:expr), *) => {
//         let s =Box::new([$($x), *]);
//         s.to_vec()
//     };
//     ($($x:expr,)*) => (vec![$($x), *])
// }

pub trait HelloMacro {
    fn hello_macro();
}
