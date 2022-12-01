use abnf::types::{Rule, Node};
use std::sync::Arc;
use crate::model::*;

// pub(crate)fn string_literal_parser_generator(rule_name: &str, string_literal: StringLiteral) -> impl Fn(String) -> AstParserResult + '_ {
pub(crate)fn string_literal_parser_generator(rule: Rule) -> anyhow::Result<Arc<dyn Fn(String) -> AstParserResult>> {
    if let Node::String(string_literal) = rule.node().clone() {
        let parser = move |input: ParserInput| {
            let rule_value = string_literal.value();
            if string_literal.is_case_sensitive() && input.as_str().starts_with(&rule_value) {
                Ok(parse(rule.name(), rule_value, input))
            } else if !string_literal.is_case_sensitive() && input.to_lowercase().starts_with(&rule_value.to_lowercase()) {
                Ok(parse(rule.name(), rule_value, input))
            } else {
                anyhow::bail!("Input {} does not start with {} with case-sensative as {}", &input, &rule_value, string_literal.is_case_sensitive())
            }
        };
        Ok(Arc::new(parser))
    } else {
        Err(anyhow::anyhow!("Provided non StringLiteral to the StringLiteral (sub) parser generator! {}", rule))
    }
}

fn parse(rule_name: &str, rule_value: &str, input: ParserInput) -> (String, AstNode) {
    println!("TEST");
    let remaining = input.as_str()[rule_value.len()..].to_string();
    println!("TEST2: {}", remaining);
    let value = AstNodeValue::Literal(input[..rule_value.len()].to_string());
    println!("TEST3: {:#?}", value);
    (remaining, AstNode::new(rule_name, value))
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use rstest::rstest;

    pub(crate) const RULE_NAME: &str = "RuleNameForTest";

    #[rstest]
    #[case("Hello", true, "Hello World", Ok((ParserInput::from(" World"), AstNode::new_literal(RULE_NAME, "Hello"))))]
    #[case("Hello", false, "hELLo World", Ok((ParserInput::from(" World"), AstNode::new_literal(RULE_NAME, "hELLo"))))]
    #[case("Hello", true, "hELLo World", Err(anyhow::Error::msg("Does not matter. Just checking for errors!")))]
    #[case("Hello", true, "Not Hello World", Err(anyhow::Error::msg("Does not matter. Just checking for errors!")))]
    #[case("Hello", false, "Not hELLo World", Err(anyhow::Error::msg("Does not matter. Just checking for errors!")))]
    fn test_string_literal_parser_tests(#[case] literal_string: &str, 
                                  #[case] case_sensative: bool, 
                                  #[case] input: &str, 
                                  #[case] expected_output: AstParserResult) {

        let parser = create_test_string_literal_parser(literal_string, case_sensative);
        let parser_output = parser(ParserInput::from(input));
        assert_string_literal_expected_output(expected_output, parser(ParserInput::from(input)))
    }

    pub(crate) fn create_test_string_literal_parser(literal_string: &str, case_sensative: bool) -> Arc<dyn Fn(String) -> AstParserResult> {
        let rule = Rule::new(RULE_NAME, Node::string(literal_string.to_string(), case_sensative));
        string_literal_parser_generator(rule).expect("Expected the parser to correctly take the string literal input.")
    }

    pub(crate) fn assert_string_literal_expected_output(expected_output: AstParserResult, actual_output: AstParserResult) {
        if let Ok(expected_value) = expected_output {
            assert!(actual_output.is_ok());
            assert_eq!(actual_output.unwrap(), expected_value);
        } else {
            assert!(actual_output.is_err());
        }

    }
}
