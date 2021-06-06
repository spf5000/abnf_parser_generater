// use std::boxed::Box;
//
// pub(crate) fn build_rule_parser<'a>(rule: &Node, parser_map: &HashMap<&str, SmithyParser<'a, ParserOutput>>) -> SmithyParser<'a, ParserOutput> {
//     match node {
//         Node::Rulename(rule_name) => parser_map.get(rule_name).expect("The rulename dependency was not found!"),
//         Node::String(value) => exact_match_parser(value),
//         Node::Prose(value) => exact_match_parser(value),
//         Node::TerminalValues(terminal_values) => terminal_value_match_parser(terminal_values),
//         _ => panic!("TODO")
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::matchers::literal_matcher::match_literal;
//     use rstest::rstest;
//
//     #[test]
//     fn match_literal_matches_test() {
//         let match_hello_world = match_literal("Hello World!");
//         assert_eq!(
//             Ok(("", ())), match_hello_world("Hello World!")
//         );
//     }
//
//     #[test]
//     fn match_literal_matches_with_extra_test() {
//         let match_hello_world = match_literal("Hello World!");
//         assert_eq!(
//             Ok((" It's a Beautiful Day!", ())), match_hello_world("Hello World! It's a Beautiful Day!")
//         );
//     }
//
//     #[test]
//     fn match_literal_does_not_match() {
//         let match_hello_world = match_literal("Hello World!");
//         assert_eq!(
//             Err("It's a Beautiful Day!"), match_hello_world("It's a Beautiful Day!")
//         );
//     }
//
//     #[rstest(input, expected,
//     case("Hello World! It's a beautiful day!", Ok(("", ((), ())))),
//     case("Hello World! It's a beautiful day! I think I'll go outside", Ok((" I think I'll go outside", ((), ())))),
//     case("It's a beautiful day! Hello World!", Err("It's a beautiful day! Hello World!")),
//     case("Hello World! I think I'll go outside!", Err("Hello World! I think I'll go outside!")),
//     case("Hello World!", Err("Hello World!")),
//     case(" It's a beautiful day!", Err(" It's a beautiful day!"))
//     )]
//     fn and_matcher_tests(input: &str, expected: Result<(&str, ((), ())), &str>) {
//         let hello_world_matcher = match_literal("Hello World!");
//         let beautiful_day_matcher = match_literal(" It's a beautiful day!");
//         let pair_matcher = and_matcher(hello_world_matcher, beautiful_day_matcher);
//
//         assert_eq!(expected, pair_matcher.parse(input));
//     }
// }
