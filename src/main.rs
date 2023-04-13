use std::fs;

use crate::parse_boox::{parse_header, parse_note_or_chapter};
use generate_markdown::print_markdown;
use nom::{ multi::many0, sequence::Tuple};

mod generate_markdown;
mod model;
mod nom_util;
mod parse_boox;

fn main() {
    let data = fs::read_to_string("data/data.txt").unwrap();

    let (_, (m, notes)) = (parse_header, many0(parse_note_or_chapter))
        .parse(&data)
        .unwrap();

    print_markdown(m, notes);
}
