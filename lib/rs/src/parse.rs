use std::mem::MaybeUninit;
use std::str::FromStr;
use calamine::DataType;
use crate::{DataId, Link, Linkable, error::ParseError};

pub(crate) trait Parsable: Sized {
    fn parse(value: &calamine::Data) -> Result<Self, ParseError>;

    fn parse_str(s: &str) -> Result<Self, ParseError>;
}

pub(crate) fn parse_id(value: &calamine::Data) -> Result<DataId, ParseError> {
    let i = match value.as_i64() {
        Some(i) => i,
        None => return Err(ParseError::InvalidFormat {
            type_name: std::any::type_name::<DataId>(),
            expected: "i64",
            actual: value.to_string(),
        }),
    };

    if i < u32::MIN as i64 || i > u32::MAX as i64 {
        return Err(ParseError::Parse(format!("{} is out of Id range", value.to_string())));
    }

    Ok(DataId(i as u32))
}

pub(crate) fn parse_i8(value: &calamine::Data) -> Result<i8, ParseError> {
    let i = match value.as_i64() {
        Some(i) => i,
        None => return Err(ParseError::Parse(format!("{} is not i8", value.to_string()))),
    };

    if i < i8::MIN as i64 || i > i8::MAX as i64 {
        return Err(ParseError::Parse(format!("{} is out of i8 range", value.to_string())));
    }

    Ok(i as i8)
}

pub(crate) fn parse_i16(value: &calamine::Data) -> Result<i16, ParseError> {
    let i = match value.as_i64() {
        Some(i) => i,
        None => return Err(ParseError::Parse(format!("{} is not i16", value.to_string()))),
    };

    if i < i16::MIN as i64 || i > i16::MAX as i64 {
        return Err(ParseError::Parse(format!("{} is out of i16 range", value.to_string())));
    }

    Ok(i as i16)
}

pub(crate) fn parse_i32(value: &calamine::Data) -> Result<i32, ParseError> {
    let i = match value.as_i64() {
        Some(i) => i,
        None => return Err(ParseError::Parse(format!("{} is not i32", value.to_string()))),
    };

    if i < i32::MIN as i64 || i > i32::MAX as i64 {
        return Err(ParseError::Parse(format!("{} is out of i32 range", value.to_string())));
    }

    Ok(i as i32)
}

pub(crate) fn parse_i64(value: &calamine::Data) -> Result<i64, ParseError> {
    let i = match value.as_i64() {
        Some(i) => i,
        None => return Err(ParseError::Parse(format!("{} is not i64", value.to_string()))),
    };

    Ok(i)
}

pub(crate) fn parse_u8(value: &calamine::Data) -> Result<u8, ParseError> {
    let i = match value.as_i64() {
        Some(i) => i,
        None => return Err(ParseError::Parse(format!("{} is not u8", value.to_string()))),
    };

    if i < u8::MIN as i64 || i > u8::MAX as i64 {
        return Err(ParseError::Parse(format!("{} is out of u8 range", value.to_string())));
    }

    Ok(i as u8)
}

pub(crate) fn parse_u16(value: &calamine::Data) -> Result<u16, ParseError> {
    let i = match value.as_i64() {
        Some(i) => i,
        None => return Err(ParseError::Parse(format!("{} is not u16", value.to_string()))),
    };

    if i < u16::MIN as i64 || i > u16::MAX as i64 {
        return Err(ParseError::Parse(format!("{} is out of u16 range", value.to_string())));
    }

    Ok(i as u16)
}

pub(crate) fn parse_u32(value: &calamine::Data) -> Result<u32, ParseError> {
    let i = match value.as_i64() {
        Some(i) => i,
        None => return Err(ParseError::Parse(format!("{} is not u32", value.to_string()))),
    };

    if i < u32::MIN as i64 || i > u32::MAX as i64 {
        return Err(ParseError::Parse(format!("{} is out of u32 range", value.to_string())));
    }

    Ok(i as u32)
}

pub(crate) fn parse_u64(value: &calamine::Data) -> Result<u64, ParseError> {
    let i = match value.as_i64() {
        Some(i) => i,
        None => return Err(ParseError::Parse(format!("{} is not u64", value.to_string()))),
    };

    Ok(i as u64)
}

pub(crate) fn parse_f32(value: &calamine::Data) -> Result<f32, ParseError> {
    let f = match value.get_float() {
        Some(f) => f,
        None => return Err(ParseError::Parse(format!("{} is not f32", value.to_string()))),
    };

    Ok(f as f32)
}

pub fn parse_f64(value: &calamine::Data) -> Result<f64, ParseError> {
    let f = match value.get_float() {
        Some(f) => f,
        None => return Err(ParseError::Parse(format!("{} is not f64", value.to_string()))),
    };

    Ok(f)
}

