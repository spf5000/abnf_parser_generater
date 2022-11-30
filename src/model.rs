pub type AstParserResult = Result<(String, AstNode), anyhow::Error>;
pub type ParserInput = String; // TODO: This should be genric or &str at a minimum

pub struct AstNode {
    rule_name: String,
    value: AstNodeValue
}

impl AstNode {
    pub(crate) fn new<T: AsRef<str>>(name: T, value: AstNodeValue) -> Self {
        Self {
            rule_name: String::from(name.as_ref()),
            value
        }
    }
}

pub enum AstNodeValue {
    Literal(String)
}
