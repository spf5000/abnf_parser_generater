use abnf_parser_parser::parse_abnf;
use std::fs::read_to_string;

fn main() -> anyhow::Result<()> {
    let cargo_root = env!("CARGO_MANIFEST_DIR");
    let path = format!("{}/configuration/smithy-idl.txt", cargo_root);

    println!("Reading Smithy IDL from {}", path);
    let smithy_abnf = read_to_string(path)?;
    let rules = parse_abnf(smithy_abnf)?;

    println!("Rules:\n{:#?}", rules);

    Ok(())
}
