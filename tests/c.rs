use test_each_file::test_each_file;

use sillydrageon::driver;

test_each_file! { in "tests/valid/" => test_parse_valid }
fn test_parse_valid(program: &str) {
    let tokens = driver::tokenize(program).unwrap();
    driver::parser(tokens).unwrap();
}
