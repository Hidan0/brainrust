use std::{
    collections::HashMap,
    env,
    error::Error,
    fs,
    io::{self, stdout, Write},
    process,
};

use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Not enough arguments.");
        process::exit(1);
    } else {
        run_from_file(args[1].clone());
    }
}

fn run_from_file(file: String) {
    let (file_content, brackets_pos) = match preprocess_source(file) {
        Err(e) => {
            eprintln!("Some error occured: {}", e);
            process::exit(1);
        }
        Ok(tup) => tup,
    };

    let mut data: Vec<u8> = vec![0];
    let mut input_buffer: Vec<u8> = vec![];
    let mut dp = 0; // data pointer
    let mut ip = 0; // instruction pointer

    loop {
        if ip < file_content.len() {
            let ch = file_content[ip] as char;

            if ch == '+' {
                if data[dp] == 255 {
                    data[dp] = 0;
                } else {
                    data[dp] += 1;
                }
            } else if ch == '-' {
                if data[dp] == 0 {
                    data[dp] = 255;
                } else {
                    data[dp] -= 1;
                }
            } else if ch == '.' {
                print!("{}", data[dp] as char);
                let _ = stdout().flush();
            } else if ch == ',' {
                if input_buffer.is_empty() {
                    let mut buffer = String::new();
                    io::stdin()
                        .read_line(&mut buffer)
                        .expect("Failed to read input!");
                    input_buffer = buffer.as_bytes().to_owned();
                }

                if let Some(c) = input_buffer.get(0) {
                    data[dp] = *c;
                    input_buffer.remove(0);
                }
            } else if ch == '>' {
                if dp + 1 >= data.len() {
                    data.push(0);
                }
                dp += 1;
            } else if ch == '<' {
                if dp > 0 {
                    dp -= 1;
                }
            } else if ch == '[' {
                if data[dp] == 0 {
                    ip = brackets_pos[&ip];
                }
            } else if ch == ']' {
                if data[dp] != 0 {
                    ip = brackets_pos[&ip];
                }
            }
            ip += 1;
        } else {
            break;
        }
    }
}

fn preprocess_source(path: String) -> Result<(Vec<u8>, HashMap<usize, usize>), Box<dyn Error>> {
    let raw_content = fs::read_to_string(path)?;

    // get rid of useless code
    let actual_code_regex = Regex::new(r"[^<>\+-\.,\[\]]").unwrap();
    let content = actual_code_regex
        .replace_all(raw_content.as_ref(), "")
        .as_bytes()
        .to_owned();

    let mut brackets_pos: HashMap<usize, usize> = HashMap::new();

    let mut pos: Vec<usize> = vec![];
    for (i, ch) in content.iter().enumerate() {
        if *ch == '[' as u8 {
            pos.push(i as usize);
        } else if *ch == ']' as u8 {
            let p = pos.pop().expect("Syntax error");
            brackets_pos.insert(p, i as usize);
            brackets_pos.insert(i, p);
        }
    }

    Ok((content, brackets_pos))
}
