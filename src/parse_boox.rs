use chrono::NaiveDateTime;
use nom::{
    branch::alt,
    bytes::{
        complete::{is_not, take_till, take_until, take_while},
        streaming::tag,
    },
    combinator::{map, opt},
    sequence::{delimited, preceded, terminated, Tuple},
    IResult,
};

use crate::{
    model::{Metadata, Note, Section},
    nom_util::take_until_multiple,
};

const SEP_TEXT: &str = " | ";

pub fn parse_header(i: &str) -> IResult<&str, Metadata> {
    let start = map(take_until(SEP_TEXT), drop);
    let sep = map(tag(SEP_TEXT), drop);
    let title = delimited(tag("<<"), take_until(">>"), tag(">>"));
    let author = is_not("\n");

    let (i, (_, _, title, author, _)) = (start, sep, title, author, opt(tag("\n"))).parse(i)?;

    Ok((
        i,
        Metadata {
            title: title.to_owned(),
            author: author.to_owned(),
        },
    ))
}

pub fn parse_note(i: &str) -> IResult<&str, Note> {
    let is_digit = |c: char| c.is_digit(10);
    let timestamp = take_until(SEP_TEXT);
    let page = delimited(take_till(is_digit), take_while(is_digit), take_until("\n"));

    let end = "-------------------\n";
    let note_tag = "【Note】";
    let options = &[note_tag, end];
    let highlight = take_until_multiple(options);
    let note = preceded(tag(note_tag), take_until(end));

    let (i, (timestamp, page, _, highlight, note, _)) =
        (timestamp, page, tag("\n"), highlight, opt(note), tag(end)).parse(i)?;

    let timestamp = NaiveDateTime::parse_from_str(timestamp, "%Y-%m-%d %H:%M ")
        .map_err(|_| nom::Err::Error(nom::error::Error::new("", nom::error::ErrorKind::Tag)))?;

    Ok((
        i,
        Note {
            timestamp,
            page: page.parse().unwrap(),
            highlight: highlight.trim().to_owned(),
            note: note.map(|s| s.trim().to_owned()),
        },
    ))
}

pub fn parse_note_or_chapter(i: &str) -> IResult<&str, Section> {
    let chapter_line = terminated(take_until("\n"), tag("\n"));

    alt((
        map(parse_note, Section::N),
        map(chapter_line, Section::Chapter),
    ))(i)
}
