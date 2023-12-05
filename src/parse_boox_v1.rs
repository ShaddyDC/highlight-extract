use nom::{
    bytes::complete::{tag, take_until},
    combinator::{all_consuming, map},
    error::{FromExternalError, VerboseError},
    multi::many0,
    sequence::{delimited, preceded, terminated, Tuple},
    IResult,
};

use crate::{
    model::{BooxFile, Highlight, Metadata, Section},
    parse_boox::parse_timestamp,
};

const MARK_TIME: &str = "Time：";
const MARK_HIGHLIGHT: &str = "【Original Text】";
const MARK_NOTE: &str = "【Annotations】";
const MARK_PAGE: &str = "【Page Number】";
const MARK_END: &str = "-------------------";
const MARK_SEP: &str = " | ";

pub fn is_v1(i: &str) -> bool {
    i.contains(MARK_TIME)
        && i.contains(MARK_HIGHLIGHT)
        && i.contains(MARK_NOTE)
        && i.contains(MARK_PAGE)
        && i.contains(MARK_END)
}

fn parse_header(i: &str) -> IResult<&str, Metadata, VerboseError<&str>> {
    let preamble = terminated(take_until(MARK_SEP), tag(MARK_SEP));
    let title = delimited(tag("<<"), take_until(">>"), tag(">>"));
    let title = delimited(preamble, title, tag("\n"));
    let author = terminated(take_until("\n"), tag("\n"));

    let (i, (title, author)) = (title, author).parse(i)?;

    Ok((
        i,
        Metadata {
            title: title.to_owned(),
            author: author.to_owned(),
        },
    ))
}

fn parse_highlight(i: &str) -> IResult<&str, Highlight, VerboseError<&str>> {
    let mut timestamp = delimited(tag(MARK_TIME), parse_timestamp, take_until(MARK_HIGHLIGHT));
    let mut highlight = preceded(tag(MARK_HIGHLIGHT), take_until(MARK_NOTE));
    let mut note = preceded(tag(MARK_NOTE), take_until(MARK_PAGE));
    let mut page = preceded(tag(MARK_PAGE), take_until(MARK_END));

    let (i, timestamp) = timestamp(i)?;
    let (i, highlight) = highlight(i)?;
    let (i, note) = note(i)?;

    let (post_page, page) = page(i)?;

    let page = page.trim().parse().map_err(|e| {
        nom::Err::Error(nom::error::VerboseError::from_external_error(
            i,
            nom::error::ErrorKind::MapRes,
            e,
        ))
    })?;

    let (i, _) = (tag(MARK_END), tag("\n")).parse(post_page)?;

    let note = note.trim();
    let note = if note.is_empty() {
        None
    } else {
        Some(note.to_owned())
    };

    Ok((
        i,
        Highlight {
            timestamp,
            page,
            highlight: highlight.trim().to_owned(),
            note,
        },
    ))
}

fn parse_sectioned_highlight(i: &str) -> IResult<&str, (String, Highlight), VerboseError<&str>> {
    let chapter_line = terminated(take_until("\n"), tag("\n"));

    (map(chapter_line, &str::to_owned), parse_highlight).parse(i)
}

pub fn parse_boox_v1(i: &str) -> IResult<&str, BooxFile, VerboseError<&str>> {
    let (i, (metadata, sectioned_highlights)) = (
        parse_header,
        all_consuming(many0(parse_sectioned_highlight)),
    )
        .parse(i)?;

    // I don't fully understand the sections logic,
    // so I'm just going to discard the information
    let sections = sectioned_highlights
        .into_iter()
        .map(|(_, h)| Section::HL(h))
        .collect();

    Ok((i, BooxFile { metadata, sections }))
}

