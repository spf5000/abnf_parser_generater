use crate::ParseResult;

pub fn any_char_matcher(input: &str) -> ParseResult<char> {
    match input.chars().next() {
        Some(char) => Ok((&input[char.len_utf8()..], char)),
        _ => Err(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn any_char_matcher_matches_test() {
        assert_eq!(
          Ok(("ello World!", 'H')), any_char_matcher("Hello World!")
        );
    }
}
