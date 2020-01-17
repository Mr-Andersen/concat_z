use std::{
    env::args,
    fs::File,
    io::{
        prelude::*,
        BufReader
    },
};

fn main() {
    let files: Vec<_> = args().skip(1).map(|name| {
        BufReader::new(File::open(name).expect("valid filename"))
    }).collect();
    let mut buffer = String::new();
    let mut last_id: f64 = -std::f64::MAX;
    for mut file in files {
        'f: loop {
            match file.read_line(&mut buffer) {
                Ok(0) => break 'f,
                Ok(_) => {
                    let column_end = buffer
                        .find(' ')
                        .expect("lines should contain several columns");
                    let new_id = buffer[..column_end]
                        .parse()
                        .expect("first column must be f64");
                    if new_id > last_id {
                        println!("{}", &buffer);
                        last_id = new_id;
                    }
                }
                Err(e) => {
                    panic!("concat_z: error while reading file: {:?}", e);
                }
            }
            buffer.clear();
        }
    }
}
