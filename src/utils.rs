#[inline]
pub(crate) fn extract_digits(s: &str) -> (&str, &str) {
    take_while(s, |c| c.is_ascii_digit())
}

pub(crate) fn extract_op(s: &str) -> (&str, &str) {
    match &s[0..1] {
        "+" | "-" | "*" | "/" => {}
        _ => panic!("bad operator"),
    }

    (&s[1..], &s[0..1])
}

pub(crate) fn extract_ident(s: &str) -> (&str, &str) {
    let valid = s
        .chars()
        .peekable()
        .peek()
        .map(|&c| c.is_ascii_alphabetic())
        .unwrap_or(false);

    if valid {
        take_while(s, |c| c.is_ascii_alphanumeric())
    } else {
        (s, "")
    }
}

#[inline]
pub(crate) fn extract_whitespaces(s: &str) -> (&str, &str) {
    take_while(s, |c| c.is_whitespace())
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

pub(crate) fn tag<'a, 'b>(begin: &'a str, s: &'b str) -> &'b str {
    if s.starts_with(begin) {
        &s[begin.len()..]
    } else {
        panic!("expected {}", begin);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_one_digit() {
        assert_eq!(extract_digits("1+2"), ("+2", "1"));
        assert_eq!(extract_digits("100+200"), ("+200", "100"));
    }

    #[test]
    fn test_extract_anything_empty_input() {
        assert_eq!(extract_digits(""), ("", ""));
    }

    #[test]
    fn test_extract_digits_without_remaning() {
        assert_eq!(extract_digits("100"), ("", "100"));
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
    fn test_extract_ident() {
        assert_eq!(extract_ident("val char"), (" char", "val"));
    }

    #[test]
    fn test_extract_ident_start_with_number() {
        assert_eq!(extract_ident("123val char"), ("123val char", ""));
    }

    #[test]
    fn test_tag() {
        assert_eq!(tag("let", "let a = 1"), " a = 1");
    }
}