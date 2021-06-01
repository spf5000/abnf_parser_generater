use crate::{Parser};

pub fn and_matcher<'a, Parser1, Parser2, Output1, Output2>(first_parser: Parser1, second_parser: Parser2)
                                                           -> impl Parser<'a, (Output1, Output2)>
where
  Parser1: Parser<'a, Output1>,
  Parser2: Parser<'a, Output2>,
{
    move |input| {
        let (remaining_input, first_result) = first_parser.parse(input)?;
        let (remaining_input, second_result) = second_parser.parse(remaining_input).or_else(|_| Err(input))?;
        return Ok((remaining_input, (first_result, second_result)));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matchers::literal_matcher::match_literal;
    use rstest::rstest;

    #[rstest(input, expected,
    case("Hello World! It's a beautiful day!", Ok(("", ((), ())))),
    case("Hello World! It's a beautiful day! I think I'll go outside", Ok((" I think I'll go outside", ((), ())))),
    case("It's a beautiful day! Hello World!", Err("It's a beautiful day! Hello World!")),
    case("Hello World! I think I'll go outside!", Err("Hello World! I think I'll go outside!")),
    case("Hello World!", Err("Hello World!")),
    case(" It's a beautiful day!", Err(" It's a beautiful day!"))
    )]
    fn and_matcher_tests(input: &str, expected: Result<(&str, ((), ())), &str>) {
        let hello_world_matcher = match_literal("Hello World!");
        let beautiful_day_matcher = match_literal(" It's a beautiful day!");
        let pair_matcher = and_matcher(hello_world_matcher, beautiful_day_matcher);

        assert_eq!(expected, pair_matcher.parse(input));
    }
}