pub(crate) fn parse_string(value: &calamine::Data) -> Result<String, ParseError> {
    let s = match value.get_string() {
        Some(s) => s,
        None => return Err(ParseError::Parse(format!("{} is not String", value.to_string()))),
    };

    Ok(s.trim().to_owned())
}

pub(crate) fn parse_link<'a, T: 'static + Linkable>(value: &calamine::Data) -> Result<Link<'a, T>, ParseError> {
    let id = parse_id(value)?;
    // let target = T::get(&id).ok_or_else(|| ParseError::MissingLink {
    //     type_name: std::any::type_name::<T>(),
    //     id,
    // })?;
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
        None => return Err(ParseError::Parse(format!("{} is not a tuple string", value.to_string()))),
    };

    if s.len() < 2
        || s.chars().nth(0).unwrap() != '('
        || s.chars().nth_back(0).unwrap() != ')' {
        return Err(ParseError::Parse(format!("{} has an invalid tuple form", value)));
    }

    let v: Vec<&str> = s.split(", ").collect();
    if v.len() != count {
        return Err(ParseError::Parse(format!("{} does not match tuple count {}", value, count)));
    }

    Ok(v)
}

impl Parsable for DataId {
    fn parse(value: &calamine::Data) -> Result<Self, ParseError> { parse_id(value) }

    fn parse_str(s: &str) -> Result<Self, ParseError> {
        let id = u32::from_str(s).map_err(|_| ParseError::Parse(format!("{} is not a DataId", s)))?;
        Ok(DataId(id))
    }
}

impl Parsable for u8 {
    fn parse(value: &calamine::Data) -> Result<Self, ParseError> { parse_u8(value) }

    fn parse_str(s: &str) -> Result<Self, ParseError> {
        Ok(u8::from_str(s).map_err(|_| ParseError::Parse(format!("{} is not u8", s)))?)
    }
}

impl Parsable for u16 {
    fn parse(value: &calamine::Data) -> Result<Self, ParseError> { parse_u16(value) }

    fn parse_str(s: &str) -> Result<Self, ParseError> {
        Ok(u16::from_str(s).map_err(|_| ParseError::Parse(format!("{} is not u16", s)))?)
    }
}

impl Parsable for u32 {
    fn parse(value: &calamine::Data) -> Result<Self, ParseError> { parse_u32(value) }

    fn parse_str(s: &str) -> Result<Self, ParseError> {
        Ok(u32::from_str(s).map_err(|_| ParseError::Parse(format!("{} is not u32", s)))?)
    }
}

impl Parsable for u64 {
    fn parse(value: &calamine::Data) -> Result<Self, ParseError> { parse_u64(value) }

    fn parse_str(s: &str) -> Result<Self, ParseError> {
        Ok(u64::from_str(s).map_err(|_| ParseError::Parse(format!("{} is not u64", s)))?)
    }
}

impl Parsable for i8 {
    fn parse(value: &calamine::Data) -> Result<Self, ParseError> { parse_i8(value) }

    fn parse_str(s: &str) -> Result<Self, ParseError> {
        Ok(i8::from_str(s).map_err(|_| ParseError::Parse(format!("{} is not i8", s)))?)
    }
}

impl Parsable for i16 {
    fn parse(value: &calamine::Data) -> Result<Self, ParseError> { parse_i16(value) }

    fn parse_str(s: &str) -> Result<Self, ParseError> {
        Ok(i16::from_str(s).map_err(|_| ParseError::Parse(format!("{} is not i16", s)))?)
    }
}

impl Parsable for i32 {
    fn parse(value: &calamine::Data) -> Result<Self, ParseError> { parse_i32(value) }

    fn parse_str(s: &str) -> Result<Self, ParseError> {
        Ok(i32::from_str(s).map_err(|_| ParseError::Parse(format!("{} is not i32", s)))?)
    }
}

impl Parsable for i64 {
    fn parse(value: &calamine::Data) -> Result<Self, ParseError> { parse_i64(value) }

    fn parse_str(s: &str) -> Result<Self, ParseError> {
        Ok(i64::from_str(s).map_err(|_| ParseError::Parse(format!("{} is not i64", s)))?)
    }
}

impl<T: 'static + Linkable> Parsable for Link<'static, T> {
    fn parse(value: &calamine::Data) -> Result<Self, ParseError> { parse_link(value) }

    fn parse_str(s: &str) -> Result<Self, ParseError> {
        let id = DataId::parse_str(s)?;
        // let target = T::get(&id).ok_or_else(|| ParseError::MissingLink {
        //     type_name: std::any::type_name::<T>(),
        //     id,
        // })?;
        let target = MaybeUninit::uninit();

        Ok(Link { id, target })
    }
}

