use super::AbnfParser;
use abnf::types::{TerminalValues, Rule, Node};
use std::iter::FromIterator;
use nom::character::complete::one_of;

/// Terminal Value matcher. Used for Terminal Value ranges (ex: U+0041 ('A') to U+005A ('Z')) or
/// Terminal Value vectors.
///
/// Creates a parser that takes a &str as input and returns a ParserOutput if the &str matches
/// the a character within the terminal value provided, otherwise the parser returns an error.
pub(crate) fn terminal_value_parser<'a>(rule: &'a Rule) -> anyhow::Result<AbnfParser<'a>> {
    if let Node::TerminalValues(terminal_value) = rule.node() {
        terminal_value_match_parser_helper(rule, terminal_value)
    } else {
        Err(anyhow::Error::msg(format!("rule {} not supported by terminal value parser!", rule)))
    }
}

fn terminal_value_match_parser_helper<'a>(rule: &'a Rule, terminal_values: &'a TerminalValues) -> anyhow::Result<AbnfParser<'a>> {
    // let characters: String = match terminal_values {
    //     TerminalValues::Concatenation(characters) => characters.iter().map(|char_value| char::from_u32(*char_value)).map(Option::unwrap).collect(),
    //     TerminalValues::Range(start, end) => (*start..=*end).map(char::from_u32).map(Option::unwrap).collect(), // TODO: find better way to unwrap Option<char>;
    // };

    // let nom_parser = nom::combinator::map(one_of(move characters.as_str()), move |char_value| char_value.to_string().as_str());
    let characters: Vec<u32> = match terminal_values {
        TerminalValues::Concatenation(characters) => characters.clone(),
        TerminalValues::Range(start, end) => (*start..=*end).collect()
    };

    let nom_parser = nom::combinator::map(one_of(characters), |char_value| char_value.to_string().as_str());
    Ok(AbnfParser::<'a>::from_rule_and_str_parser(rule, nom_parser))
}

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

