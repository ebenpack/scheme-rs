use std::{
    env, fs,
    io::{self},
};

use scheme_rs::{lisp_val::LispVal, Thingus};

fn main() -> io::Result<()> {
    // TODO: Make this a lil' more sophisticated.
    // Currently it just executes whatever file it was passed
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    let contents = fs::read_to_string(file_name).expect("Should have been able to read the file");

    let signal = Box::new(move |_v: &mut Vec<LispVal>| {});
    let t = Thingus::new(signal);
    let result = t.eval(&contents);
    println!("{}", result);
    Ok(())
}
