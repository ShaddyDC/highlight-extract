use std::fs;

use crate::display_markdown::DisplayMarkdown;
use parse_boox::parse_boox;

mod display_markdown;
mod model;
mod nom_util;
mod parse_boox;

fn main() {
    let data = fs::read_to_string("data/data.txt").unwrap();

    let boox = parse_boox(&data).unwrap().1;

    println!("{}", DisplayMarkdown(&boox));
}
