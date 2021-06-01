use crate::{Parser};

pub fn or_matcher<'a, Parser1, Parser2, Output>(first_parser: Parser1, second_parser: Parser2)
                                                           -> impl Parser<'a, Output>
    where
        Parser1: Parser<'a, Output>,
        Parser2: Parser<'a, Output>,
{
    move |input| {
        first_parser.parse(input).or(second_parser.parse(input))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matchers::literal_matcher::match_literal;
    use rstest::rstest;

    #[rstest(input, expected,
    case("Hello World! It's a beautiful day!", Ok((" It's a beautiful day!", ()))),
    case("It's a beautiful day! I think I'll go outside", Ok((" I think I'll go outside", ()))),
    case("It's a beautiful day! Hello World!", Ok((" Hello World!", ()))),
    case("I think I'll go outside!", Err("I think I'll go outside!")),
    )]
    fn or_matcher_tests(input: &str, expected: Result<(&str, ()), &str>) {
        let hello_world_matcher = match_literal("Hello World!");
        let beautiful_day_matcher = match_literal("It's a beautiful day!");
        let pair_matcher = or_matcher(hello_world_matcher, beautiful_day_matcher);

        assert_eq!(expected, pair_matcher.parse(input));
    }
}
