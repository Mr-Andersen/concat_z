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
    let mut args = args().skip(1);
    let mut files: Vec<_>  = Vec::with_capacity(args.len());
    let mut output: Box<dyn FnMut(std::fmt::Arguments) -> std::io::Result<()>>
        = Box::new(|a| std::io::stdout().write_fmt(a));
    while let Some(name) = args.next() {
        if name != "-o" {
            files.push(BufReader::new(File::open(name).expect("valid filename")));
        } else {
            let mut out_file
                = BufWriter::new(
                    File::create(args.next()
                            .expect("'-o' should be followed by output file name"))
                        .expect("valid output file name")
                );
            output = Box::new(move |a| out_file.write_fmt(a));
        }
    }
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
                        output(format_args!("{}", &buffer)).unwrap();
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
