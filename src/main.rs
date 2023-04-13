use std::fs;

use chrono::NaiveDateTime;
use nom::{
    branch::alt,
    bytes::{
        complete::{is_not, take_till, take_until, take_while},
        streaming::tag,
    },
    combinator::{map, opt},
    error::ParseError,
    multi::many0,
    sequence::{delimited, preceded, terminated, Tuple},
    FindSubstring, IResult, InputLength, Parser,
};

#[derive(Debug)]
struct Metadata {
    title: String,
    author: String,
}

const SEP_TEXT: &str = " | ";

fn parse_header(i: &str) -> IResult<&str, Metadata> {
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

#[derive(Debug)]
struct Note {
    timestamp: NaiveDateTime,
    page: u32,
    highlight: String,
    note: Option<String>,
}

fn parse_note(i: &str) -> IResult<&str, Note> {
    let is_digit = |c: char| c.is_digit(10);
    let timestamp = take_until(SEP_TEXT);
    let page = delimited(take_till(is_digit), take_while(is_digit), take_until("\n"));

    let end = "-------------------\n";
    let note_tag = "【Note】";
    // let highlight = take_until(end).or(take_until(note_tag));
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

fn take_until_multiple<I, E>(matches: &[I]) -> impl FnMut(I) -> IResult<I, I, E> + '_
where
    I: Clone + nom::InputTake + InputLength + FindSubstring<I> + HasLen,
    E: ParseError<I>,
{
    |input| {
        matches
            .iter()
            .map(|s| take_until::<I, I, E>(s.clone()).parse(input.clone()))
            .min_by_key(|v| v.as_ref().map(|(_, s)| s.len()).unwrap_or(usize::MAX)) // TODO 0 is wrong
            .expect("array should not be empty")
    }
}

trait HasLen {
    fn len(&self) -> usize;
}

impl HasLen for &str {
    fn len(&self) -> usize {
        str::len(&self)
    }
}

#[derive(Debug)]
enum Section<'a> {
    N(Note),
    Chapter(&'a str),
}

fn parse_note_or_chapter(i: &str) -> IResult<&str, Section> {
    let chapter_line = terminated(take_until("\n"), tag("\n"));

    alt((
        map(parse_note, Section::N),
        map(chapter_line, Section::Chapter),
    ))(i)
}

fn main() {
    let data = fs::read_to_string("data/data.txt").unwrap();

    let (_, (m, notes)) = (parse_header, many0(parse_note_or_chapter))
        .parse(&data)
        .unwrap();

    println!("# {}", &m.title);
    println!("");
    println!("**Author:** {}", &m.author);
    println!("\n---\n");

    println!("## Highlights\n");

    for section in notes {
        match section {
            Section::Chapter(c) => println!("## {c}\n"),
            Section::N(highlight) => {
                println!(
                    "### Highlight (Page {}, {})\n",
                    &highlight.page, &highlight.timestamp
                );
                for line in highlight.highlight.lines() {
                    println!("> {line}");
                }
                println!("");

                if let Some(n) = &highlight.note {
                    println!("{n}\n");
                }
            }
        }
    }
}
