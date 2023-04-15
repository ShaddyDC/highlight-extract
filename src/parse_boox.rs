//! # `parse_boox`
//!
//! `parse_boox` is a collection of functions to parse boox highlight export text files

use chrono::NaiveDateTime;
use nom::{
    bytes::complete::{tag, take_while_m_n},
    combinator::recognize,
    error::{FromExternalError, VerboseError},
    sequence::tuple,
    IResult,
};

use crate::{
    model::BooxFile,
    parse_boox_v1::{is_v1, parse_boox_v1},
    parse_boox_v2::parse_boox_v2,
};

pub const fn is_digit(c: char) -> bool {
    c.is_ascii_digit()
}

pub fn parse_timestamp(i: &str) -> IResult<&str, NaiveDateTime, VerboseError<&str>> {
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

pub fn parse_boox(i: &str) -> IResult<&str, BooxFile, VerboseError<&str>> {
    if is_v1(i) {
        parse_boox_v1(i)
    } else {
        parse_boox_v2(i)
    }
}

#[test]
fn all_data_test() {
    use std::fs;

    let files = fs::read_dir("./test/data/").unwrap();

    for file in files {
        let boox = file
            .map(|e| e.path())
            .and_then(fs::read_to_string)
            .map(|s| parse_boox(&s).map(|(_, b)| b).unwrap());

        assert!(boox.is_ok());
    }
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
