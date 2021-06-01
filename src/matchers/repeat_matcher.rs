use crate::{Parser};

pub fn repeat_matcher<'a, P, Output>(sub_parser: P) -> impl Parser<'a, Vec<Output>>
    where P: Parser<'a, Output>
{
    move |input| {
        let mut output = Vec::new();
        let mut remaining = input;
        while let Ok((parsed_remaining, parser_result)) = sub_parser.parse(remaining) {
            output.push(parser_result);
            remaining = parsed_remaining;
        }
        return Ok((remaining, output));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matchers::literal_matcher::match_literal;
    use rstest::rstest;

    #[rstest(input, expected,
    case("Hello World! ", Ok(("", vec![()]))),
    case("Hello World! I think I'll go outside", Ok(("I think I'll go outside", vec![()]))),
    case("It's a beautiful day! Hello World! ", Ok(("It's a beautiful day! Hello World! ", vec![]))),
    case("Hello World! Hello World! I think I'll go outside!", Ok(("I think I'll go outside!", vec![(), ()]))),
    case("Hello World! Hello World! Hello World! Hello World! Hello World! ", Ok(("", vec![(), (), (), (), ()]))),
    case("Hello World! Hello World! Hello World! Hello World! Hello World! It's a beautiful day!", Ok(("It's a beautiful day!", vec![(), (), (), (), ()]))),
    case(" It's a beautiful day!", Ok((" It's a beautiful day!", vec![])))
    )]
    fn repeat_matcher_tests(input: &str, expected: Result<(&str, Vec<()>), &str>) {
        let hello_world_matcher = match_literal("Hello World! ");
        let repeat_matcher = repeat_matcher(hello_world_matcher);

        assert_eq!(expected, repeat_matcher.parse(input));
    }
}
