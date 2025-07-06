use std::fs::File;
use std::io::{BufReader, BufWriter};
use ropey::Rope;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Welcome to Crabby!
    // This is a basic text editor using clap, ropey, ratatui and crossterm
   let mut text = Rope::from_reader(
    BufReader::new(File::open("src/store/dummy.txt")?)
   )?;
   
   println!("{}", text.line(3));

   let start_idx = text.line_to_char(3);
   let end_idx = text.line_to_char(4);

   text.remove(start_idx..end_idx);

   text.insert(start_idx, "Waiting to hold some cool words");

   let start_idx = text.line_to_char(3);
   let end_idx = text.line_to_char(4);
   println!("{}", text.slice(start_idx..end_idx));

   text.write_to(
    BufWriter::new(File::create("src/store/dummychanged.txt")?)
   )?;

    Ok(())
}
