extern crate scheme_rs;

use std::env::current_dir;
use std::fs;

use scheme_rs::environment::Ports;
use scheme_rs::lisp_val::LispVal;

// TODO
#[test]
fn test() -> Result<(), String> {
    let mut path = current_dir().map_err(|_| "Whoopsie")?;
    path.push("tests/test.scm");
    let contents = fs::read_to_string(path).map_err(|_| "Something went wrong reading the file")?;

    for result in scheme_rs::eval(
        &contents,
        Ports::new(Box::new(|_port: &mut Vec<LispVal>| {})),
    )
    .split('\n')
    .filter(|line| line.trim() != "")
    {
        assert_eq!(result, "#t");
    }

    Ok(())
}
