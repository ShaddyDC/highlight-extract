//! # parse_boox
//!
//! `parse_boox` is a collection of functions to parse boox highlight export text files
//!
//! use chrono::NaiveDateTime;

use chrono::NaiveDateTime;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_until, take_while, take_while_m_n},
    combinator::{all_consuming, map, opt, recognize},
    error::{FromExternalError, VerboseError},
    multi::many0,
    sequence::{delimited, preceded, terminated, tuple, Tuple},
    IResult, Parser,
};

use crate::{
    model::{BooxFile, Highlight, Metadata, Section},
    nom_util::take_until_multiple,
};

const SEP_TEXT: &str = " | ";

pub fn is_digit(c: char) -> bool {
    c.is_digit(10)
}

pub fn parse_header(i: &str) -> IResult<&str, Metadata, VerboseError<&str>> {
    let start = map(take_until(SEP_TEXT), drop);
    let sep = map(tag(SEP_TEXT), drop);
    let title = delimited(tag("<<"), take_until(">>"), tag(">>"));
    let author = terminated(take_until("\n"), opt(tag("\n")));

    let (i, (_, _, title, author)) = (start, sep, title, author).parse(i)?;

    Ok((
        i,
        Metadata {
            title: title.to_owned(),
            author: author.to_owned(),
        },
    ))
}

fn parse_timestamp(i: &str) -> IResult<&str, NaiveDateTime, VerboseError<&str>> {
    let mut timestamp = recognize(tuple((
        take_while_m_n(4, 4, is_digit),
        tag("-"),
        take_while_m_n(2, 2, is_digit),
        tag("-"),
        take_while_m_n(2, 2, is_digit),
        tag(" "),
        take_while_m_n(2, 2, is_digit),
        tag(":"),
        take_while_m_n(2, 2, is_digit),
    )));

    timestamp(i).and_then(|t| {
        let matched = t.1;
        let timestamp = NaiveDateTime::parse_from_str(matched, "%Y-%m-%d %H:%M").map_err(|e| {
            nom::Err::Error(nom::error::VerboseError::from_external_error(
                i,
                nom::error::ErrorKind::MapRes,
                e,
            ))
        })?;

        Ok((t.0, timestamp))
    })
}

pub fn parse_highlight(i: &str) -> IResult<&str, Highlight, VerboseError<&str>> {
    let page = delimited(take_till(is_digit), take_while(is_digit), take_until("\n"));

    const NOTE_END_MARKER: &str = "-------------------";
    const NOTE_TAG: &str = "【Note】";
    const HIGHLIGHT_END_MARKERS: &[&str; 2] = &[NOTE_TAG, NOTE_END_MARKER];

    let mut highlight = take_until_multiple(HIGHLIGHT_END_MARKERS);
    let note = preceded(tag(NOTE_TAG), take_until(NOTE_END_MARKER));

    let (i, timestamp) = parse_timestamp(i)?;
    let (i, page) = terminated(page, tag("\n")).parse(i).and_then(|(r, m)| {
        let v = m.parse().map_err(|e| {
            nom::Err::Error(nom::error::VerboseError::from_external_error(
                m,
                nom::error::ErrorKind::MapRes,
                e,
            ))
        })?;

        Ok((r, v))
    })?;
    let (i, highlight) = highlight(i).map(|(r, m)| (r, m.trim().to_owned()))?;
    let (i, note) = opt(note)(i).map(|(r, m)| (r, m.map(|s| s.trim().to_owned())))?;
    let (i, _) = (tag(NOTE_END_MARKER), opt(tag("\n"))).parse(i)?;

    Ok((
        i,
        Highlight {
            timestamp,
            page,
            highlight,
            note,
        },
    ))
}

pub fn parse_highlight_or_chapter(i: &str) -> IResult<&str, Section, VerboseError<&str>> {
    let chapter_line = terminated(take_until("\n"), tag("\n"));

    alt((
        map(parse_highlight, Section::HL),
        map(map(chapter_line, &str::to_owned), Section::Chapter),
    ))(i)
}

pub fn parse_boox(i: &str) -> IResult<&str, BooxFile, VerboseError<&str>> {
    let (i, (metadata, sections)) = (
        parse_header,
        all_consuming(many0(parse_highlight_or_chapter)),
    )
        .parse(&i)?;

    Ok((i, BooxFile { metadata, sections }))
}

#[test]
fn boox_test() {
    use chrono::NaiveDate;

    let data = include_str!("../data/data.txt");

    let res = parse_boox(data);

    assert_eq!(
        res,
        Ok((
            "",
            BooxFile {
                metadata: Metadata {
                    title: "Building a Second Brain -- A Proven Method".to_owned(),
                    author: "Tiago Forte".to_owned()
                },
                sections: vec![
                    Section::HL(Highlight {
                        timestamp: NaiveDate::from_ymd_opt(2023, 4, 3)
                            .unwrap()
                            .and_hms_opt(0, 41, 0)
                            .unwrap(),
                        page: 6,
                        highlight: "PKM—or personal knowledge management".to_owned(),
                        note: None
                    }),
                    Section::Chapter("Chapter 3: How a Second Brain Works".to_string()),
                    Section::HL(Highlight {
                        timestamp: NaiveDate::from_ymd_opt(2023, 4, 3)
                            .unwrap()
                            .and_hms_opt(1, 21, 0)
                            .unwrap(),
                        page: 32,
                        highlight: "We bookmark articles to read later, but rarely find the time to revisit them again".to_owned(),
                        note: Some("There's too many to \nactually read them all".to_owned())
                    }),
                    Section::HL(Highlight {
                        timestamp: NaiveDate::from_ymd_opt(2023, 4, 3)
                            .unwrap()
                            .and_hms_opt(16, 57, 0)
                            .unwrap(),
                        page: 39,
                        highlight: "In other words, \nthe jobs that are most likely to stick around are those that involve promoting or defending a particular perspective".to_owned(),
                        note: Some("Not sure about now with LLMs".to_owned())
                    }),
                    Section::HL(Highlight {
                        timestamp: NaiveDate::from_ymd_opt(2023, 4, 3)
                            .unwrap()
                            .and_hms_opt(17, 01, 0)
                            .unwrap(),
                        page: 40,
                        highlight: "Multimedia".to_owned(),
                        note: None
                    }),
                ]
            }
        ))
    )
}

