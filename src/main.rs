use std::io::{BufReader, BufRead};
use std::fs::File;

use anyhow::Context;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    //for arg in std::env::args().skip(1) {
    //    println!("Hello, world! -> {arg}");
    //}

    let path = dirs::home_dir()
        .context("Can't get home directory")?
        .join(".bash_history");

    let history = BufReader::new(File::open(path)?);
    for line in history.lines() {
        println!("{}", line?);
    }

    //for line in std::fs::read_to_string(path)?
    //    .lines()
    //    .rev()
        //.unique()
        
    //    {
    //    println!("{}", line);
    //}

    Ok(())
}
