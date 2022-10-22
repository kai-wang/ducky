const WHITESPACE: &[char] = &[' ', '\n'];

#[inline]
pub(crate) fn extract_digits(s: &str) -> Result<(&str, &str), String> {
    take_while1(s, |c| c.is_ascii_digit(), "expected digits".to_string())
}

pub(crate) fn extract_op(s: &str) -> (&str, &str) {
    match &s[0..1] {
        "+" | "-" | "*" | "/" => {}
        _ => panic!("bad operator"),
    }

    (&s[1..], &s[0..1])
}

pub(crate) fn extract_ident(s: &str) -> Result<(&str, &str), String> {
    let valid = s
        .chars()
        .peekable()
        .peek()
        .map(|&c| c.is_ascii_alphabetic())
        .unwrap_or(false);

    if valid {
        Ok(take_while(s, |c| c.is_ascii_alphanumeric()))
    } else {
        Err("expected identifier".to_string())
    }
}

#[inline]
pub(crate) fn extract_whitespaces(s: &str) -> (&str, &str) {
    take_while(s, |c| WHITESPACE.contains(&c))
}

pub(crate) fn extract_whitespaces1(s: &str) -> Result<(&str, &str), String> {
    take_while1(s, |c| WHITESPACE.contains(&c), "expected whitespace".to_string())
}

pub(crate) fn take_while2(accept: impl Fn(char) -> bool, s: &str) -> (&str, &str) {
    let extracted_end = s
        .char_indices()
        .find_map(|(idx, c)| if accept(c) { None } else { Some(idx) })
        .unwrap_or_else(|| s.len());

    let extracted = &s[..extracted_end];
    let remainder = &s[extracted_end..];
    (remainder, extracted)
}

pub(crate) fn take_while<F>(s: &str, f: F) -> (&str, &str)
where F: Fn(char) -> bool
{
    let match_end = s
        .char_indices()
        .find_map(|(idx, c)| if f(c) { None } else { Some(idx) } )
        .unwrap_or_else(|| s.len());

    (&s[match_end..], &s[..match_end])
}

pub(crate) fn take_while1<F>(s: &str, f: F, msg: String) -> Result<(&str, &str), String>
where
F: Fn(char) -> bool 
{
    let (remaining, v) = take_while(s, f);
    if v.is_empty() {
        Err(msg)
    } else {
        Ok((remaining, v))
    }
}
pub(crate) fn tag<'a, 'b>(begin: &'a str, s: &'b str) -> Result<&'b str, String> {
    if s.starts_with(begin) {
        Ok(&s[begin.len()..])
    } else {
        Err(format!("expected {}", begin))
    }
}

pub(crate) fn sequence<T>(
    parser: impl Fn(&str) -> Result<(&str, T), String>,
    separator_parser: impl Fn(&str) -> (&str, &str),
    mut s: &str,
) -> Result<(&str, Vec<T>), String> {
    let mut items = Vec::new();

    while let Ok((new_s, item)) = parser(s) {
        s = new_s;
        items.push(item);

        let (new_s, _) = separator_parser(s);
        s = new_s;
    }

    Ok((s, items))
}

pub(crate) fn sequence1<T>(
    parser: impl Fn(&str) -> Result<(&str, T), String>,
    separator_parser: impl Fn(&str) -> (&str, &str),
    s: &str,
) -> Result<(&str, Vec<T>), String> {
    let (s, sequence) = sequence(parser, separator_parser, s)?;

    if sequence.is_empty() {
        Err("expected a sequence with more than one item".to_string())
    } else {
        Ok((s, sequence))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_one_digit() {
        assert_eq!(extract_digits("1+2"), Ok(("+2", "1")));
        assert_eq!(extract_digits("100-200"), Ok(("-200", "100")));
    }

    #[test]
    fn test_extract_anything_empty_input() {
        assert_eq!(extract_digits(""), Err("expected digits".to_string()));
    }

    #[test]
    fn test_extract_digits_without_remaning() {
        assert_eq!(extract_digits("100"), Ok(("", "100")));
    }

    #[test]
    fn test_extract_invalid_digits() {
        assert_eq!(extract_digits("abcd"), Err("expected digits".to_string()));
    }

    #[test]
    fn test_extract_plus() {
        assert_eq!(extract_op("+2"), ("2", "+"));
    }

    #[test]
    fn test_extract_minus() {
        assert_eq!(extract_op("-10"), ("10", "-"));
    }

    #[test]
    fn test_extract_star() {
        assert_eq!(extract_op("*3"), ("3", "*"));
    }

    #[test]
    fn test_extract_slash() {
        assert_eq!(extract_op("/4"), ("4", "/"));
    }

    #[test]
    fn test_extract_ws() {
        assert_eq!(extract_whitespaces("    12312"), ("12312", "    "));
    }

    #[test]
    fn test_extract_newlines_or_spaces() {
        assert_eq!(extract_whitespaces(" \n \n \nabc"), ("abc", " \n \n \n"));
    }

    #[test]
    fn test_extract_non_spaces() {
        assert_eq!(
            extract_whitespaces1("blah"),
            Err("expected whitespace".to_string()),
        );
    }

    #[test]
    fn test_extract_ident() {
        assert_eq!(extract_ident("val char"), Ok((" char", "val")));
    }

    #[test]
    fn test_extract_ident_start_with_number() {
        assert_eq!(extract_ident("123val char"), Err("expected identifier".to_string()));
    }

    #[test]
    fn test_tag() {
        assert_eq!(tag("let", "let a = 1"), Ok(" a = 1"));
    }
}