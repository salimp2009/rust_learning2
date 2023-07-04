use std::fs::File;
use std::io::ErrorKind;
pub fn recoverables_err1() {
    let greeting_file_result = std::fs::File::open("./src/hello.txt");
    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("./src/hello.txt") {
                Ok(fc) => fc,
                Err(err) => panic!("Prpblem creating file {:#?}", err),
            },
            other_error => panic!("problem openning the file {:#?}", other_error),
        },
    };
}

fn main() {
    recoverables_err1();
}
