use std::mem::MaybeUninit;
use std::str::FromStr;
use calamine::DataType;
use crate::{DataId, Link, Linkable, error::ParseError};

pub(crate) trait Parsable: Sized {
    fn parse(value: &calamine::Data) -> Result<Self, ParseError>;

    fn parse_str(s: &str) -> Result<Self, ParseError>;
}

pub(crate) fn parse_id(value: &calamine::Data) -> Result<DataId, ParseError> {
    let i = to_integer::<DataId>(value)?;
    check_range(i, u32::MIN, u32::MAX)?;

    Ok(DataId(i as u32))
}

pub(crate) fn parse_i8(value: &calamine::Data) -> Result<i8, ParseError> {
    let i = to_integer::<i8>(value)?;
    check_range(i, i8::MIN, i8::MAX)?;

    Ok(i as i8)
}

pub(crate) fn parse_i16(value: &calamine::Data) -> Result<i16, ParseError> {
    let i = to_integer::<i16>(value)?;
    check_range(i, i16::MIN, i16::MAX)?;

    Ok(i as i16)
}

pub(crate) fn parse_i32(value: &calamine::Data) -> Result<i32, ParseError> {
    let i = to_integer::<i32>(value)?;
    check_range(i, i32::MIN, i32::MAX)?;

    Ok(i as i32)
}

pub(crate) fn parse_i64(value: &calamine::Data) -> Result<i64, ParseError> {
    let i = to_integer::<i64>(value)?;

    Ok(i)
}

pub(crate) fn parse_u8(value: &calamine::Data) -> Result<u8, ParseError> {
    let i = to_integer::<u8>(value)?;
    check_range(i, u8::MIN, u8::MAX)?;

    Ok(i as u8)
}

pub(crate) fn parse_u16(value: &calamine::Data) -> Result<u16, ParseError> {
    let i = to_integer::<u16>(value)?;
    check_range(i, u16::MIN, u16::MAX)?;

    Ok(i as u16)
}

pub(crate) fn parse_u32(value: &calamine::Data) -> Result<u32, ParseError> {
    let i = to_integer::<u32>(value)?;
    check_range(i, u32::MIN, u32::MAX)?;

    Ok(i as u32)
}

pub(crate) fn parse_u64(value: &calamine::Data) -> Result<u64, ParseError> {
    let i = to_integer::<u64>(value)?;

    Ok(i as u64)
}

pub(crate) fn parse_f32(value: &calamine::Data) -> Result<f32, ParseError> {
    let f = to_float::<f32>(value)?;

    Ok(f as f32)
}

pub fn parse_f64(value: &calamine::Data) -> Result<f64, ParseError> {
    let f = to_float::<f64>(value)?;

    Ok(f)
}

pub(crate) fn parse_string(value: &calamine::Data) -> Result<String, ParseError> {
    let s = match value.get_string() {
        Some(s) => s,
        None => return Err(ParseError::InvalidFormat {
            type_name: std::any::type_name::<String>(),
            expected: "string",
            actual: value.to_string(),
        }),
    };

    Ok(s.trim().to_owned())
}

pub(crate) fn parse_link<'a, T: 'static + Linkable>(value: &calamine::Data) -> Result<Link<'a, T>, ParseError> {
    let id = parse_id(value)?;
    let target = MaybeUninit::uninit();

    Ok(Link { id, target })
}

pub(crate) fn parse_tuple_2<T1, T2>(value: &calamine::Data) -> Result<(T1, T2), ParseError>
where
    T1: Parsable,
    T2: Parsable,
{
    let tuple = parse_tuple(value, 2)?;

    let first = T1::parse_str(tuple[0])?;
    let second = T2::parse_str(tuple[1])?;

    Ok((first, second))
}

pub(crate) fn parse_tuple_3<T1, T2, T3>(value: &calamine::Data) -> Result<(T1, T2, T3), ParseError>
where
    T1: Parsable,
    T2: Parsable,
    T3: Parsable,
{
    let tuple = parse_tuple(value, 3)?;

    let first = T1::parse_str(tuple[0])?;
    let second = T2::parse_str(tuple[1])?;
    let third = T3::parse_str(tuple[2])?;

    Ok((first, second, third))
}

pub(crate) fn parse_tuple_4<T1, T2, T3, T4>(value: &calamine::Data) -> Result<(T1, T2, T3, T4), ParseError>
where
    T1: Parsable,
    T2: Parsable,
    T3: Parsable,
    T4: Parsable,
{
    let tuple = parse_tuple(value, 4)?;

    let first = T1::parse_str(tuple[0])?;
    let second = T2::parse_str(tuple[1])?;
    let third = T3::parse_str(tuple[2])?;
    let fourth = T4::parse_str(tuple[3])?;

    Ok((first, second, third, fourth))
}

