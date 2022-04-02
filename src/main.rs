use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::io::{self, prelude::*};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Symbol {
    stack_record_count: u64,
    stack_count: u64,
    name: String,
}

fn main() -> io::Result<()> {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} module-filter-string", args[0]);
        return Ok(());
    }

    let filter_str = &args[1];
    let stdin = io::stdin();
    let stdin = stdin.lock();
    let mut symbols = HashMap::new();
//    let regex = Regex::new(r"\s{4,}([a-fA-F0-9]{8,16}) (.*?) \(([^\)]*?\))$").unwrap();
    let stack_record_regex = Regex::new(r"\s{4,}([a-fA-F0-9]{6,16}) (.*?) \(([^\)]*?\))").unwrap();
    let mut is_top_of_stack = false;
    let mut stack_count = 0;
    let mut stack_record_count = 0;
    let mut line_count = 0;

    for line in stdin.lines() {
        line_count += 1;
        if let Some(grps) = stack_record_regex.captures(&line?) {
            // println!("======== 2");
            let (addr, sym, module) = (&grps[1], &grps[2], &grps[3]);

            if sym.starts_with("??") || !module.contains(filter_str) {
                continue;
            }
            //  println!("======== {},{},{}", filter_str, sym, module);

            let record = symbols
                .entry(u64::from_str_radix(addr, 16).unwrap())
                .or_insert_with(|| Symbol {
                    name: sym.to_owned(),
                    stack_record_count: 0,
                    stack_count: 0,
                });

            stack_record_count += 1;
            record.stack_record_count += 1;

            if is_top_of_stack {
                record.stack_count += 1;
                is_top_of_stack = false;
                stack_count += 1;
            }
        } else {
            is_top_of_stack = true;
        }
    }

    for (addr, sym) in &symbols {
        println!("0x{:X},{},{},{}", addr, sym.stack_record_count, sym.stack_count, sym.name);
    }

    eprintln!("stack_count={}, stack_record_count={} line_count={}", stack_count, stack_record_count, line_count);
    if symbols.is_empty() {
        eprintln!("warn: no records processed");
    } else {
        eprintln!("{} symbols processed", symbols.keys().len());
    }

    Ok(())
}
