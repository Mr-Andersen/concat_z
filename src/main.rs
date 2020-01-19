use std::{
    env::args,
    fs::File,
    io::{
        prelude::*,
        BufReader,
        BufWriter
    },
};

fn main() {
    let mut args = args().skip(1).peekable();
    if args.len() == 0 {
        println!("Usage: concat_z [-o out_file] file1 file2 ...");
        return;
    }
    let mut output_file = match args.peek() {
        Some(a) if a == "-o" => {
            let _ = args.next().unwrap();
            Some(BufWriter::new(
                    File::create(args.next()
                            .expect("'-o' should be followed by output file name"))
                        .expect("valid output file name")
                ))
        },
        _ => None
    };
    let files: Vec<_> = args.map(|name| BufReader::new(File::open(name).expect("valid filename"))).collect();
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
                        match &mut output_file {
                            Some(f) => f.write_fmt(format_args!("{}", &buffer)).unwrap(),
                            None => println!("{}", &buffer)
                        }
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