fn parse_tuple(value: &calamine::Data, count: usize) -> Result<Vec<&str>, ParseError> {
    let s = match value.get_string() {
        Some(s) => s.trim(),
        None => return Err(ParseError::InvalidFormat {
            type_name: "tuple",
            expected: "string",
            actual: value.to_string(),
        }),
    };

    if s.len() < 2
        || s.chars().nth(0).unwrap() != '('
        || s.chars().nth_back(0).unwrap() != ')' {
        return Err(ParseError::InvalidFormat {
            type_name: "tuple",
            expected: "(a, b, c, ...)",
            actual: value.to_string(),
        });
    }

    let v: Vec<&str> = s[1..s.len() - 1]
        .split(",")
        .map(|item| item.trim())
        .filter(|item| !item.is_empty())
        .collect();
    if v.len() != count {
        return Err(ParseError::InvalidItemCount {
            type_name: "tuple",
            expected: count,
            actual: v.len(),
        });
    }

    Ok(v)
}

fn to_integer<T>(value: &calamine::Data) -> Result<i64, ParseError> {
    match value.as_i64() {
        Some(i) => Ok(i),
        None => Err(ParseError::InvalidFormat {
            type_name: std::any::type_name::<T>(),
            expected: "integer",
            actual: value.to_string(),
        }),
    }
}

fn string_to_integer<T: FromStr>(s: &str) -> Result<T, ParseError> {
    let i = T::from_str(s).map_err(|_| ParseError::InvalidFormat {
        type_name: std::any::type_name::<T>(),
        expected: "integer",
        actual: s.to_string(),
    })?;

    Ok(i)
}

fn to_float<T>(value: &calamine::Data) -> Result<f64, ParseError> {
    match value.get_float() {
        Some(f) => Ok(f),
        None => Err(ParseError::InvalidFormat {
            type_name: std::any::type_name::<T>(),
            expected: "float",
            actual: value.to_string(),
        }),
    }
}

fn check_range<T: Into<i64>>(value: i64, min: T, max: T) -> Result<(), ParseError> {
    let (min, max) = (min.into(), max.into());
    if min <= value && value <= max {
        return Ok(())
    }

    Err(ParseError::OutOfRange {
        type_name: std::any::type_name::<T>(),
        min,
        max,
        value,
    })
}

impl Parsable for DataId {
    fn parse(value: &calamine::Data) -> Result<Self, ParseError> { parse_id(value) }

    fn parse_str(s: &str) -> Result<Self, ParseError> {
        let id = string_to_integer::<u32>(s)?;
        Ok(DataId(id))
    }
}

impl Parsable for u8 {
    fn parse(value: &calamine::Data) -> Result<Self, ParseError> { parse_u8(value) }

    fn parse_str(s: &str) -> Result<Self, ParseError> {
        Ok(string_to_integer::<u8>(s)?)
    }
}

impl Parsable for u16 {
    fn parse(value: &calamine::Data) -> Result<Self, ParseError> { parse_u16(value) }

    fn parse_str(s: &str) -> Result<Self, ParseError> {
        Ok(string_to_integer::<u16>(s)?)
    }
}

impl Parsable for u32 {
    fn parse(value: &calamine::Data) -> Result<Self, ParseError> { parse_u32(value) }

    fn parse_str(s: &str) -> Result<Self, ParseError> {
        Ok(string_to_integer::<u32>(s)?)
    }
}

impl Parsable for u64 {
    fn parse(value: &calamine::Data) -> Result<Self, ParseError> { parse_u64(value) }

    fn parse_str(s: &str) -> Result<Self, ParseError> {
        Ok(string_to_integer::<u64>(s)?)
    }
}

impl Parsable for i8 {
    fn parse(value: &calamine::Data) -> Result<Self, ParseError> { parse_i8(value) }

    fn parse_str(s: &str) -> Result<Self, ParseError> {
        Ok(string_to_integer::<i8>(s)?)
    }
}

impl Parsable for i16 {
    fn parse(value: &calamine::Data) -> Result<Self, ParseError> { parse_i16(value) }

    fn parse_str(s: &str) -> Result<Self, ParseError> {
        Ok(string_to_integer::<i16>(s)?)
    }
}

impl Parsable for i32 {
    fn parse(value: &calamine::Data) -> Result<Self, ParseError> { parse_i32(value) }

    fn parse_str(s: &str) -> Result<Self, ParseError> {
        Ok(string_to_integer::<i32>(s)?)
    }
}

impl Parsable for i64 {
    fn parse(value: &calamine::Data) -> Result<Self, ParseError> { parse_i64(value) }

    fn parse_str(s: &str) -> Result<Self, ParseError> {
        Ok(string_to_integer::<i64>(s)?)
    }
}

impl<T: 'static + Linkable> Parsable for Link<'static, T> {
    fn parse(value: &calamine::Data) -> Result<Self, ParseError> { parse_link(value) }

    fn parse_str(s: &str) -> Result<Self, ParseError> {
        let id = DataId::parse_str(s)?;
        let target = MaybeUninit::uninit();

        Ok(Link { id, target })
    }
}

