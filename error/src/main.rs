use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn main() {
    let f = File::open("hello.txt");

    match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!(
                        //ファイルを作成しようとしましたが、問題がありました
                        "Tried to create file but there was a problem: {:?}",
                        e
                    )
                }
            }
        }
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        }
    };
    let result = read_username_from_file();
    match result {
        Ok(s) => println!("{}", s),
        Err(e) => println!("{:?}", e),
    }
}
