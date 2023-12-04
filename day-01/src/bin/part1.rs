use std::fs::File;
use std::io::prelude::*;

fn counting(contents: String) -> u32 {
    let mut first = 10;
    let mut last = 10;
    let mut result = 0;

    let lines = contents.lines();
    for (i, line) in lines.enumerate(){
        for symb in line.chars(){
            match symb.to_digit(10){
                Some(num) => {
                    last = num;
                    if first == 10{
                        first = num;
                    }
                },
                None => continue,
            };
        }
        result += first * 10 + last;
        first = 10;
        last = 10;
    }
    result
}

fn main() -> std::io::Result<()> {
    let mut input = File::open("sample1.txt")?;
    let mut contents = String::new();    

    input.read_to_string(&mut contents).unwrap();
    println!("{}", counting(contents));
    Ok(())
}