#[test]
fn all_data_test() {
    use std::fs;

    let files = fs::read_dir("./data/").unwrap();

    for file in files {
        let boox = file
            .map(|e| e.path())
            .and_then(fs::read_to_string)
            .map(|s| parse_boox(&s).map(|(_, b)| b).unwrap());

        assert!(boox.is_ok());
    }
}

#[test]
fn section_test() {
    use chrono::NaiveDate;

    assert_eq!(
        parse_highlight_or_chapter(
            "2023-04-03 01:21  |  Page No.: 32\nWe bookmark articles to read later\n【Note】There's too many\n-------------------\n"
        ),
        Ok((
            "",
            Section::HL(Highlight {
                timestamp: NaiveDate::from_ymd_opt(2023, 4, 3)
                    .unwrap()
                    .and_hms_opt(1, 21, 0)
                    .unwrap(),
                page: 32,
                highlight: "We bookmark articles to read later".to_owned(),
                note: Some("There's too many".to_owned())
            })
        ))
    );

    assert_eq!(
        parse_highlight_or_chapter("Chapter 3: How a Second Brain Works\n"),
        Ok((
            "",
            Section::Chapter("Chapter 3: How a Second Brain Works".to_string())
        ))
    );
}

#[test]
fn highlight_test() {
    use chrono::NaiveDate;
    use nom::{
        error::{ErrorKind::TakeWhileMN, ParseError, VerboseError},
        Err,
    };

    assert_eq!(
        parse_highlight(
            "2023-04-03 01:21  |  Page No.: 32\nWe bookmark articles to read later\n【Note】There's too many\n-------------------\n"
        ),
        Ok((
            "",
            Highlight {
                timestamp: NaiveDate::from_ymd_opt(2023, 4, 3)
                    .unwrap()
                    .and_hms_opt(1, 21, 0)
                    .unwrap(),
                page: 32,
                highlight: "We bookmark articles to read later".to_owned(),
                note: Some("There's too many".to_owned())
            }
        ))
    );

    assert_eq!(
        parse_highlight(
            "2023-04-03 01:21  |  Page No.: 32\nWe bookmark articles to read later\n【Note】There's too many\n-------------------"
        ),
        Ok((
            "",
            Highlight {
                timestamp: NaiveDate::from_ymd_opt(2023, 4, 3)
                    .unwrap()
                    .and_hms_opt(1, 21, 0)
                    .unwrap(),
                page: 32,
                highlight: "We bookmark articles to read later".to_owned(),
                note: Some("There's too many".to_owned())
            }
        ))
    );

    assert_eq!(
        parse_highlight("Reading Notes"),
        Err(Err::Error(VerboseError::from_error_kind(
            "Reading Notes",
            TakeWhileMN
        )))
    );

    assert_eq!(
        parse_highlight("Chapter 3: How a Second Brain Works"),
        Err(Err::Error(VerboseError::from_error_kind(
            "Chapter 3: How a Second Brain Works",
            TakeWhileMN
        )))
    );
}

#[test]
fn header_test() {
    use nom::{
        error::{ErrorKind::TakeUntil, ParseError, VerboseError},
        Err,
    };

    assert_eq!(
        parse_header("Reading Notes | <<Building a Second Brain -- A Proven Method>>Tiago Forte\n"),
        Ok((
            "",
            Metadata {
                title: "Building a Second Brain -- A Proven Method".to_owned(),
                author: "Tiago Forte".to_owned()
            }
        ))
    );

    assert_eq!(
        parse_header("Reading Notes"),
        Err(Err::Error(VerboseError::from_error_kind(
            "Reading Notes",
            TakeUntil
        )))
    );
}

#[test]
fn timestamp_test() {
    use chrono::NaiveDate;
    use nom::{
        error::{ErrorKind::TakeWhileMN, ParseError, VerboseError},
        Err,
    };

    assert_eq!(
        parse_timestamp("2023-04-03 00:41"),
        Ok((
            "",
            NaiveDate::from_ymd_opt(2023, 4, 3)
                .unwrap()
                .and_hms_opt(0, 41, 0)
                .unwrap()
        ))
    );

    assert_eq!(
        parse_timestamp("oh no 2023-04-03 00:41"),
        Err(Err::Error(VerboseError::from_error_kind(
            "oh no 2023-04-03 00:41",
            TakeWhileMN
        )))
    );
}
