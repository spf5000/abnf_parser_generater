use std::collections::HashMap;
use std::sync::Arc;
use abnf::types::{Rule, Node};

use crate::CreateParserOutput;
use crate::model::{AstParserResult, ParserInput};


pub(crate)fn rule_name_parser_generator(rule: Rule, parsers: &HashMap<String, Arc<dyn Fn(ParserInput) -> AstParserResult>>) -> CreateParserOutput {
    if let Node::Rulename(rule_name) = rule.node().clone() {
        if let Some(subparser) = parsers.get(&rule_name) {
            CreateParserOutput::Success(subparser.clone())
        } else {
            CreateParserOutput::Pending(rule)
        }
    } else {
        CreateParserOutput::Failed(anyhow::anyhow!("Provided non RuleName to the RuleName (sub) parser generator! {}", rule))
    }
}


#[cfg(test)]
mod test {
    use rstest::rstest;
    use crate::AstNode;

    use crate::parsers::string_literal::tests::{
        RULE_NAME, 
        create_test_string_literal_parser, 
        assert_string_literal_expected_output,
    };

    use super::*;

    #[rstest]
    #[case("Hello", true, "Hello World", Ok((ParserInput::from(" World"), AstNode::new_literal(RULE_NAME, "Hello"))))]
    #[case("Hello", false, "hELLo World", Ok((ParserInput::from(" World"), AstNode::new_literal(RULE_NAME, "hELLo"))))]
    #[case("Hello", true, "hELLo World", Err(anyhow::Error::msg("Does not matter. Just checking for errors!")))]
    #[case("Hello", true, "Not Hello World", Err(anyhow::Error::msg("Does not matter. Just checking for errors!")))]
    #[case("Hello", false, "Not hELLo World", Err(anyhow::Error::msg("Does not matter. Just checking for errors!")))]
    fn rule_name_parser_test_with_subparser(#[case] literal_string: &str, 
                             #[case] case_sensative: bool, 
                             #[case] input: &str, 
                             #[case] expected_output: AstParserResult) {
        let subparser = create_test_string_literal_parser(literal_string, case_sensative);

        let parser_map = {
            let mut map = HashMap::new();
            map.insert(RULE_NAME.to_string(), subparser);
            map
        };

        let rule = Rule::new("Ignored", Node::rulename(RULE_NAME));
        if let CreateParserOutput::Success(parser) = rule_name_parser_generator(rule, &parser_map) {
            assert_string_literal_expected_output(expected_output, parser(ParserInput::from(input)))
        } else {
            panic!("Failed to get parser from rulename parser!");
        }
    }

    #[test]
    fn rule_name_parser_test_without_subparser() {
        let rule = Rule::new("Ignored", Node::rulename(RULE_NAME));
        let expected_output = rule.clone();
        if let CreateParserOutput::Pending(returned_rule) = rule_name_parser_generator(rule, &HashMap::new()) {
            assert_eq!(expected_output, returned_rule);
        } else {
            panic!("Failed to get parser from rulename parser!");
        }
    }

    #[test]
    fn rule_name_parser_test_with_mismatched_subparser() {
        let rule = Rule::new("Ignored", Node::rulename(RULE_NAME));
        let expected_output = rule.clone();
        let parser_map = {
            let mut map = HashMap::new();
            let subparser: Arc<dyn Fn(ParserInput) -> anyhow::Result<(ParserInput, AstNode)>> = Arc::new(|_| anyhow::bail!("Dummy Subparser!"));
            map.insert("SomeUnusedRule".to_string(), subparser);
            map
        };

        if let CreateParserOutput::Pending(returned_rule) = rule_name_parser_generator(rule, &parser_map) {
            assert_eq!(expected_output, returned_rule);
        } else {
            panic!("Failed to get parser from rulename parser!");
        }
    }
}
