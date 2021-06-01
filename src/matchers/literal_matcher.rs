use crate::ParseResult;

pub fn match_literal(expected: &'static str) -> impl Fn(&str) -> ParseResult<()> {
    move |input| match input.get(0..expected.len()) {
        Some(next) if next == expected => Ok((&input[expected.len()..], ())),
        _ => Err(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn match_literal_matches_test() {
        let match_hello_world = match_literal("Hello World!");
        assert_eq!(
          Ok(("", ())), match_hello_world("Hello World!")
        );
    }

    #[test]
    fn match_literal_matches_with_extra_test() {
        let match_hello_world = match_literal("Hello World!");
        assert_eq!(
            Ok((" It's a Beautiful Day!", ())), match_hello_world("Hello World! It's a Beautiful Day!")
        );
    }

    #[test]
    fn match_literal_does_not_match() {
        let match_hello_world = match_literal("Hello World!");
        assert_eq!(
            Err("It's a Beautiful Day!"), match_hello_world("It's a Beautiful Day!")
        );
    }
}