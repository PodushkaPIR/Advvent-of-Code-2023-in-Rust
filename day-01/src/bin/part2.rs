use std::fs::File;
use std::io::prelude::*;

fn countingV2(contents: String) -> u32 {
    let mut index = 0;
    let mut sum = 0;
    let lines = contents.lines(); 
    for (_i, line) in lines.enumerate(){
        let line_iter = std::iter::from_fn(move || {
            let reduce = &line[index..];
            let result = if reduce.starts_with("one"){
                Some('1')
            } else if reduce.starts_with("two"){
                Some('2')
            } else if reduce.starts_with("three"){
                Some('3')
            } else if reduce.starts_with("four"){
                Some('4')
            } else if reduce.starts_with("five"){
                Some('5')
            } else if reduce.starts_with("six"){
                Some('6')
            } else if reduce.starts_with("seven"){
                Some('7')
            } else if reduce.starts_with("eight"){
                Some('8')
            } else if reduce.starts_with("nine"){
                Some('9')
            } else {
                let result = reduce.chars().next();
                result
            };
            index += 1;
            result
        });

        let mut it = line_iter
            .filter_map(|symb| symb.to_digit(10));
        let first = it.next().expect("it should be a number");

        sum += match it.last(){
            Some(num) => format!("{first}{num}"),
            None => format!("{first}{first}"),
        }.parse::<u32>().expect("it should be a valid value");
    }
    sum 
}

fn main() -> std::io::Result<()>{
    let mut input = File::open("input2.txt")?;
    let mut contents = String::new();

    input.read_to_string(&mut contents).unwrap();
    println!("{}", countingV2(contents));
    Ok(())
}
