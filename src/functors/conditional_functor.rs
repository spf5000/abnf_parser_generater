use crate::{Parser};

pub fn conditional<'a, P, A, Cond>(parser: P, condition: Cond) -> impl Parser<'a, A>
where
    P: Parser<'a, A>,
    Cond: Fn(&A) -> bool
{
    move |input| {
        let (remaining, output) = parser.parse(input)?;
        if condition(&output) { Ok((remaining, output)) } else { Err(input) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matchers::any_char_matcher::any_char_matcher;
    use crate::matchers::repeat_matcher::repeat_matcher;

    #[test]
    fn conditional_matcher_whitespace_matches_test() {
        let matcher = conditional(any_char_matcher, |c| c.is_whitespace());
        assert_eq!(
            Ok(("Hello World!", ' ')), matcher.parse(" Hello World!")
        );
    }

    #[test]
    fn conditional_matcher_whitespace_does_not_match_test() {
        let matcher = conditional(any_char_matcher, |c| c.is_whitespace());
        assert_eq!(
            Err("Hello World!"), matcher.parse("Hello World!")
        );
    }

    #[test]
    fn conditional_matcher_alphabetical_matches_test() {
        let matcher = conditional(any_char_matcher, |c| c.is_alphabetic());
        assert_eq!(
            Ok(("ello World!", 'H')), matcher.parse("Hello World!")
        );
    }

    #[test]
    fn conditional_matcher_alphabetical_does_not_match_test() {
        let matcher = conditional(any_char_matcher, |c| c.is_alphabetic());
        assert_eq!(
            Err(" Hello World!"), matcher.parse(" Hello World!")
        );
    }

    #[test]
    fn conditional_matcher_with_vector_matches() {
        let alphabetical_matcher = conditional(any_char_matcher, |c| c.is_alphabetic());
        let matcher = conditional(repeat_matcher(alphabetical_matcher), |vector| !vector.is_empty());
        assert_eq!(
            Ok((" World!", vec!['H', 'e', 'l', 'l', 'o'])), matcher.parse("Hello World!")
        );
    }
    #[test]
    fn conditional_matcher_with_vector_does_not_match() {
        let alphabetical_matcher = conditional(any_char_matcher, |c| c.is_alphabetic());
        let matcher = conditional(repeat_matcher(alphabetical_matcher), |vector| !vector.is_empty());
        assert_eq!(
            Err(" Hello World!"), matcher.parse(" Hello World!")
        );
    }
}
