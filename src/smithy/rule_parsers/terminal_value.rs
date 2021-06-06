// use super::{SmithyParser, ParserOutput, nom_output_mapper};
// use abnf::types::{TerminalValues, Rule, Node};
// use nom::character::complete::char;
//
// /// Terminal Value matcher. Used for Terminal Value ranges (ex: U+0041 ('A') to U+005A ('Z')) or
// /// Terminal Value vectors.
// ///
// /// Creates a parser that takes a &str as input and returns a ParserOutput if the &str matches
// /// the a character within the terminal value provided, otherwise the parser returns an error.
// pub(crate) fn terminal_value_match_parser<'a>(rule: &'a Rule) -> anyhow::Result<SmithyParser<'a>> {
//     if let Node::TerminalValues(terminal_value) = rule.node() {
//         Ok(|input| {
//             terminal_value_match_parser_helper(rule, terminal_value)(input)
//         })
//     } else {
//         anyhow::Error::msg(format!("rule {} not supported by terminal value parser!", rule))
//     }
// }
//
// fn terminal_value_match_parser_helper<'a>(rule: &'a Rule, terminal_values: &'a TerminalValues) -> anyhow::Result<SmithyParser<'a>> {
//     let characters: &Vec<u32>= match terminal_values {
//         TerminalValues::Concatenation(characters) => characters,
//         TerminalValues::Range(start, end) => (start..=end).collect()
//     };
//     let mut nom_input = String::from("");
//     for character in characters {
//         let c = char::from_u32(*character).ok_or(anyhow::Error::msg(format!("Terminal Value {:?} included non-valid u32 values", rule)))?;
//         nom_input.push(c);
//     }
//
//     Ok(|input| {
//         nom::character::complete::one_of(nom_input.as_ref())
//     })
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::smithy::rule_parsers::SmithyParseResult;
//
//     #[rstest(input, start, end, expected,
//     case("Hello World!" 'A', 'Z', Ok(("ello World!", 'H'))),
//     case("Hello World!" '1', '9', Err("TODO")),
//     case("Hello World!" 'a', 'z', Err("TODO")),
//     case("12345" '1', '9', Ok(("2345", '1'))),
//     case("12345" 'A', 'Z', Err("TODO")),
//     case("12345" '2', '9', Err("TODO")),
//     )]
//     fn terminal_value_range_matches_tests(input: &str, start: char, end: char, expected: SmithyParseResult<ParserOutput>) {
//         let terminal_value = TerminalValues::Range(u32::from(start), u32::from(end));
//         let parser = terminal_value_match_parser(&terminal_value);
//         assert_eq!(expected, parser(input));
//     }
//
//     #[rstest(input, chars, expected,
//     case("abc" vec!['a', 'e', 'i', 'o', 'u'], Ok(("bc", 'a'))),
//     case("efg" vec!['a', 'e', 'i', 'o', 'u'], Ok(("fg", 'e'))),
//     case("ijk" vec!['a', 'e', 'i', 'o', 'u'], Ok(("jk", 'i'))),
//     case("opq" vec!['a', 'e', 'i', 'o', 'u'], Ok(("pq", 'o'))),
//     case("uvw" vec!['a', 'e', 'i', 'o', 'u'], Ok(("vw", 'u'))),
//     case("xyz" vec!['a', 'e', 'i', 'o', 'u'], Err("TODO")),
//     )]
//     fn terminal_value_concat_matches_tests(input: &str, chars: Vec<char>, expected: SmithyParseResult<ParserOutput>) {
//         let terminal_value = TerminalValues::Concatenation(chars.into().map(u32::from).collect());
//         let parser = terminal_value_match_parser(&terminal_value);
//         assert_eq!(expected, parser(input));
//     }
// }
//
