// Copyright 2021 the Tectonic Project
// Licensed under the MIT License.

//! Format-file parsing utilities.

use nom::{
    error::ErrorKind as NomErrorKind, error::ParseError, number::complete::be_i32, Err as NomErr,
    IResult, Input,
};

pub fn satisfy_be_i32<I, Error: ParseError<I>>(
    expected: i32,
) -> impl Fn(I) -> IResult<I, i32, Error>
where
    I: Input<Item = u8>,
{
    move |i: I| {
        let (new_input, value) = be_i32(i)?;
        if value != expected {
            return Err(NomErr::Error(Error::from_error_kind(
                new_input,
                NomErrorKind::Satisfy,
            )));
        }
        Ok((new_input, value))
    }
}

pub fn ranged_be_i32<I, Error: ParseError<I>>(
    min: i32,
    max: i32,
) -> impl Fn(I) -> IResult<I, i32, Error>
where
    I: Input<Item = u8>,
{
    move |i: I| {
        let (new_input, value) = be_i32(i)?;
        if value < min || value > max {
            return Err(NomErr::Error(Error::from_error_kind(
                new_input,
                NomErrorKind::Satisfy,
            )));
        }
        Ok((new_input, value))
    }
}