#[test]
fn boox_test() {
    use chrono::NaiveDate;

    let data = include_str!("../test/data/v1.txt");

    let res = parse_boox_v1(data);

    assert_eq!(
        res,
        Ok((
            "",
            BooxFile {
                metadata: Metadata {
                    title: "One Up on Wall Street - Peter Lynch & John Rothchild (952)".to_owned(),
                    author: "Peter Lynch; John Rothchild".to_owned()
                },
                sections: vec![
                    Section::HL(Highlight {
                        timestamp: NaiveDate::from_ymd_opt(2022, 3, 7)
                            .unwrap()
                            .and_hms_opt(1, 11, 0)
                            .unwrap(),
                        page: 13,
                        highlight: "tics to a degree neither side could have imagined in the doldrums of the early 1970s, when I first took the helm at Magellan. At that low point, demoralized investors had to remind themselves that bear markets don’t last forever, and those with patience held on to their stocks and mutual funds for the fifteen years it took the Dow and other averages to regain the prices reached in the mid-1960s. Today it’s worth reminding ourselves that bull markets don’t last forever and that patience is required in both directions.On  of this book I say the breakup of ATT".to_owned(),
                        note: Some("some very good annotation".to_owned())
                    }),
                    Section::HL(Highlight {
                        timestamp: NaiveDate::from_ymd_opt(2022, 3, 7)
                            .unwrap()
                            .and_hms_opt(14, 2, 0)
                            .unwrap(),
                        page: 20,
                        highlight: "valued at $10 billion may not be worth a dime. As expectations turn to reality, the winners will be more obvious than they are today. Investors who see this will have time to act on their “edge.”".to_owned(),
                        note: None
                    }),
                    Section::HL(Highlight {
                        timestamp: NaiveDate::from_ymd_opt(2022, 3, 7)
                            .unwrap()
                            .and_hms_opt(14, 2, 0)
                            .unwrap(),
                        page: 20,
                        highlight: "Microsoft went public in 1986 at 15 cents a share. Three years later you could buy a share for under $1, and from there it advanced eightyfold. (The stock has “split” several times along the way, so original shares never actually sold for 15 cents—for further explanation, see the footnote on .) If you took the Missouri “show me” approach and waited to buy Microsoft until it triumphed with Windows 95, you still made seven times your money. You didn’t have to be a programmer to notice Microsoft everywhere you looked. Except in the Apple orchard, all new computers".to_owned(),
                        note: None
                    }),
                    Section::HL(Highlight {
                        timestamp: NaiveDate::from_ymd_opt(2022, 3, 7)
                            .unwrap()
                            .and_hms_opt(1, 20, 0)
                            .unwrap(),
                        page: 22,
                        highlight: "Street Journal and Barron’s, and get a snapshot review of almost any publicly traded company. From there you can access “Zack’s” and get a summary of ratings from all the analysts who follow a particular stock.Again thanks to the Internet, the cost of buying and selling stocks has been drastically reduced for the small investor, the way it was reduced for institutional investors in 1975. On-line trading has pressured traditional brokerage houses to reduce commissions and transaction fees, continuing a trend that began with the birth of the discount broker two decades ago.You may be wondering what’s happened to my investing habits since I left Magellan. Instead of following thousands".to_owned(),
                        note: None
                    }),
                ]
            }
        ))
    )
}

#[test]
fn highlight_test() {
    use chrono::NaiveDate;
    use nom::{
        error::{ErrorKind::Tag, ParseError, VerboseError},
        Err,
    };

    assert_eq!(
        parse_highlight(
            "Time：2022-03-07 01:11\n【Original Text】tics to a degree\n【Annotations】some very good annotation\n【Page Number】13\n-------------------\n"
        ),
        Ok((
            "",
            Highlight {
                timestamp: NaiveDate::from_ymd_opt(2022, 3, 7)
                    .unwrap()
                    .and_hms_opt(1, 11, 0)
                    .unwrap(),
                page: 13,
                highlight: "tics to a degree".to_owned(),
                note: Some("some very good annotation".to_owned())
            }
        ))
    );

    assert_eq!(
        parse_highlight("Reading Notes"),
        Err(Err::Error(VerboseError::from_error_kind(
            "Reading Notes",
            Tag
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
        parse_header("Reading Notes | <<One Up on Wall Street - Peter Lynch & John Rothchild (952)>>\nPeter Lynch; John Rothchild\n"),
        Ok((
            "",
            Metadata {
                title: "One Up on Wall Street - Peter Lynch & John Rothchild (952)".to_owned(),
                author: "Peter Lynch; John Rothchild".to_owned()
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
