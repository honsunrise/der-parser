use crate::ber::*;
use crate::error::*;
use nom::bytes::complete::take;
use nom::Err;

/// Read a TAGGED EXPLICIT value (function version)
///
/// The following parses `[2] EXPLICIT INTEGER`:
///
/// ```rust
/// # use der_parser::ber::*;
/// # use der_parser::error::BerResult;
/// use nom::combinator::map_res;
/// #
/// fn parse_int_explicit(i:&[u8]) -> BerResult<u32> {
///     parse_ber_tagged_explicit(
///         2,
///         parse_ber_u32
///     )(i)
/// }
///
/// # let bytes = &[0xa2, 0x05, 0x02, 0x03, 0x01, 0x00, 0x01];
/// let res = parse_int_explicit(bytes);
/// # match res {
/// #     Ok((rem,val)) => {
/// #         assert!(rem.is_empty());
/// #         assert_eq!(val, 0x10001);
/// #     },
/// #     _ => assert!(false)
/// # }
/// ```
pub fn parse_ber_tagged_explicit<'a, T, F>(tag: u32, f: F) -> impl Fn(&'a [u8]) -> BerResult<T>
where
    F: Fn(&'a [u8]) -> BerResult<T>,
{
    move |i: &[u8]| {
        let (i, hdr) = ber_read_element_header(i)?;
        if hdr.tag.0 != tag {
            return Err(Err::Error(BerError::InvalidTag));
        }
        let (i, data) = take(hdr.len as usize)(i)?;
        let (_rest, item) = f(data)?;
        Ok((i, item))
    }
}

/// Read a TAGGED IMPLICIT value (function version)
///
/// The following parses `[2] IMPLICIT INTEGER`:
///
/// ```rust
/// # use der_parser::ber::*;
/// # use der_parser::error::BerResult;
/// use nom::combinator::map_res;
/// #
/// fn parse_int_implicit(i:&[u8]) -> BerResult<u32> {
///     map_res(
///         parse_ber_tagged_implicit(
///             2,
///             parse_ber_content(BerTag::Integer),
///         ),
///         |x: BerObjectContent| x.as_u32()
///     )(i)
/// }
///
/// # let bytes = &[0x82, 0x03, 0x01, 0x00, 0x01];
/// let res = parse_int_implicit(bytes);
/// # match res {
/// #     Ok((rem,val)) => {
/// #         assert!(rem.is_empty());
/// #         assert_eq!(val, 0x10001);
/// #     },
/// #     _ => assert!(false)
/// # }
/// ```
pub fn parse_ber_tagged_implicit<'a, T, F>(tag: u32, f: F) -> impl Fn(&'a [u8]) -> BerResult<T>
where
    F: Fn(&'a [u8], &'_ BerObjectHeader, usize) -> BerResult<'a, T>,
{
    move |i: &[u8]| {
        let (i, hdr) = ber_read_element_header(i)?;
        if hdr.tag.0 != tag {
            return Err(Err::Error(BerError::InvalidTag));
        }
        let (i, data) = take(hdr.len as usize)(i)?;
        let (_rest, item) = f(data, &hdr, MAX_RECURSION)?;
        // XXX DER: check that _rest.is_empty()?
        Ok((i, item))
    }
}
