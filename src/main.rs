use std::error::Error;
use std::fs::File;
use std::io::{BufRead,BufReader};
use std::collections::BTreeSet;
#[derive(Debug)]
struct Seat{
    row: u8,
    column: u8,
    id: usize,
}

fn main() -> Result<(), Box<dyn Error>> {
    let raw = File::open("./input.txt")?;
    let buf = BufReader::new(raw);
    //let mut seats: Vec<Seat> = Vec::new();
    let mut seats = BTreeSet::new();
    let mut max_id: usize = 0;
    for line in buf.lines() {
        let line = line.unwrap();
        let mut itr = line.chars();
        let mut row: u8 = 0;
        let mut column: u8 = 0;
        for n in (0..7).rev() {
            match itr.next().unwrap() {
                'F' => {},
                'B' => row |= 1 << n,
                _ => panic!("Parsing error!"),
            }
        }
        for n in (0..3).rev() {
            match itr.next().unwrap() {
                'L' => {},
                'R' => column |= 1 << n,
                _ => panic!("Parsing error!"),
            }
        }
        let id: usize = (row as usize * 8) + (column as usize);
        max_id = std::cmp::max(id,max_id);
        //seats.push(Seat{row,column,id});
        //println!("{}:{}:{}:{}",line,row,column,id);
        seats.insert(id);
    }

    let mut itr = seats.iter();
    let mut prev=  itr.next().unwrap();
    for n in itr {
        if n - prev > 1 {
            println!("{}",prev+1);
        }
        prev = n;
    }
    //println!("{}",max_id);
    return Ok(());
}
